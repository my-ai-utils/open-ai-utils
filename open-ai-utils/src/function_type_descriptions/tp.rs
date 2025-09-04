use my_json::json_writer::JsonObjectWriter;
use rust_extensions::StrOrString;

use crate::*;

#[async_trait::async_trait]
impl<T: GetJsonTypeName> FunctionTypeDescription for T {
    async fn get_type_description(
        description: Option<&str>,
        default: Option<&str>,
        enum_data: Option<Vec<StrOrString<'static>>>,
    ) -> my_json::json_writer::JsonObjectWriter {
        generate_description_of_parameter::<T>(description, default, enum_data)
    }
}

fn generate_description_of_parameter<T: GetJsonTypeName>(
    description: Option<&str>,
    _default: Option<&str>,
    enum_data: Option<Vec<StrOrString<'static>>>,
) -> my_json::json_writer::JsonObjectWriter {
    let tp = T::TYPE_NAME;

    let mut result = JsonObjectWriter::new()
        .write("type", tp)
        .write_if_some("description", description);

    if let Some(enum_data) = enum_data {
        result = result.write_iter("enum", enum_data.iter().map(|itm| itm.as_str()));
    }

    result
}
