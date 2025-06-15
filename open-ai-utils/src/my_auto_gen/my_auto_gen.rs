use std::sync::Arc;

use flurl::{FlUrl, FlUrlError};
use serde::de::DeserializeOwned;

use crate::{
    FunctionDescriptionJsonModel, FunctionToolCallDescription, OpenAiRequestBodyBuilder,
    my_auto_gen::{
        AutoGenSettings, MyAutoGenInner, OpenAiRespModel, ToolFunction, ToolFunctionHolder,
    },
};

pub struct MyAutoGen {
    inner: MyAutoGenInner,
}

impl MyAutoGen {
    pub fn register_function<
        ParamType: FunctionToolCallDescription + DeserializeOwned + Send + Sync + 'static,
        TToolFunction: ToolFunction<ParamType> + Send + Sync + 'static,
    >(
        &mut self,
        func_name: &'static str,
        func_description: &'static str,
        tool_function: Arc<TToolFunction>,
    ) {
        let func_json_description = FunctionDescriptionJsonModel {
            name: func_name.to_string(),
            description: func_description.to_string(),
            parameters: ParamType::get_description(),
        };

        let holder = ToolFunctionHolder::new(func_name, tool_function);

        let holder = Arc::new(holder);

        self.inner
            .register(func_name, func_json_description, holder);
    }

    fn populate_request_builder(&self, rb: &mut OpenAiRequestBodyBuilder) {
        for func in self.inner.get_func_descriptions() {
            rb.add_tools_call_description(func.clone());
        }
    }

    pub async fn execute(
        &self,
        settings: &AutoGenSettings,
        rb: &mut OpenAiRequestBodyBuilder,
    ) -> Result<(), String> {
        self.populate_request_builder(rb);

        loop {
            let response = self
                .execute_request(settings, &rb)
                .await
                .map_err(|itm| itm.to_string())?;

            let message_to_analyze = match response.peek_message() {
                Some(message_to_analyze) => message_to_analyze,
                None => {
                    return Err(format!("Invalid response {:?}", response));
                }
            };

            match message_to_analyze {
                super::OpenAiResponse::Message(message) => {
                    rb.add_assistant_message(message.to_string());
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

                        let result = func.call(&tool_call_model.function.arguments).await;

                        rb.add_tool_call_response(tool_call_model, result);
                    }
                }
            }
        }
    }

    async fn execute_request(
        &self,
        settings: &AutoGenSettings,
        rb: &OpenAiRequestBodyBuilder,
    ) -> Result<OpenAiRespModel, FlUrlError> {
        let mut response = FlUrl::new(settings.url.as_str())
            .with_header(
                "Authorization",
                format!("Bearer {}", settings.api_key.as_str()),
            )
            .post_json(rb.get_model())
            .await?;

        let body = response.get_body_as_str().await?;

        println!("OpenAi resp: ```{}```", body);

        let model: Result<OpenAiRespModel, _> = serde_json::from_str(body);

        match model {
            Ok(model) => {
                return Ok(model);
            }
            Err(err) => {
                println!("Can not deserialize JsonModel. Err: `{}`", err);
                panic!("Can not deserialize JsonModel. Err: `{}`", err);
            }
        }
    }
}
