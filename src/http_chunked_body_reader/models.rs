use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChunkResultModel {
    pub id: String,
    pub choices: Vec<ChunkModelChose>,
}

impl ChunkResultModel {
    pub fn get_text(&self) -> Option<&str> {
        let choices = self.choices.get(0)?;
        choices.delta.content.as_deref()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChunkModelChose {
    pub delta: ChunkChoseDelta,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChunkChoseDelta {
    pub content: Option<String>,
    pub tool_calls: Option<Vec<ToolsCallModel>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolsCallModel {
    #[serde(rename = "type")]
    r#type: String,
}

pub struct FunctionCallModel {
    pub name: Option<String>,
    pub arguments: Option<String>,
}
