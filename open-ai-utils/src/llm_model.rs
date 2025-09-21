use crate::*;

#[derive(Debug, Clone, Copy)]
pub enum LlmModel {
    Gpt4o(Gpt4Settings),
    Gpt4oMini(Gpt4Settings),
    Gpt5(Gpt5Settings),
    Gpt5Mini(Gpt5Settings),
    Gpt5Nano(Gpt5Settings),
    Qwen3_30bA3b(QwenSettings),
}

impl LlmModel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LlmModel::Gpt4o(_) => "gpt-4o",
            LlmModel::Gpt4oMini(_) => "gpt-4o-mini",
            LlmModel::Gpt5(_) => "gpt-5",
            LlmModel::Gpt5Mini(_) => "gpt-5-mini",
            LlmModel::Gpt5Nano(_) => "gpt-5-nano",
            LlmModel::Qwen3_30bA3b(_) => "Qwen/Qwen3-30B-A3B",
        }
    }

    pub fn is_gpt_5(&self) -> bool {
        match self {
            LlmModel::Gpt5(_) => true,
            LlmModel::Gpt5Mini(_) => true,
            LlmModel::Gpt5Nano(_) => true,
            _ => false,
        }
    }
    pub fn is_qwen3(&self) -> bool {
        match self {
            LlmModel::Qwen3_30bA3b(_) => true,
            _ => false,
        }
    }

    pub fn to_string(&self) -> String {
        self.as_str().to_string()
    }
    pub fn as_settings(&self) -> SettingsMode {
        match self {
            LlmModel::Gpt4o(settings) => SettingsMode::Gpt4(*settings),
            LlmModel::Gpt4oMini(settings) => SettingsMode::Gpt4(*settings),
            LlmModel::Gpt5(gpt5_settings) => SettingsMode::Gpt5(*gpt5_settings),
            LlmModel::Gpt5Mini(gpt5_settings) => SettingsMode::Gpt5(*gpt5_settings),
            LlmModel::Gpt5Nano(gpt5_settings) => SettingsMode::Gpt5(*gpt5_settings),
            LlmModel::Qwen3_30bA3b(settings) => SettingsMode::Qwen(*settings),
        }
    }

    pub fn is_qwen_think(&self) -> Option<bool> {
        match self.as_settings() {
            SettingsMode::Gpt4(_) => None,
            SettingsMode::Gpt5(_) => None,
            SettingsMode::Qwen(qwen_settings) => Some(qwen_settings.think),
        }
    }
}

pub enum SettingsMode {
    Gpt4(Gpt4Settings),
    Gpt5(Gpt5Settings),
    Qwen(QwenSettings),
}
