use serde::{Deserialize, Serialize};

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
    #[serde(rename = "minimal")]
    Minimal,
    #[serde(rename = "low")]
    Low,
}
