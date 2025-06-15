use rust_extensions::StrOrString;

use crate::FunctionToolCallDescription;

use super::*;

pub struct OpenAiRequestBodyBuilder {
    model: OpenAiRequestModel,
}

impl OpenAiRequestBodyBuilder {
    pub fn new(system_prompt: impl Into<StrOrString<'static>>, model: LlmModel) -> Self {
        let system_prompt: StrOrString<'static> = system_prompt.into();
        Self {
            model: OpenAiRequestModel {
                model: model.to_string(),
                tools: vec![],
                messages: vec![OpenAiMessageModel {
                    role: "system".to_owned(),
                    content: Some(system_prompt.to_string()),
                    tool_calls: None,
                    tool_call_id: None,
                }],
            },
        }
    }

    pub fn add_user_message(&mut self, message: String) {
        self.model.messages.push(OpenAiMessageModel {
            role: "user".to_owned(),
            content: Some(message),
            tool_calls: None,
            tool_call_id: None,
        });
    }

    pub fn add_assistant_message(&mut self, message: String) {
        self.model.messages.push(OpenAiMessageModel {
            role: "assistant".to_owned(),
            content: Some(message),
            tool_calls: None,
            tool_call_id: None,
        });
    }

    pub fn add_tool_calls<TToolCallModel: FunctionToolCallDescription>(&mut self) {
        self.model.tools.push(FunctionToolCallModel {
            tp: "function".to_string(),
            function: TToolCallModel::get_description(),
        });
    }

    pub fn get_model(&self) -> &OpenAiRequestModel {
        &self.model
    }
}
