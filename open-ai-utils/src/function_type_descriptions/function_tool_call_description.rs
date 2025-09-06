#[async_trait::async_trait]
pub trait FunctionToolCallDescription {
    async fn get_description() -> my_json::json_writer::JsonObjectWriter;
}
