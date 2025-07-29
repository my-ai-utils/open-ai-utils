use rust_extensions::slice_of_u8_utils::SliceOfU8Ext;

use crate::my_auto_gen::StreamJsonModel;

const DATA_PREFIX: &'static [u8] = b"data: ";
const DATA_SUFFIX: &'static [u8] = b"\n\n";
pub struct StreamedResponseReader {
    data: Vec<u8>,
}

impl StreamedResponseReader {
    pub fn new() -> Self {
        Self { data: vec![] }
    }
    pub fn append_data(&mut self, delta: &[u8]) {
        self.data.extend_from_slice(delta);
    }

    pub fn try_get_next_chunk(&mut self) -> Option<StreamJsonModel> {
        if self.data.len() < DATA_PREFIX.len() + DATA_SUFFIX.len() {
            return None;
        }

        let end_pos = self.data.find_sequence_pos(DATA_SUFFIX, 0)?;

        let chunk = &self.data[DATA_PREFIX.len()..end_pos];

        if chunk.ends_with(b"[DONE]") {
            return None;
        }

        //  println!("`{}`", std::str::from_utf8(chunk).unwrap());

        let result: Result<StreamJsonModel, serde_json::Error> = serde_json::from_slice(chunk);

        let result = match result {
            Ok(result) => result,
            Err(err) => {
                match std::str::from_utf8(self.data.as_slice()) {
                    Ok(json) => {
                        println!("Json: {}", json);
                    }
                    Err(_) => {
                        println!("Invalid Json. Got Binary {} bytes", chunk.len());
                    }
                }

                panic!("Can not deserialize Streamed model. Err: {}", err);
            }
        };

        self.data.drain(..end_pos + 2);

        // let sub_chunk = &self.data[DATA_PREFIX.len()..];

        Some(result)
    }
}
