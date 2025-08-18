use serde::{Deserialize, Serialize};

use crate::ToolCallFunctionDescription;

pub enum OpenAiResponse<'s> {
    Message(&'s str),
    ToolCall(&'s [ToolCallModel]),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAiRespModel {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<OpenAiChoiceModel>,
}

impl OpenAiRespModel {
    pub fn peek_message<'s>(&'s self) -> Option<OpenAiResponse<'s>> {
        let first_choice = self.choices.first()?;

        if let Some(content) = first_choice.message.content.as_ref() {
            let result = OpenAiResponse::Message(content);
            return Some(result);
        }

        if let Some(tool_calls) = first_choice.message.tool_calls.as_ref() {
            return Some(OpenAiResponse::ToolCall(tool_calls.as_slice()));
        }

        None
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAiChoiceModel {
    pub index: i64,
    pub message: OpenAiMessageModel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAiMessageModel {
    pub role: String,
    pub content: Option<String>,
    pub tool_calls: Option<Vec<ToolCallModel>>,
}

impl OpenAiMessageModel {
    pub fn is_system(&self) -> bool {
        self.role == crate::request_builder::roles::SYSTEM_ROLE
    }

    pub fn is_user(&self) -> bool {
        self.role == crate::request_builder::roles::USER_ROLE
    }

    pub fn is_assistant(&self) -> bool {
        self.role == crate::request_builder::roles::ASSISTANT_ROLE
    }

    pub fn is_tool(&self) -> bool {
        self.role == crate::request_builder::roles::TOOL_ROLE
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolCallModel {
    pub id: String,
    #[serde(rename = "type")]
    pub tp: String,
    pub function: ToolCallFunctionDescription,
}
