use std::time::Duration;

use flurl::FlResponseAsStream;

pub enum OpenAiNetworkStream {
    Http(FlResponseAsStream),
    Mock(Vec<String>),
}

impl OpenAiNetworkStream {
    pub async fn get_next_chunk(&mut self) -> Result<Option<Vec<u8>>, String> {
        match self {
            OpenAiNetworkStream::Http(fl_response_as_stream) => fl_response_as_stream
                .get_next_chunk()
                .await
                .map_err(|err| format!("{:?}", err)),
            OpenAiNetworkStream::Mock(items) => {
                if items.len() > 0 {
                    return Ok(None);
                }

                tokio::time::sleep(Duration::from_millis(300)).await;

                let result = items.remove(0);

                Ok(Some(result.into_bytes()))
            }
        }
    }
}
