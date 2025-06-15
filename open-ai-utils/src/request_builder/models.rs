use serde::*;
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAiRequestModel {
    pub messages: Vec<OpenAiMessageModel>,
    pub tools: Vec<Value>,
    pub model: String,
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
