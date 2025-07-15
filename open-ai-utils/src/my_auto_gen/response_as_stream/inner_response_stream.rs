use crate::{OpenAiRequestBodyBuilder, my_auto_gen::TechRequestLogItem};

use super::*;
use flurl::FlResponseAsStream;
use rust_extensions::date_time::DateTimeAsMicroseconds;

pub struct OpenAiInnerResponseStream {
    fl_url_response: FlResponseAsStream,
    streamed_response_reader: StreamedResponseReader,
    current_tool_call: Option<CurrentToolCall>,
    eof: bool,
}

impl OpenAiInnerResponseStream {
    pub fn new(fl_url_response: FlResponseAsStream) -> Self {
        Self {
            fl_url_response,
            streamed_response_reader: StreamedResponseReader::new(),
            current_tool_call: None,
            eof: false,
        }
    }

    pub async fn get_next_chunk(
        &mut self,
        rb: &OpenAiRequestBodyBuilder,
    ) -> Result<Option<OpenAiStreamChunk>, String> {
        if self.eof {
            return Ok(None);
        }

        loop {
            let Some(mut data) = self.streamed_response_reader.try_get_next_chunk() else {
                let next_chunk_from_http = self.fl_url_response.get_next_chunk().await.unwrap();

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

            while let Some(choice) = data.get_choice() {
                if let Some(content) = choice.delta.content {
                    if content.len() > 0 {
                        self.current_tool_call = None;
                        return Ok(Some(OpenAiStreamChunk::Text(content)));
                    }
                }

                if let Some(tool_calls) = choice.delta.tool_calls {
                    for tool_call in tool_calls {
                        if let Some(function) = tool_call.function {
                            if let Some(id) = function.id {
                                match &mut self.current_tool_call {
                                    Some(c_tc) => {
                                        c_tc.id.push_str(id.as_str());
                                    }
                                    None => {
                                        self.current_tool_call = Some(CurrentToolCall {
                                            id: id,
                                            arguments: Default::default(),
                                            name: Default::default(),
                                        })
                                    }
                                }
                            }
                            if let Some(fn_name) = function.name {
                                match &mut self.current_tool_call {
                                    Some(c_tc) => {
                                        c_tc.name.push_str(fn_name.as_str());
                                    }
                                    None => {
                                        self.current_tool_call = Some(CurrentToolCall {
                                            name: fn_name,
                                            arguments: Default::default(),
                                            id: Default::default(),
                                        })
                                    }
                                }
                            }
                            if let Some(params) = function.arguments {
                                match &mut self.current_tool_call {
                                    Some(c_tc) => {
                                        c_tc.arguments.push_str(params.as_str());
                                    }
                                    None => {
                                        self.current_tool_call = Some(CurrentToolCall {
                                            id: Default::default(),
                                            name: Default::default(),
                                            arguments: params,
                                        })
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        if let Some(tool_call) = self.current_tool_call.take() {
            return Ok(Some(OpenAiStreamChunk::ToolCall {
                fn_name: tool_call.name,
                params: tool_call.arguments,
                id: tool_call.id,
                result: String::new(),
            }));
        }

        Ok(None)
    }
}

/*
async fn read_fl_url_stream(
    mut fl_url_response: FlResponseAsStream,
    sender: tokio::sync::mpsc::Sender<Result<OpenAiStreamChunk, String>>,
) {
}
 */
pub struct CurrentToolCall {
    pub name: String,
    pub arguments: String,
    pub id: String,
}
