#[derive(Debug)]
pub enum OpenAiStreamHttpChunk {
    Text(String),
    ToolCalls(Vec<ToolCallChunkHttpModel>),
    Done,
}

#[derive(Debug)]
pub struct ToolCallChunkHttpModel {
    pub id: String,
    pub fn_name: String,
    pub params: String,
}

#[derive(Debug)]
pub enum OpenAiStreamChunk {
    Text(String),
    ToolCalls(Vec<ToolCallChunkModel>),
}

#[derive(Debug)]
pub struct ToolCallChunkModel {
    pub id: String,
    pub fn_name: String,
    pub params: String,
    pub result: String,
}
