use serde::*;

pub struct OpenAiRequestBodyBuilder {
    model: OpenAiRequestModel,
}

impl OpenAiRequestBodyBuilder {
    pub fn new(system_prompt: String) -> Self {
        Self {
            model: OpenAiRequestModel {
                messages: vec![OpenAiMessageModel {
                    role: "system".to_owned(),
                    content: Some(system_prompt),
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

    pub fn get_model(&self) -> &OpenAiRequestModel {
        &self.model
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAiRequestModel {
    pub messages: Vec<OpenAiMessageModel>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAiMessageModel {
    pub role: String,
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<ToolCallsModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolCallsModel {
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub function: ToolCallFunctionDescription,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolCallFunctionDescription {
    pub name: String,
    pub arguments: String,
}
