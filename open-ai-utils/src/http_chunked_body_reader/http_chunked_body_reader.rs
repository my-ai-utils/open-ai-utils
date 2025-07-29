use rust_extensions::array_of_bytes_iterator::{ArrayOfBytesIterator, VecIterator};

const PACKET_MARKET: &'static [u8] = b"data:";

const END_PACKET: &'static [u8] = b"[DONE]";

#[derive(Default)]
pub struct HttpOpenAiChunkedBodyReader {
    buffer: VecIterator,
}

impl HttpOpenAiChunkedBodyReader {
    pub fn extend(&mut self, data: &[u8]) {
        self.buffer.extend(data);
    }
    pub fn get_next_data(&mut self) -> Result<Option<super::ChunkResultModel>, NextDataErr> {
        let from_pos = self
            .buffer
            .peek_and_find_sequence_pos_from_current_pos(PACKET_MARKET);

        let Some(from_pos) = from_pos else {
            return Err(NextDataErr::NotEnoughData);
        };

        let next_pos = self
            .buffer
            .peek_and_find_sequence_pos(from_pos + PACKET_MARKET.len(), PACKET_MARKET);

        let check_slice_result = match next_pos {
            Some(next_pos) => {
                let slice = &self.buffer.get_src_slice()[from_pos + PACKET_MARKET.len()..next_pos];
                let result = check_slice(slice);

                if result.is_ok() {
                    self.buffer.set_pos(next_pos);
                    self.buffer.gc();
                }

                result
            }

            None => {
                let slice = &self.buffer.get_src_slice()[from_pos + PACKET_MARKET.len()..];
                let result = check_slice(slice);

                if result.is_ok() {
                    self.buffer.clear();
                }

                result
            }
        };

        check_slice_result.map_err(|_| NextDataErr::NotEnoughData)
    }
}

fn check_slice(slice: &[u8]) -> Result<Option<super::ChunkResultModel>, ()> {
    //   println!("```{}```", std::str::from_utf8(slice).unwrap());
    let pos_from = trim_from_begin(slice);
    let pos_to = trim_from_end(slice);

    let slice = &slice[pos_from..pos_to];

    //  println!("```{}```", std::str::from_utf8(slice).unwrap());
    //  println!("----");

    if slice == END_PACKET {
        return Ok(None);
    }

    let result: Result<super::ChunkResultModel, _> = serde_json::from_slice(slice);

    match result {
        Ok(result) => Ok(Some(result)),
        Err(_) => Err(()),
    }
}

fn trim_from_begin(slice: &[u8]) -> usize {
    for (index, b) in slice.iter().enumerate() {
        if *b > 32 {
            return index;
        }
    }

    0
}

fn trim_from_end(slice: &[u8]) -> usize {
    let mut pos = slice.len() - 1;

    while pos > 0 {
        if slice[pos] > 32 {
            return pos + 1;
        }

        pos -= 1;
    }

    slice.len()
}

#[derive(Debug)]
pub enum NextDataErr {
    NotEnoughData,
    Other(String),
}

impl NextDataErr {
    pub fn is_not_enough_data(&self) -> bool {
        match self {
            NextDataErr::NotEnoughData => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::http_chunked_body_reader::HttpOpenAiChunkedBodyReader;

    #[test]
    fn test_reading() {
        let mut body_reader = HttpOpenAiChunkedBodyReader::default();

        let next_data = body_reader.get_next_data().unwrap_err();

        assert!(next_data.is_not_enough_data());

        let next_chunk = r#"data: {"id":"chatcmpl-BgGPdEggKnxk1Pi4v7OCisP3t1eKi","object":"chat.completion.chunk","created":1749412565,"model":"gpt-4o-2024-08-06","service_tier":"default","system_fingerprint":"fp_07871e2ad8","choices":[{"index":0,"delta":{"content":" I"},"logprobs":null,"finish_reason":null}]}"#;
        body_reader.extend(next_chunk.as_bytes());

        let next_data = body_reader.get_next_data().unwrap();

        let next_data = next_data.unwrap();

        assert_eq!(next_data.get_text().unwrap(), " I");

        let next_data = body_reader.get_next_data().unwrap_err();

        assert!(next_data.is_not_enough_data());

        let next_chunk = r#"data: {"id":"chatcmpl-BgGPdEggKnxk1Pi4v7OCisP3t1eKi","object":"chat.completion.chunk","created":1749412565,"model":"gpt-4o-2024-08-06","service_tier":"default","system_fingerprint":"fp_07871e2ad8","choices":[{"index":0,"delta":{"content":"’m"},"logprobs":null,"finish_reason":null}]}

data: {"id":"chatcmpl-BgGPdEggKnxk1Pi4v7OCisP3t1eKi","object":"chat.completion.chunk","created":1749412565,"model":"gpt-4o-2024-08-06","service_tier":"default","system_fingerprint":"fp_07871e2ad8","choices":[{"index":0,"delta":{"content":" Sarah"},"logprobs":null,"finish_reason":null}]}"#;
        body_reader.extend(next_chunk.as_bytes());

        let next_data = body_reader.get_next_data().unwrap();

        let next_data = next_data.unwrap();

        assert_eq!(next_data.get_text().unwrap(), "’m");

        let next_data = body_reader.get_next_data().unwrap();

        let next_data = next_data.unwrap();

        assert_eq!(next_data.get_text().unwrap(), " Sarah");

        let next_chunk = r#"data: {"id":"chatcmpl-BgGPdEggKnxk1Pi4v7OCisP3t1eKi","object":"chat.completion.chunk","created":1749412565,"model":"gpt-4o-2024-08-06","service_tier":"default","system"#;
        body_reader.extend(next_chunk.as_bytes());
        let next_data = body_reader.get_next_data().unwrap_err();
        assert!(next_data.is_not_enough_data());

        let next_chunk = r#"_fingerprint":"fp_07871e2ad8","choices":[{"index":0,"delta":{"content":"!"},"logprobs":null,"finish_reason":null}]}"#;
        body_reader.extend(next_chunk.as_bytes());

        let next_data = body_reader.get_next_data().unwrap();

        let next_data = next_data.unwrap();

        assert_eq!(next_data.get_text().unwrap(), "!");

        let next_chunk = "  data: [DONE]   ";
        body_reader.extend(next_chunk.as_bytes());

        let next_data = body_reader.get_next_data().unwrap();

        assert!(next_data.is_none());
    }

    #[test]
    fn parse_func() {
        let mut body_reader = HttpOpenAiChunkedBodyReader::default();
        let next_chunk = r#"data: {"id":"chatcmpl-BgcBfWT77LRKMykIMKs2hJnCo6QYh","object":"chat.completion.chunk","created":1749496267,"model":"gpt-4o-2024-08-06","service_tier":"default","system_fingerprint":"fp_07871e2ad8","choices":[{"index":0,"delta":{"tool_calls":[{"index":0,"function":{"arguments":"\",\""}}]},"logprobs":null,"finish_reason":null}]}"#;

        body_reader.extend(next_chunk.as_bytes());

        let next = body_reader.get_next_data().unwrap().unwrap();

        println!("{:?}", next.get_func());
    }
}
