use std::{sync::Arc, time::Duration};

use flurl::FlUrl;
use tokio::sync::RwLock;

use crate::{
    OpenAiRequestBodyBuilder, ToolCallFunctionDescription,
    my_auto_gen::{
        AutoGenSettings, MyAutoGenInner, OpenAiInnerResponseStream, OpenAiStreamChunk,
        ToolCallModel,
    },
};

pub async fn execute_request_as_stream(
    settings: AutoGenSettings,
    sender: tokio::sync::mpsc::Sender<Result<OpenAiStreamChunk, String>>,
    rb: Arc<OpenAiRequestBodyBuilder>,
    inner: Arc<RwLock<MyAutoGenInner>>,
    ctx: String,
) {
    loop {
        let mut fl_url = FlUrl::new(settings.url.as_str()).set_timeout(Duration::from_secs(60));

        if let Some(api_key) = settings.api_key.as_ref() {
            fl_url = fl_url.with_header("Authorization", format!("Bearer {}", api_key));
        };

        if settings.do_not_reuse_connection.unwrap_or(false) {
            fl_url = fl_url.do_not_reuse_connection();
        }

        let model = rb
            .modify_and_get_result(|rb| {
                rb.set_stream();
                rb.get_model().clone()
            })
            .await;

        let response = fl_url
            .post_json(&model)
            .await
            .map_err(|itm| itm.to_string());

        let response = match response {
            Ok(response) => response,
            Err(err) => {
                let _ = sender.send(Err(err)).await;
                return;
            }
        };

        let status_code = response.get_status_code();

        if status_code != 200 {
            let body = response.receive_body().await.unwrap();
            println!("OpenAI status code: {}", status_code);
            println!("{:?}", std::str::from_utf8(body.as_slice()));
            let err = format!("Status code: {}", status_code);

            let _ = sender.send(Err(err)).await;
            return;
        }

        let mut response = OpenAiInnerResponseStream::new(response.get_body_as_stream());

        let mut tool_calls_to_execute = vec![];
        while let Some(next_chunk) = response.get_next_chunk().await.unwrap() {
            match next_chunk {
                OpenAiStreamChunk::Text(text) => {
                    let _ = sender.send(Ok(OpenAiStreamChunk::Text(text))).await;
                }
                OpenAiStreamChunk::ToolCall {
                    id,
                    fn_name,
                    params,
                    result: _,
                } => {
                    tool_calls_to_execute.push(ToolCallModel {
                        id,
                        tp: "function".into(),
                        function: ToolCallFunctionDescription {
                            name: fn_name,
                            arguments: params,
                        },
                    });
                }
            }
        }

        if tool_calls_to_execute.len() == 0 {
            break;
        }

        let tool_call_results =
            super::super::exec_tool_call(&tool_calls_to_execute, &rb, &inner, &ctx).await;

        match tool_call_results {
            Ok(mut results) => {
                for tool_call in tool_calls_to_execute {
                    let result = results.remove(0);
                    let _ = sender
                        .send(Ok(OpenAiStreamChunk::ToolCall {
                            id: tool_call.id,
                            fn_name: tool_call.function.name,
                            params: tool_call.function.arguments,
                            result: result.result_data,
                        }))
                        .await;
                }
            }
            Err(err) => {
                let _ = sender.send(Err(err)).await;
                return;
            }
        }
    }
}
