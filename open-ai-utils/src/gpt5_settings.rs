use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default)]
pub struct Gpt5Settings {
    pub reasoning_effort: Option<Gpt5ReasoningEffort>,
    pub verbosity: Option<Gpt5VerbosityEffort>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Gpt5ReasoningEffort {
    #[serde(rename = "hight")]
    Hight,
    #[serde(rename = "minimal")]
    Minimal,
    #[serde(rename = "low")]
    Low,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Gpt5VerbosityEffort {
    #[serde(rename = "hight")]
    Hight,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "low")]
    Low,
}
