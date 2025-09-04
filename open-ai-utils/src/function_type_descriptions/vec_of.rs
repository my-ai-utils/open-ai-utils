use my_json::json_writer::JsonObjectWriter;
use rust_extensions::StrOrString;

use crate::{FunctionTypeDescription, GetJsonTypeName};

#[async_trait::async_trait]
impl<T: GetJsonTypeName> FunctionTypeDescription for Vec<T> {
    async fn get_type_description(
        description: Option<&str>,
        default: Option<&str>,
        enum_data: Option<Vec<StrOrString<'static>>>,
    ) -> JsonObjectWriter {
        generate_description_of_vec_parameter::<T>(description, default, enum_data).await
    }
}

async fn generate_description_of_vec_parameter<Tp: GetJsonTypeName + FunctionTypeDescription>(
    description: Option<&str>,
    default: Option<&str>,
    enum_data: Option<Vec<StrOrString<'static>>>,
) -> JsonObjectWriter {
    let mut result = JsonObjectWriter::new()
        .write("type", "array")
        .write_if_some("description", description)
        .write_if_some("default", default);

    result = fill_array_sub_elements(result, Tp::TYPE_NAME, &enum_data);

    if enum_data.is_some() {
        result = result.write("uniqueItems", true);
    }

    result
}

pub fn fill_array_sub_elements(
    writer: JsonObjectWriter,
    tp_name: &str,
    enum_data: &Option<Vec<StrOrString<'static>>>,
) -> JsonObjectWriter {
    writer.write_json_object("items", move |items| {
        let mut items = items.write("type", tp_name);
        if let Some(enum_data) = enum_data {
            let enums = enum_data.iter().map(|itm| itm.as_str());
            items = items.write_iter("enum", enums);
        }

        items
    })
}
