use rust_extensions::StrOrString;

use crate::{
    my_auto_gen::{TechRequestLogItem, TechRequestLogger, ToolCallModel},
    roles::*,
};

use super::*;

pub struct OpenAiRequestBodyBuilderInner {
    tools: Vec<ToolsDescriptionJsonModel>,
    model: OpenAiRequestModel,
    pub(crate) tech_log: TechRequestLogger,
}

impl OpenAiRequestBodyBuilderInner {
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
                frequency_penalty: None,
                presence_penalty: None,
                n: None,
            },
            tech_log: Default::default(),
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
                frequency_penalty: None,
                presence_penalty: None,
                n: None,
            },
            tech_log: Default::default(),
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

    pub fn add_system_message(&mut self, message: impl Into<String>) {
        let message = message.into();
        self.model.messages.push(OpenAiMessageModel {
            role: SYSTEM_ROLE.to_owned(),
            content: Some(message),
            tool_calls: None,
            tool_call_id: None,
        });
    }

    pub fn add_user_message(&mut self, message: impl Into<String>) {
        let message = message.into();
        self.model.messages.push(OpenAiMessageModel {
            role: USER_ROLE.to_owned(),
            content: Some(message),
            tool_calls: None,
            tool_call_id: None,
        });
    }

    pub fn add_assistant_message(&mut self, message: impl Into<String>) {
        self.model.messages.push(OpenAiMessageModel {
            role: ASSISTANT_ROLE.to_owned(),
            content: Some(message.into()),
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

    pub fn add_ok_tool_call_response(&mut self, src: &ToolCallModel, result: String) {
        self.model.messages.push(OpenAiMessageModel {
            role: TOOL_ROLE.to_owned(),
            content: Some(result),
            tool_calls: None,
            tool_call_id: Some(src.id.to_string()),
        });
    }

    pub fn get_history(&self) -> &[OpenAiMessageModel] {
        self.model.messages.as_slice()
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
                frequency_penalty: None,
                presence_penalty: None,
                n: None,
            },
            tools: vec![],
            tech_log: Default::default(),
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

    /*
    pub fn set_other_parameters(&mut self, other_request_data: OtherRequestData) {
        self.model.temperature = other_request_data.temperature;
        self.model.top_p = other_request_data.top_p;
        self.model.n = other_request_data.n;
        self.model.presence_penalty = other_request_data.presence_penalty;
        self.model.frequency_penalty = other_request_data.frequency_penalty;
    }
     */

    pub fn get_model(&mut self, other_request_data: &OtherRequestData) -> OpenAiRequestModel {
        if self.tools.len() > 0 {
            if self.model.tools.is_none() {
                self.model.tools = Some(serde_json::to_value(&self.tools).unwrap());
            }
        }

        let mut result = self.model.clone();

        result.n = other_request_data.n;
        result.presence_penalty = other_request_data.presence_penalty;
        result.frequency_penalty = other_request_data.frequency_penalty;
        result.top_p = other_request_data.top_p;
        result.temperature = other_request_data.temperature;

        result
    }

    pub fn remove_tool_calls(&mut self) {
        self.model.messages.retain(|itm| {
            if itm.role == ASSISTANT_ROLE && itm.tool_calls.is_some() {
                return false;
            }

            if itm.role == TOOL_ROLE {
                return false;
            }

            true
        });
    }

    pub fn get_last_message(&self) -> &OpenAiMessageModel {
        self.model.messages.last().unwrap()
    }

    pub fn write_tech_log(&mut self, item: TechRequestLogItem) {
        self.tech_log.add(item);
    }
}
