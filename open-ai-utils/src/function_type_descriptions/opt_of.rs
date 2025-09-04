use my_json::json_writer::{JsonNullValue, JsonObjectWriter};
use rust_extensions::StrOrString;

use crate::{FunctionTypeDescription, GetJsonTypeName};

#[async_trait::async_trait]
impl<T: GetJsonTypeName> FunctionTypeDescription for Option<T> {
    async fn get_type_description(
        description: Option<&str>,
        default: Option<&str>,
        enum_data: Option<Vec<StrOrString<'static>>>,
    ) -> my_json::json_writer::JsonObjectWriter {
        generate_description_of_opt_parameter::<T>(description, default, enum_data.as_deref())
    }
}

fn generate_description_of_opt_parameter<T: GetJsonTypeName>(
    description: Option<&str>,
    default: Option<&str>,
    enum_data: Option<&[StrOrString<'static>]>,
) -> my_json::json_writer::JsonObjectWriter {
    let tp = T::TYPE_NAME;

    let mut result = JsonObjectWriter::new()
        .write("type", tp)
        .write_if_some("description", description);

    if let Some(enum_data) = enum_data {
        result = result.write_iter("enum", enum_data.iter().map(|itm| itm.as_str()));
    };

    if let Some(default) = default {
        result = result.write("default", default);
    } else {
        result = result.write("default", JsonNullValue);
    }

    result
}
