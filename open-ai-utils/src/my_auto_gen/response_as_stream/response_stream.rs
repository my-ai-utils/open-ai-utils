use crate::my_auto_gen::*;

pub struct OpenAiResponseStream {
    rx: tokio::sync::mpsc::Receiver<Result<OpenAiStreamChunk, String>>,
}

impl OpenAiResponseStream {
    pub fn new() -> (
        Self,
        tokio::sync::mpsc::Sender<Result<OpenAiStreamChunk, String>>,
    ) {
        let (tx, rx) = tokio::sync::mpsc::channel(32768);
        let result = Self { rx };

        (result, tx)
    }

    pub async fn get_next(&mut self) -> Result<Option<OpenAiStreamChunk>, String> {
        let next_one = self.rx.recv().await;

        let Some(next_one) = next_one else {
            return Ok(None);
        };

        match next_one {
            Ok(value) => Ok(Some(value)),
            Err(err) => Err(err),
        }
    }
}
