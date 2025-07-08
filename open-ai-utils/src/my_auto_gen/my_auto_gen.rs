use std::{sync::Arc, time::Duration};

use flurl::{FlResponseAsStream, FlUrl};
use rust_extensions::{base64::IntoBase64, date_time::DateTimeAsMicroseconds};
use serde::de::DeserializeOwned;
use tokio::sync::RwLock;

use crate::{
    FunctionToolCallDescription, OpenAiRequestBodyBuilder,
    my_auto_gen::{
        AutoGenSettings, MyAutoGenInner, OpenAiRespModel, RemoteToolFunctions,
        RemoteToolFunctionsHandler, TechRequestLogger, ToolFunction, ToolFunctions,
    },
};

pub struct MyAutoGen {
    inner: RwLock<MyAutoGenInner>,
}

impl MyAutoGen {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(MyAutoGenInner::new()),
        }
    }

    pub async fn register_remote_tool_functions(
        &self,
        remote_tool_function: Arc<dyn RemoteToolFunctions + Send + Sync + 'static>,
    ) {
        let mut inner = self.inner.write().await;
        match &mut *inner {
            MyAutoGenInner::NotInitialized => {
                *inner = MyAutoGenInner::RemoteToolFunctions(
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

    pub async fn register_function<
        ParamType: FunctionToolCallDescription + DeserializeOwned + Send + Sync + 'static,
        TToolFunction: ToolFunction<ParamType> + Send + Sync + 'static,
    >(
        &self,
        func_name: &'static str,
        func_description: &'static str,
        tool_function: Arc<TToolFunction>,
    ) {
        let mut inner = self.inner.write().await;
        let tool_functions = match &mut *inner {
            MyAutoGenInner::NotInitialized => {
                let local_tool_functions = ToolFunctions::new();
                *inner = MyAutoGenInner::LocalToolFunctions(local_tool_functions);
                inner.unwrap_as_local_functions_mut()
            }
            MyAutoGenInner::LocalToolFunctions(data) => data,
            MyAutoGenInner::RemoteToolFunctions(_) => {
                panic!("Remote tool functions is already registered");
            }
        };

        tool_functions.register_function(func_name, func_description, tool_function);
    }

    pub async fn execute(
        &self,
        settings: &AutoGenSettings,
        rb: &mut OpenAiRequestBodyBuilder,
        tech_logs: &mut TechRequestLogger,
        ctx: &str,
    ) -> Result<Vec<ToolCallsResult>, String> {
        {
            let inner = self.inner.read().await;
            inner.populate_request_builder(rb).await;
        }

        let mut tool_calls_result = Vec::new();

        loop {
            let req_ts = DateTimeAsMicroseconds::now();
            let req_txt = serde_json::to_string(rb.get_model()).unwrap();

            let request = execute_request(settings, rb)
                .await
                .map_err(|itm| itm.to_string());

            let (model, response_body) = match request {
                Ok(resp) => resp,
                Err(err) => {
                    tech_logs.add(super::TechRequestLogItem {
                        req_ts: req_ts,
                        request: format_response(req_txt.as_str()),
                        resp_ts: DateTimeAsMicroseconds::now(),
                        response: format_response(err.to_string().as_str()),
                    });

                    return Err(err);
                }
            };

            tech_logs.add(super::TechRequestLogItem {
                req_ts: req_ts,
                request: format_response(req_txt.as_str()),
                resp_ts: DateTimeAsMicroseconds::now(),
                response: format_response(response_body.as_str()),
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
                    return Ok(tool_calls_result);
                }
                super::OpenAiResponse::ToolCall(tool_call_models) => {
                    rb.add_assistant_response_as_tool_calls(tool_call_models);
                    for tool_call_model in tool_call_models {
                        let func_name = tool_call_model.function.name.as_str();

                        let call_result = {
                            let inner = self.inner.read().await;
                            let result = inner
                                .invoke_func(func_name, &tool_call_model.function.arguments, ctx)
                                .await?;
                            result
                        };

                        tool_calls_result.push(ToolCallsResult {
                            fn_name: tool_call_model.function.name.to_string(),
                            call_result: call_result.clone(),
                        });

                        rb.add_tool_call_response(tool_call_model, call_result);
                    }
                }
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

async fn execute_request(
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
        let body = response.receive_body().await.unwrap();
        println!("OpenAI status code: {}", status_code);
        println!("{:?}", std::str::from_utf8(body.as_slice()));
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

#[derive(Debug, Clone)]
pub struct ToolCallsResult {
    pub fn_name: String,
    pub call_result: String,
}

fn format_response(src: &str) -> serde_json::Value {
    let result: Result<serde_json::Value, _> = serde_json::from_str(src);

    match result {
        Ok(result) => result,
        Err(_) => serde_json::Value::String(src.to_string()),
    }
}
