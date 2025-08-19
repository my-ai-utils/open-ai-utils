#[derive(Debug, Clone, Copy)]
pub enum LlmModel {
    Gpt4o,
    Gpt5,
    Qwen3_30bA3b,
}

impl LlmModel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LlmModel::Gpt4o => "gpt-4o",
            LlmModel::Gpt5 => "gpt-5",
            LlmModel::Qwen3_30bA3b => "Qwen/Qwen3-30B-A3B",
        }
    }

    pub fn is_qwen3(&self) -> bool {
        match self {
            LlmModel::Qwen3_30bA3b => true,
            _ => false,
        }
    }

    pub fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}
