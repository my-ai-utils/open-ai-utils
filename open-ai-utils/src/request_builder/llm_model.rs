#[derive(Debug, Clone, Copy)]
pub enum LlmModel {
    Gpt4o,
    Gpt5,
    Qwen3_32B,
}

impl LlmModel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LlmModel::Gpt4o => "gpt-4o",
            LlmModel::Gpt5 => "gpt-5",
            LlmModel::Qwen3_32B => "Qwen/Qwen3-32B",
        }
    }

    pub fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}
