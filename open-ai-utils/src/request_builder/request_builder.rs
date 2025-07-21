use rust_extensions::StrOrString;
use tokio::sync::Mutex;

use crate::my_auto_gen::{TechRequestLogItem, TechRequestLogger, ToolCallModel};

use super::*;

pub struct OpenAiRequestBodyBuilder {
    pub(crate) inner: Mutex<OpenAiRequestBodyBuilderInner>,
}

impl OpenAiRequestBodyBuilder {
    pub fn new(model: LlmModel) -> Self {
        Self {
            inner: Mutex::new(OpenAiRequestBodyBuilderInner::new(model)),
        }
    }

    pub fn new_with_system_prompt(
        system_prompt: impl Into<StrOrString<'static>>,
        model: LlmModel,
    ) -> Self {
        Self {
            inner: Mutex::new(OpenAiRequestBodyBuilderInner::new_with_system_prompt(
                system_prompt,
                model,
            )),
        }
    }

    pub fn from_history(
        system_prompt: impl Into<StrOrString<'static>>,
        history: Vec<OpenAiMessageModel>,
        model: LlmModel,
    ) -> Self {
        Self {
            inner: Mutex::new(OpenAiRequestBodyBuilderInner::from_history(
                system_prompt,
                history,
                model,
            )),
        }
    }

    pub async fn add_user_message(&self, message: impl Into<StrOrString<'static>>) {
        let mut write_access = self.inner.lock().await;
        write_access.add_user_message(message);
    }

    pub async fn add_tools_call_description(&self, func_description: serde_json::Value) {
        let mut write_access = self.inner.lock().await;
        write_access.add_tools_call_description(func_description);
    }

    pub async fn get_model(&self, other_request_data: &OtherRequestData) -> OpenAiRequestModel {
        let mut write_access = self.inner.lock().await;
        let result = write_access.get_model(other_request_data);

        result
    }

    pub async fn add_assistant_message(&self, message: String) {
        let mut write_access = self.inner.lock().await;
        write_access.add_assistant_message(message);
    }

    pub async fn add_tool_call_response(&self, src: &ToolCallModel, result: String) {
        let mut write_access = self.inner.lock().await;
        write_access.add_tool_call_response(src, result);
    }

    pub async fn add_assistant_response_as_tool_calls(
        &self,
        tool_calls_ai_response: &[ToolCallModel],
    ) {
        let mut write_access = self.inner.lock().await;
        write_access.add_assistant_response_as_tool_calls(tool_calls_ai_response);
    }

    pub async fn add_tools(&self, tools: serde_json::Value) {
        let mut write_access = self.inner.lock().await;
        write_access.add_tools(tools);
    }

    pub async fn modify(&self, rb: impl Fn(&mut OpenAiRequestBodyBuilderInner)) {
        let mut write_access = self.inner.lock().await;
        rb(&mut write_access);
    }

    pub async fn remove_tool_calls(&self) {
        let mut write_access = self.inner.lock().await;
        write_access.remove_tool_calls();
    }

    pub async fn modify_and_get_result<TResult>(
        &self,
        rb: impl Fn(&mut OpenAiRequestBodyBuilderInner) -> TResult,
    ) -> TResult {
        let mut write_access = self.inner.lock().await;
        rb(&mut write_access)
    }

    pub async fn get<TResult>(
        &self,
        callback: impl Fn(&OpenAiRequestBodyBuilderInner) -> TResult,
    ) -> TResult {
        let read_access = self.inner.lock().await;
        callback(&read_access)
    }

    pub async fn write_tech_log(&self, itm: TechRequestLogItem) {
        let mut write_access = self.inner.lock().await;
        write_access.write_tech_log(itm);
    }

    pub async fn get_tech_log(&self) -> TechRequestLogger {
        let mut write_access = self.inner.lock().await;
        std::mem::take(&mut write_access.tech_log)
    }
}
