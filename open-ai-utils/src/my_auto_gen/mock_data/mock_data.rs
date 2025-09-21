use serde::{Deserialize, Serialize};

const MOCK_DATA: &'static [u8] = std::include_bytes!("mock_data.json");

pub fn get_mock_items() -> Vec<String> {
    let mut items =
        serde_json::from_slice::<Vec<MockItem>>(MOCK_DATA).expect("Failed to parse mock data");

    items.retain(|itm| itm.tp == "Chunk");
    items
        .iter()
        .map(|itm| itm.data.as_str().unwrap().to_string())
        .collect()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MockItem {
    pub tp: String,
    pub data: serde_json::Value,
}

use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    OpenAiRequestBodyBuilder,
    my_auto_gen::{OpenAiInnerResponseStream, OpenAiNetworkStream},
};

#[tokio::test]
async fn test() {
    let settings = super::get_mock_items();

    let data = Arc::new(Mutex::new(settings));

    let network_stream = OpenAiNetworkStream::Mock(data.clone());
    let mut stream = OpenAiInnerResponseStream::new(network_stream);

    let rb = OpenAiRequestBodyBuilder::new(crate::LlmModel::Gpt4o(crate::Gpt4Settings::default()));
    while let Some(chunk) = stream.get_next_chunk(&rb).await.unwrap() {
        println!("{:?}", chunk);
    }

    let network_stream = OpenAiNetworkStream::Mock(data.clone());
    let mut stream = OpenAiInnerResponseStream::new(network_stream);

    let rb = OpenAiRequestBodyBuilder::new(crate::LlmModel::Gpt4o(crate::Gpt4Settings::default()));
    while let Some(chunk) = stream.get_next_chunk(&rb).await.unwrap() {
        println!("{:?}", chunk);
    }
}
