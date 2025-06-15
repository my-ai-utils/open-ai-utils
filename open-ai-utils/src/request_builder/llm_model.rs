#[derive(Debug, Clone, Copy)]
pub enum LlmModel {
    Gpt4o,
}

impl LlmModel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LlmModel::Gpt4o => "gpt-4o",
        }
    }

    pub fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}
