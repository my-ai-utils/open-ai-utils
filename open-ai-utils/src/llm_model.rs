#[derive(Debug, Clone, Copy)]
pub enum LlmModel {
    Gpt4o,
    Gpt4oMini,
    Gpt5,
    Gpt5Mini,
    Gpt5Nano,
    Qwen3_30bA3b,
}

impl LlmModel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LlmModel::Gpt4o => "gpt-4o",
            LlmModel::Gpt4oMini => "gpt-4o-mini",
            LlmModel::Gpt5 => "gpt-5",
            LlmModel::Gpt5Mini => "gpt-5-mini",
            LlmModel::Gpt5Nano => "gpt-5-nano",
            LlmModel::Qwen3_30bA3b => "Qwen/Qwen3-30B-A3B",
        }
    }

    pub fn is_gpt_5(&self) -> bool {
        match self {
            LlmModel::Gpt5 => true,
            LlmModel::Gpt5Mini => true,
            LlmModel::Gpt5Nano => true,
            _ => false,
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
