use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TechRequestLogItem {
    pub req_ts: DateTimeAsMicroseconds,
    pub request: String,
    pub resp_ts: DateTimeAsMicroseconds,
    pub response: String,
}

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
