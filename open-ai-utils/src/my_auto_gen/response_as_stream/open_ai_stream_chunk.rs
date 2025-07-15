#[derive(Debug)]
pub enum OpenAiStreamChunk {
    Text(String),
    ToolCall {
        id: String,
        fn_name: String,
        params: String,
        result: String,
    },
}
