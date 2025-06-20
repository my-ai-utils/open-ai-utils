use rust_extensions::StrOrString;

use crate::my_auto_gen::ToolCallModel;

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
                max_tokens: None,
                temperature: None,
                top_p: None,
                stream: None,
            },
        }
    }

    pub fn set_max_tokens(&mut self, value: usize) {
        self.model.max_tokens = Some(value);
    }

    pub fn set_top_p(&mut self, value: f64) {
        self.model.top_p = Some(value);
    }

    pub fn set_temperature(&mut self, value: f64) {
        self.model.temperature = Some(value);
    }

    pub fn set_stream(&mut self) {
        self.model.stream = Some(true);
    }

    pub fn add_user_message(&mut self, message: impl Into<StrOrString<'static>>) {
        let message = message.into();
        self.model.messages.push(OpenAiMessageModel {
            role: "user".to_owned(),
            content: Some(message.to_string()),
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

    pub fn add_assistant_response_as_tool_calls(
        &mut self,
        tool_calls_ai_response: &[ToolCallModel],
    ) {
        let mut tool_calls = Vec::new();

        for itm in tool_calls_ai_response {
            tool_calls.push(ToolCallsModel {
                id: itm.id.to_string(),
                tp: itm.tp.to_string(),
                function: itm.function.clone(),
            });
        }

        self.model.messages.push(OpenAiMessageModel {
            role: "assistant".to_owned(),
            content: None,
            tool_calls: Some(tool_calls),
            tool_call_id: None,
        });
    }
    pub fn add_tool_call_response(&mut self, src: &ToolCallModel, result: String) {
        self.model.messages.push(OpenAiMessageModel {
            role: "tool".to_owned(),
            content: Some(result),
            tool_calls: None,
            tool_call_id: Some(src.id.to_string()),
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
                max_tokens: None,
                temperature: None,
                top_p: None,
                stream: None,
            },
        }
    }

    pub fn add_tools_call_description(&mut self, func_description: FunctionDescriptionJsonModel) {
        self.model.tools.push(ToolsDescriptionJsonModel {
            tp: "function".to_string(),
            function: Some(func_description),
        });
    }

    pub fn get_model(&self) -> &OpenAiRequestModel {
        &self.model
    }

    pub fn get_last_message(&self) -> &OpenAiMessageModel {
        self.model.messages.last().unwrap()
    }
}
