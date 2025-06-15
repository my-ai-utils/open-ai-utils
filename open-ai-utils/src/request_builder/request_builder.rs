use rust_extensions::StrOrString;

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

    pub fn get_history_to_serialize(&self) -> &[OpenAiMessageModel] {
        &self.model.messages[1..]
    }

    pub fn from_history(
        system_prompt: impl Into<StrOrString<'static>>,
        history: Vec<OpenAiMessageModel>,
        model: LlmModel,
    ) -> Self {
        let system_prompt: StrOrString<'static> = system_prompt.into();
        let mut messages = vec![OpenAiMessageModel {
            role: "system".to_owned(),
            content: Some(system_prompt.to_string()),
            tool_calls: None,
            tool_call_id: None,
        }];

        messages.extend(history);

        Self {
            model: OpenAiRequestModel {
                model: model.to_string(),
                tools: vec![],
                messages,
            },
        }
    }

    pub fn add_tool_calls(&mut self, func_description: FunctionDescriptionJsonModel) {
        self.model.tools.push(func_description);
    }

    pub fn get_model(&self) -> &OpenAiRequestModel {
        &self.model
    }
}
