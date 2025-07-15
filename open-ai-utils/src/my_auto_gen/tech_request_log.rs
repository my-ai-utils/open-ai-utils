use rust_extensions::date_time::DateTimeAsMicroseconds;
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
    pub data: String,
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
