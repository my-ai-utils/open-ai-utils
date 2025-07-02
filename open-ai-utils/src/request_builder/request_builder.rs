use rust_extensions::StrOrString;

use crate::my_auto_gen::ToolCallModel;

use super::*;

const SYSTEM_ROLE: &'static str = "system";

const ASSISTANT_ROLE: &'static str = "assistant";

pub struct OpenAiRequestBodyBuilder {
    tools: Vec<ToolsDescriptionJsonModel>,
    model: OpenAiRequestModel,
}

impl OpenAiRequestBodyBuilder {
    pub fn new(model: LlmModel) -> Self {
        Self {
            tools: vec![],
            model: OpenAiRequestModel {
                model: model.to_string(),
                tools: serde_json::from_str("[]").unwrap(),
                messages: vec![],
                max_tokens: None,
                temperature: None,
                top_p: None,
                stream: None,
            },
        }
    }

    pub fn new_with_system_prompt(
        system_prompt: impl Into<StrOrString<'static>>,
        model: LlmModel,
    ) -> Self {
        let system_prompt: StrOrString<'static> = system_prompt.into();
        let messages = vec![OpenAiMessageModel {
            role: SYSTEM_ROLE.to_owned(),
            content: Some(system_prompt.to_string()),
            tool_calls: None,
            tool_call_id: None,
        }];

        Self {
            tools: vec![],
            model: OpenAiRequestModel {
                model: model.to_string(),
                tools: serde_json::from_str("[]").unwrap(),
                messages,
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
            role: ASSISTANT_ROLE.to_owned(),
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
            role: ASSISTANT_ROLE.to_owned(),
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
            role: SYSTEM_ROLE.to_owned(),
            content: Some(system_prompt.to_string()),
            tool_calls: None,
            tool_call_id: None,
        }];

        messages.extend(history);

        Self {
            model: OpenAiRequestModel {
                model: model.to_string(),
                tools: serde_json::from_str("[]").unwrap(),
                messages,
                max_tokens: None,
                temperature: None,
                top_p: None,
                stream: None,
            },
            tools: vec![],
        }
    }

    pub fn add_tools_call_description(&mut self, func_description: serde_json::Value) {
        self.tools.push(ToolsDescriptionJsonModel {
            tp: "function".to_string(),
            function: Some(func_description),
        });
        self.model.tools = None;
    }

    pub fn add_tools(&mut self, tools: serde_json::Value) {
        self.model.tools = Some(tools);
    }

    pub fn get_model(&mut self) -> OpenAiRequestModel {
        if self.tools.len() > 0 {
            if self.model.tools.is_none() {
                self.model.tools = Some(serde_json::to_value(&self.tools).unwrap());
            }
        }
        let mut result = self.model.clone();

        result.messages.retain(|itm| itm.content.is_some());

        result
    }

    pub fn get_last_message(&self) -> &OpenAiMessageModel {
        self.model.messages.last().unwrap()
    }
}
