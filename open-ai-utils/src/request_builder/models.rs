use serde::*;

use crate::{roles::*, *};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAiRequestModel {
    pub messages: Vec<OpenAiMessageModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<serde_json::Value>,
    pub model: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<Gpt5ReasoningEffort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbosity: Option<Gpt5VerbosityEffort>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

impl OpenAiRequestModel {
    pub fn new(model: LlmModel, messages: Vec<OpenAiMessageModel>) -> Self {
        let mut result = OpenAiRequestModel {
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
            reasoning_effort: None,
            verbosity: None,
        };
        match model.as_settings() {
            SettingsMode::Gpt4(settings) => {
                result.presence_penalty = settings.presence_penalty;
                result.frequency_penalty = settings.frequency_penalty;
                result.n = settings.n;
                result.top_p = settings.top_p;
                result.temperature = settings.temperature;
            }
            SettingsMode::Gpt5(settings) => {
                result.reasoning_effort = settings.reasoning_effort;
                result.verbosity = settings.verbosity;
            }
            SettingsMode::Qwen(_) => {}
        }

        result
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAiMessageModel {
    pub role: String,
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCallsModel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

impl MessageRole for OpenAiMessageModel {
    fn get_role(&self) -> &str {
        &self.role
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolCallsModel {
    pub id: String,
    #[serde(rename = "type")]
    pub tp: String,
    pub function: ToolCallFunctionDescription,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolCallFunctionDescription {
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionDescriptionJsonModel {
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
    pub parameters: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolsDescriptionJsonModel {
    #[serde(rename = "type")]
    pub tp: String,
    pub function: Option<serde_json::Value>,
}
