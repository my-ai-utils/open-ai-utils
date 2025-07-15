use tokio::sync::RwLock;

use crate::{
    OpenAiRequestBodyBuilder,
    my_auto_gen::{MyAutoGenInner, ToolCallModel, ToolCallsResult},
};

pub async fn exec_tool_call(
    tool_call_models: &[ToolCallModel],
    rb: &OpenAiRequestBodyBuilder,
    inner: &RwLock<MyAutoGenInner>,
    ctx: &str,
) -> Result<Vec<ToolCallsResult>, String> {
    let mut tool_calls_result = Vec::new();
    rb.add_assistant_response_as_tool_calls(tool_call_models)
        .await;
    for tool_call_model in tool_call_models {
        let func_name = tool_call_model.function.name.as_str();

        let call_result = {
            let inner = inner.read().await;
            let result = inner
                .invoke_func(func_name, &tool_call_model.function.arguments, ctx)
                .await?;
            result
        };

        tool_calls_result.push(ToolCallsResult {
            fn_name: tool_call_model.function.name.to_string(),
            request_data: tool_call_model.function.arguments.to_string(),
            result_data: call_result.clone(),
        });

        rb.add_tool_call_response(tool_call_model, call_result)
            .await;
    }
    Ok(tool_calls_result)
}
