use rust_extensions::StrOrString;

#[async_trait::async_trait]
pub trait FunctionTypeDescription {
    async fn get_type_description(
        description: Option<&str>,
        default: Option<&str>,
        enum_data: Option<Vec<StrOrString<'static>>>,
    ) -> my_json::json_writer::JsonObjectWriter;
}

pub trait GetJsonTypeName {
    const TYPE_NAME: &'static str;
    const OPTIONAL: bool;
}

impl GetJsonTypeName for u8 {
    const TYPE_NAME: &'static str = "number";
    const OPTIONAL: bool = false;
}

impl GetJsonTypeName for i8 {
    const TYPE_NAME: &'static str = "number";
    const OPTIONAL: bool = false;
}

impl GetJsonTypeName for u16 {
    const TYPE_NAME: &'static str = "number";
    const OPTIONAL: bool = false;
}

impl GetJsonTypeName for i16 {
    const TYPE_NAME: &'static str = "number";
    const OPTIONAL: bool = false;
}

impl GetJsonTypeName for u32 {
    const TYPE_NAME: &'static str = "number";
    const OPTIONAL: bool = false;
}

impl GetJsonTypeName for i32 {
    const TYPE_NAME: &'static str = "number";
    const OPTIONAL: bool = false;
}

impl GetJsonTypeName for u64 {
    const TYPE_NAME: &'static str = "number";
    const OPTIONAL: bool = false;
}

impl GetJsonTypeName for i64 {
    const TYPE_NAME: &'static str = "number";
    const OPTIONAL: bool = false;
}

impl GetJsonTypeName for f64 {
    const TYPE_NAME: &'static str = "number";
    const OPTIONAL: bool = false;
}

impl GetJsonTypeName for f32 {
    const TYPE_NAME: &'static str = "number";
    const OPTIONAL: bool = false;
}

impl GetJsonTypeName for usize {
    const TYPE_NAME: &'static str = "number";
    const OPTIONAL: bool = false;
}

impl GetJsonTypeName for isize {
    const TYPE_NAME: &'static str = "number";
    const OPTIONAL: bool = false;
}

impl GetJsonTypeName for bool {
    const TYPE_NAME: &'static str = "boolean";
    const OPTIONAL: bool = false;
}

impl GetJsonTypeName for String {
    const TYPE_NAME: &'static str = "string";
    const OPTIONAL: bool = false;
}

impl GetJsonTypeName for &'_ str {
    const TYPE_NAME: &'static str = "string";
    const OPTIONAL: bool = false;
}

impl GetJsonTypeName for &'_ String {
    const TYPE_NAME: &'static str = "string";
    const OPTIONAL: bool = false;
}
