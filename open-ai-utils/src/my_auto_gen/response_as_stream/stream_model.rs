use serde::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamJsonModel {
    pub choices: Vec<OpenAiStreamChoice>,
}

impl StreamJsonModel {
    pub fn get_choice(&mut self) -> Option<OpenAiStreamChoice> {
        if self.choices.len() == 0 {
            return None;
        }

        Some(self.choices.remove(0))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAiStreamChoice {
    pub delta: DeltaAiStreamData,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeltaAiStreamData {
    pub role: Option<String>,
    pub content: Option<String>,
    pub tool_calls: Option<Vec<ToolCallsJsonModel>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolCallsJsonModel {
    pub id: Option<String>,
    pub r#type: Option<String>,
    pub function: Option<ToolCallFuncDescriptionJsonModel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolCallFuncDescriptionJsonModel {
    pub name: Option<String>,
    pub arguments: Option<String>,
}
