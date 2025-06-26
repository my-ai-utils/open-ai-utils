use std::{sync::Arc, time::Duration};

use flurl::{FlResponseAsStream, FlUrl};
use rust_extensions::{base64::IntoBase64, date_time::DateTimeAsMicroseconds};
use serde::de::DeserializeOwned;

use crate::{
    FunctionToolCallDescription, OpenAiRequestBodyBuilder,
    my_auto_gen::{
        AutoGenSettings, MyAutoGenInner, OpenAiRespModel, RemoteToolFunctions,
        RemoteToolFunctionsHandler, TechRequestLogger, ToolFunction, ToolFunctions,
    },
};

pub struct MyAutoGen {
    inner: MyAutoGenInner,
}

impl MyAutoGen {
    pub fn new() -> Self {
        Self {
            inner: MyAutoGenInner::new(),
        }
    }

    pub fn register_remote_tool_functions(
        &mut self,
        remote_tool_function: Arc<dyn RemoteToolFunctions + Send + Sync + 'static>,
    ) {
        match &mut self.inner {
            MyAutoGenInner::NotInitialized => {
                self.inner = MyAutoGenInner::RemoteToolFunctions(
                    RemoteToolFunctionsHandler {
                        data_src: remote_tool_function,
                    }
                    .into(),
                );
            }
            _ => {
                panic!("Local or remote tool_function is already registered");
            }
        }
    }

    pub fn register_function<
        ParamType: FunctionToolCallDescription + DeserializeOwned + Send + Sync + 'static,
        TToolFunction: ToolFunction<ParamType> + Send + Sync + 'static,
    >(
        &mut self,
        func_name: &'static str,
        func_description: &'static str,
        tool_function: Arc<TToolFunction>,
    ) {
        let tool_functions = match &mut self.inner {
            MyAutoGenInner::NotInitialized => {
                let local_tool_functions = ToolFunctions::new();
                self.inner = MyAutoGenInner::LocalToolFunctions(local_tool_functions);
                self.inner.unwrap_as_local_functions_mut()
            }
            MyAutoGenInner::LocalToolFunctions(data) => data,
            MyAutoGenInner::RemoteToolFunctions(_) => {
                panic!("Remote tool functions is already registered");
            }
        };

        tool_functions.register_function(func_name, func_description, tool_function);
    }

    async fn populate_request_builder(&self, rb: &mut OpenAiRequestBodyBuilder) {
        match &self.inner {
            MyAutoGenInner::NotInitialized => {}
            MyAutoGenInner::LocalToolFunctions(local_tool_functions) => {
                let tools = local_tool_functions.get_tools_description();
                rb.add_tools(tools);
            }
            MyAutoGenInner::RemoteToolFunctions(handler) => {
                let description = handler.data_src.get_tools_description().await;
                let tools = serde_json::to_value(&description);

                if let Err(err) = &tools {
                    println!("Can not parse tools description. Err:{}", err);
                    println!("{}", &description);
                    panic!("{}", err);
                }
                rb.add_tools(tools.unwrap());
            }
        }
    }

    pub async fn execute(
        &self,
        settings: &AutoGenSettings,
        rb: &mut OpenAiRequestBodyBuilder,
        tech_logs: &mut TechRequestLogger,
    ) -> Result<(), String> {
        self.populate_request_builder(rb).await;

        loop {
            let req_ts = DateTimeAsMicroseconds::now();
            let req_txt = serde_json::to_string(rb.get_model()).unwrap();

            let (model, response_body) = self
                .execute_request(settings, rb)
                .await
                .map_err(|itm| itm.to_string())?;

            tech_logs.add(super::TechRequestLogItem {
                req_ts: req_ts,
                request: req_txt,
                resp_ts: DateTimeAsMicroseconds::now(),
                response: response_body.clone(),
            });

            let message_to_analyze = match model.peek_message() {
                Some(message_to_analyze) => message_to_analyze,
                None => {
                    return Err(format!("Invalid response {:?}", response_body));
                }
            };

            match message_to_analyze {
                super::OpenAiResponse::Message(message) => {
                    rb.add_assistant_message(message.to_string());
                    return Ok(());
                }
                super::OpenAiResponse::ToolCall(tool_call_models) => {
                    rb.add_assistant_response_as_tool_calls(tool_call_models);
                    for tool_call_model in tool_call_models {
                        let func_name = tool_call_model.function.name.as_str();
                        let func = self.inner.get_func(func_name);

                        let Some(func) = func else {
                            return Err(format!(
                                "Function call with name {} is not found",
                                func_name
                            ));
                        };

                        let result = func
                            .call(func_name, &tool_call_model.function.arguments)
                            .await;

                        rb.add_tool_call_response(tool_call_model, result);
                    }
                }
            }
        }
    }

    async fn execute_request(
        &self,
        settings: &AutoGenSettings,
        rb: &mut OpenAiRequestBodyBuilder,
    ) -> Result<(OpenAiRespModel, String), String> {
        let mut fl_url = FlUrl::new(settings.url.as_str()).set_timeout(Duration::from_secs(60));

        if let Some(api_key) = settings.api_key.as_ref() {
            fl_url = fl_url.with_header("Authorization", format!("Bearer {}", api_key));
        };

        if settings.do_not_reuse_connection.unwrap_or(false) {
            fl_url = fl_url.do_not_reuse_connection();
        }

        let response = fl_url
            .post_json(rb.get_model())
            .await
            .map_err(|itm| itm.to_string())?;

        let status_code = response.get_status_code();

        if status_code != 200 {
            return Err(format!("Status code: {}", status_code));
        }

        let body = response
            .receive_body()
            .await
            .map_err(|itm| itm.to_string())?;

        let model: Result<OpenAiRespModel, _> = serde_json::from_slice(body.as_slice());

        match model {
            Ok(model) => {
                let body = match std::str::from_utf8(body.as_slice()) {
                    Ok(body_as_str) => body_as_str.to_string(),
                    Err(_) => body.as_slice().into_base64(),
                };

                return Ok((model, body));
            }
            Err(err) => {
                println!("Can not deserialize JsonModel. Err: `{}`", err);
                panic!("Can not deserialize JsonModel. Err: `{}`", err);
            }
        }
    }

    pub async fn execute_request_as_stream(
        &self,
        settings: &AutoGenSettings,
        rb: &mut OpenAiRequestBodyBuilder,
    ) -> Result<FlResponseAsStream, String> {
        let mut fl_url = FlUrl::new(settings.url.as_str()).set_timeout(Duration::from_secs(60));

        if let Some(api_key) = settings.api_key.as_ref() {
            fl_url = fl_url.with_header("Authorization", format!("Bearer {}", api_key));
        };

        if settings.do_not_reuse_connection.unwrap_or(false) {
            fl_url = fl_url.do_not_reuse_connection();
        }

        let response = fl_url
            .post_json(rb.get_model())
            .await
            .map_err(|itm| itm.to_string())?;

        Ok(response.get_body_as_stream())
    }
}
