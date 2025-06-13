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

    pub fn get_func(&self) -> Option<(&str, &str)> {
        let choices = self.choices.get(0)?;
        let tool_calls = choices.delta.tool_calls.as_ref()?;

        let first_tool_call = tool_calls.get(0)?;

        if let Some(function) = first_tool_call.function.as_ref() {
            let name = if let Some(name) = function.name.as_ref() {
                name.as_str()
            } else {
                ""
            };

            let args = if let Some(args) = function.arguments.as_ref() {
                args.as_str()
            } else {
                ""
            };

            return Some((name, args));
        }
        None
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
    pub function: Option<FunctionCallModel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionCallModel {
    pub name: Option<String>,
    pub arguments: Option<String>,
}
