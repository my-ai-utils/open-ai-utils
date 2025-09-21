use std::{collections::HashMap, sync::Arc, time::Duration};

use flurl::{FlUrl, FlUrlResponse, body::FlUrlBody};
use rust_extensions::{Logger, date_time::DateTimeAsMicroseconds};
use tokio::sync::{Mutex, RwLock};

use crate::{
    OpenAiRequestBodyBuilder, OtherRequestData, ToolCallFunctionDescription, my_auto_gen::*,
};

pub async fn execute_request_as_stream(
    settings: AutoGenSettings,
    sender: tokio::sync::mpsc::Sender<Result<OpenAiStreamChunk, String>>,
    rb: Arc<OpenAiRequestBodyBuilder>,
    inner: Arc<RwLock<MyAutoGenInner>>,
    ctx: String,
    other_request_data: OtherRequestData,
    logger: Arc<dyn Logger + Send + Sync>,
) {
    let mut text_result = String::new();

    let mut mock_items = None;

    loop {
        let mut had_fn_called = false;
        let mut response = match &settings {
            AutoGenSettings::HttpRequest(settings_model) => {
                match prepare_open_ai_fl_url_streamed_request(
                    &settings_model,
                    &rb,
                    &logger,
                    &other_request_data,
                )
                .await
                {
                    Ok(response) => response,
                    Err(err) => {
                        let _ = sender.send(Err(err)).await;
                        return;
                    }
                }
            }
            AutoGenSettings::Mock(items) => {
                if mock_items.is_none() {
                    mock_items = Some(Arc::new(Mutex::new(items.clone())));
                }
                OpenAiInnerResponseStream::new(OpenAiNetworkStream::Mock(
                    mock_items.clone().unwrap(),
                ))
            }
        };

        while let Some(next_chunk) = response.get_next_chunk(&rb).await.unwrap() {
            match next_chunk {
                OpenAiStreamHttpChunk::Text(text) => {
                    text_result.push_str(&text);
                    let _ = sender.send(Ok(OpenAiStreamChunk::Text(text))).await;
                }
                OpenAiStreamHttpChunk::ToolCalls(tool_calls) => {
                    had_fn_called = true;
                    if text_result.len() > 0 {
                        rb.add_assistant_message(std::mem::take(&mut text_result))
                            .await;
                    }

                    let mut tool_calls_to_execute = vec![];

                    for tool_call in tool_calls {
                        tool_calls_to_execute.push(ToolCallModel {
                            id: tool_call.id,
                            tp: "function".into(),
                            function: ToolCallFunctionDescription {
                                name: tool_call.fn_name,
                                arguments: tool_call.params,
                            },
                        });
                    }

                    let tool_call_results =
                        super::super::exec_tool_call(&tool_calls_to_execute, &rb, &inner, &ctx)
                            .await;

                    match tool_call_results {
                        Ok(mut results) => {
                            let mut to_send = Vec::with_capacity(tool_calls_to_execute.len());
                            for tool_call in tool_calls_to_execute {
                                let result = results.remove(0);

                                to_send.push(ToolCallChunkModel {
                                    id: tool_call.id,
                                    fn_name: tool_call.function.name,
                                    params: tool_call.function.arguments,
                                    result: result.result_data,
                                });
                            }

                            let _ = sender.send(Ok(OpenAiStreamChunk::ToolCalls(to_send))).await;
                        }
                        Err(err) => {
                            let ctx = match &settings {
                                AutoGenSettings::HttpRequest(settings_model) => {
                                    create_logs_context(&settings_model, None)
                                }
                                AutoGenSettings::Mock(_) => None,
                            };
                            logger.write_error(
                                "execute_open_ai_request_as_stream".to_string(),
                                err.to_string(),
                                ctx,
                            );
                            let _ = sender.send(Err(err)).await;
                            return;
                        }
                    }
                }
            }
        }

        if text_result.len() > 0 {
            rb.add_assistant_message(std::mem::take(&mut text_result))
                .await;
        }

        if !had_fn_called {
            break;
        }
    }
}

async fn prepare_open_ai_fl_url_streamed_request(
    settings: &HttpRequestSettingsModel,
    rb: &OpenAiRequestBodyBuilder,
    logger: &Arc<dyn Logger + Send + Sync>,
    other_request_data: &OtherRequestData,
) -> Result<OpenAiInnerResponseStream, String> {
    let response = prepare_open_ai_fl_url(settings, rb, logger, other_request_data).await?;
    let status_code = response.get_status_code();

    if status_code != 200 {
        let body = response.receive_body().await.unwrap();
        println!("OpenAI status code: {}", status_code);
        println!("{:?}", std::str::from_utf8(body.as_slice()));

        let body_str = std::str::from_utf8(body.as_slice()).unwrap_or("Body is not UTF-8");

        let err = format!("Status code: {}. Body: {}", status_code, body_str);
        logger.write_error(
            "execute_open_ai_request_as_stream".to_string(),
            err.to_string(),
            create_logs_context(&settings, None),
        );
        return Err(err);
    }

    let response =
        OpenAiInnerResponseStream::new(OpenAiNetworkStream::Http(response.get_body_as_stream()));

    Ok(response)
}

async fn prepare_open_ai_fl_url(
    settings: &HttpRequestSettingsModel,
    rb: &OpenAiRequestBodyBuilder,
    logger: &Arc<dyn Logger + Send + Sync>,
    other_request_data: &OtherRequestData,
) -> Result<FlUrlResponse, String> {
    let mut attempt = 0;
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
                rb.get_model(other_request_data)
            })
            .await;

        println!("{}", serde_json::to_string_pretty(&model).unwrap());

        rb.write_tech_log(TechRequestLogItem::new_data_as_str(
            DateTimeAsMicroseconds::now(),
            TechLogItemType::Request,
            serde_json::to_string(&model).unwrap(),
        ))
        .await;
        let response = fl_url.post(FlUrlBody::as_json(&model)).await;

        match response {
            Ok(response) => return Ok(response),
            Err(err) => {
                let msg = format!("{:?}", err);

                logger.write_debug_info(
                    "prepare_open_ai_streamed_request".into(),
                    msg.clone(),
                    create_logs_context(settings, Some(attempt)),
                );
                if attempt >= 3 {
                    return Err(format!("{:?}", err));
                }

                attempt += 1;
            }
        };
    }
}

fn create_logs_context(
    settings: &HttpRequestSettingsModel,
    attempt: Option<usize>,
) -> Option<HashMap<String, String>> {
    let mut ctx = HashMap::new();

    if let Some(attempt) = attempt {
        ctx.insert("attempt".to_string(), attempt.to_string());
    }

    ctx.insert("url".to_string(), settings.url.as_str().to_string());
    Some(ctx)
}
