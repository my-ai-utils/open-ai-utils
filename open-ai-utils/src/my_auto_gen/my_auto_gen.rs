use std::sync::Arc;

use rust_extensions::{Logger, date_time::DateTimeAsMicroseconds};
use serde::de::DeserializeOwned;
use tokio::sync::RwLock;

use crate::{
    FunctionToolCallDescription, OpenAiRequestBodyBuilder,
    my_auto_gen::{
        AutoGenSettings, MyAutoGenInner, OpenAiResponseStream, RemoteToolFunctions,
        RemoteToolFunctionsHandler, ToolFunction, ToolFunctions,
    },
};

use super::argentic_response::*;

pub struct MyAutoGen {
    inner: Arc<RwLock<MyAutoGenInner>>,
    logger: Arc<dyn Logger + Send + Sync>,
}

impl MyAutoGen {
    pub fn new(logger: Arc<dyn Logger + Send + Sync>) -> Self {
        Self {
            inner: Arc::new(RwLock::new(MyAutoGenInner::new())),
            logger,
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

        tool_functions
            .register_function(func_name, func_description, tool_function)
            .await;
    }

    pub async fn execute(
        &self,
        settings: &AutoGenSettings,
        rb: &OpenAiRequestBodyBuilder,
        ctx: &str,
    ) -> Result<Vec<super::argentic_response::ToolCallsResult>, String> {
        {
            let inner = self.inner.read().await;
            inner.populate_request_builder(rb).await;
        }

        let mut tool_calls_result: Vec<ToolCallsResult> = Vec::new();

        loop {
            let request =
                super::argentic_response::execute_fl_url_request(settings.unwrap_as_http(), rb)
                    .await
                    .map_err(|itm| itm.to_string());

            let (model, response_body) = match request {
                Ok(resp) => resp,
                Err(err) => {
                    rb.write_tech_log(super::TechRequestLogItem::new_data_as_str(
                        DateTimeAsMicroseconds::now(),
                        super::TechLogItemType::Response,
                        err.as_str(),
                    ))
                    .await;

                    return Err(err);
                }
            };

            rb.write_tech_log(super::TechRequestLogItem::new_data_as_str(
                DateTimeAsMicroseconds::now(),
                super::TechLogItemType::Response,
                response_body.as_str(),
            ))
            .await;

            let message_to_analyze = match model.peek_message() {
                Some(message_to_analyze) => message_to_analyze,
                None => {
                    return Err(format!("Invalid response {:?}", response_body));
                }
            };

            match message_to_analyze {
                super::OpenAiResponse::Message(message) => {
                    rb.add_assistant_message(message.to_string()).await;
                    return Ok(tool_calls_result);
                }
                super::OpenAiResponse::ToolCall(tool_call_models) => {
                    let tool_call_results =
                        super::exec_tool_call(tool_call_models, rb, &self.inner, ctx).await?;

                    tool_calls_result.extend(tool_call_results);
                }
            }
        }
    }

    pub async fn execute_request_as_stream(
        &self,
        settings: &AutoGenSettings,
        rb: Arc<OpenAiRequestBodyBuilder>,
        ctx: &str,
    ) -> Result<OpenAiResponseStream, String> {
        {
            let inner = self.inner.read().await;
            inner.populate_request_builder(&rb).await;
        }

        let (result, sender) = OpenAiResponseStream::new();

        tokio::spawn(super::execute_request_as_stream(
            settings.clone(),
            sender,
            rb,
            self.inner.clone(),
            ctx.to_string(),
            self.logger.clone(),
        ));

        Ok(result)
    }
}
