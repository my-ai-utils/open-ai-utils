use crate::{OpenAiRequestBodyBuilder, my_auto_gen::TechRequestLogItem};

use super::*;
use rust_extensions::date_time::DateTimeAsMicroseconds;

pub struct OpenAiInnerResponseStream {
    network_stream: OpenAiNetworkStream,
    streamed_response_reader: StreamedResponseReader,
    tool_calls: Vec<ToolCallChunkHttpModel>,
    eof: bool,
}

impl OpenAiInnerResponseStream {
    pub fn new(network_stream: OpenAiNetworkStream) -> Self {
        Self {
            network_stream,
            streamed_response_reader: StreamedResponseReader::new(),
            tool_calls: vec![],
            eof: false,
        }
    }

    pub async fn get_next_chunk(
        &mut self,
        rb: &OpenAiRequestBodyBuilder,
    ) -> Result<Option<OpenAiStreamHttpChunk>, String> {
        if self.eof {
            return Ok(None);
        }

        loop {
            let Some(data) = self.streamed_response_reader.try_get_next_chunk() else {
                let next_chunk_from_http = self.network_stream.get_next_chunk().await.unwrap();

                if next_chunk_from_http.is_none() {
                    self.eof = true;
                    break;
                }

                let next_chunk_from_http = next_chunk_from_http.unwrap();

                rb.write_tech_log(TechRequestLogItem::new_data_as_str(
                    DateTimeAsMicroseconds::now(),
                    crate::my_auto_gen::TechLogItemType::Chunk,
                    std::str::from_utf8(next_chunk_from_http.as_slice()).unwrap(),
                ))
                .await;

                self.streamed_response_reader
                    .append_data(&next_chunk_from_http);
                continue;
            };

            let mut data = match data {
                StreamReaderResult::Data(data) => data,
                StreamReaderResult::Done => {
                    if self.tool_calls.len() > 0 {
                        let tool_calls = std::mem::take(&mut self.tool_calls);
                        return Ok(Some(OpenAiStreamHttpChunk::ToolCalls(tool_calls)));
                    }

                    return Ok(None);
                }
            };

            while let Some(choice) = data.get_choice() {
                if let Some(content) = choice.delta.content {
                    if content.len() > 0 {
                        return Ok(Some(OpenAiStreamHttpChunk::Text(content)));
                    }
                }

                if let Some(tool_calls) = choice.delta.tool_calls {
                    for tool_call in tool_calls {
                        if let Some(id) = tool_call.id {
                            self.tool_calls.push(ToolCallChunkHttpModel {
                                id: id,
                                fn_name: Default::default(),
                                params: Default::default(),
                            });
                        }

                        if let Some(function) = tool_call.function {
                            if let Some(fn_name) = function.name {
                                if let Some(last) = self.tool_calls.last_mut() {
                                    last.fn_name.push_str(fn_name.as_str());
                                }
                            }
                            if let Some(params) = function.arguments {
                                if let Some(last) = self.tool_calls.last_mut() {
                                    last.params.push_str(params.as_str());
                                }
                            }
                        }
                    }
                }
            }
        }

        if self.tool_calls.len() > 0 {
            let tool_calls = std::mem::take(&mut self.tool_calls);
            return Ok(Some(OpenAiStreamHttpChunk::ToolCalls(tool_calls)));
        }

        Ok(None)
    }
}
