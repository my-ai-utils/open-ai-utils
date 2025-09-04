use my_json::json_writer::JsonObjectWriter;
use rust_extensions::StrOrString;

use crate::{FunctionTypeDescription, GetJsonTypeName};

#[async_trait::async_trait]
impl<T: GetJsonTypeName> FunctionTypeDescription for Option<Vec<T>> {
    async fn get_type_description(
        description: Option<&str>,
        default: Option<&str>,
        enum_data: Option<Vec<StrOrString<'static>>>,
    ) -> my_json::json_writer::JsonObjectWriter {
        generate_description_of_opt_of_vec_parameter::<T>(description, default, enum_data).await
    }
}

async fn generate_description_of_opt_of_vec_parameter<Tp: GetJsonTypeName>(
    description: Option<&str>,
    default: Option<&str>,
    enum_data: Option<Vec<StrOrString<'static>>>,
) -> my_json::json_writer::JsonObjectWriter {
    let result = JsonObjectWriter::new()
        .write("type", "array")
        .write_if_some("description", description)
        .write("default", default);

    super::vec_of::fill_array_sub_elements(result, Tp::TYPE_NAME, &enum_data)
        .write("uniqueItems", true)
}
