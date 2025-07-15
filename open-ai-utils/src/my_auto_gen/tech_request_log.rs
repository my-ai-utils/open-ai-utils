use rust_extensions::{StrOrString, date_time::DateTimeAsMicroseconds};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum TechLogItemType {
    Request,
    Response,
    Chunk,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TechRequestLogItem {
    pub timestamp: DateTimeAsMicroseconds,
    pub tp: TechLogItemType,
    pub data: serde_json::Value,
}

impl TechRequestLogItem {
    pub fn new_data_as_str<'s>(
        timestamp: DateTimeAsMicroseconds,
        tp: TechLogItemType,
        data: impl Into<StrOrString<'s>>,
    ) -> Self {
        let data: StrOrString<'s> = data.into();
        Self {
            timestamp,
            tp,
            data: format_response(data.as_str()),
        }
    }
}

#[derive(Debug, Default)]
pub struct TechRequestLogger {
    items: Vec<TechRequestLogItem>,
}

impl TechRequestLogger {
    pub fn new() -> Self {
        Self {
            items: Default::default(),
        }
    }

    pub fn add(&mut self, item: TechRequestLogItem) {
        self.items.push(item);
    }

    pub fn into_vec(self) -> Vec<TechRequestLogItem> {
        self.items
    }
}

fn format_response(src: &str) -> serde_json::Value {
    let result: Result<serde_json::Value, _> = serde_json::from_str(src);

    match result {
        Ok(result) => result,
        Err(_) => serde_json::Value::String(src.to_string()),
    }
}
