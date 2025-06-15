pub trait FunctionToolCallDescription {
    fn get_description() -> serde_json::Value;
}

pub trait FunctionTypeDescription {
    fn get_type_description(
        description: &str,
        default: Option<&str>,
        enum_data: Option<&[&str]>,
    ) -> serde_json::Value;
}

pub trait GetJsonTypeName {
    const NAME: &'static str;
}

impl GetJsonTypeName for u8 {
    const NAME: &'static str = "number";
}

impl GetJsonTypeName for i8 {
    const NAME: &'static str = "number";
}

impl GetJsonTypeName for u16 {
    const NAME: &'static str = "number";
}

impl GetJsonTypeName for i16 {
    const NAME: &'static str = "number";
}

impl GetJsonTypeName for u32 {
    const NAME: &'static str = "number";
}

impl GetJsonTypeName for i32 {
    const NAME: &'static str = "number";
}

impl GetJsonTypeName for u64 {
    const NAME: &'static str = "number";
}

impl GetJsonTypeName for i64 {
    const NAME: &'static str = "number";
}

impl GetJsonTypeName for f64 {
    const NAME: &'static str = "number";
}

impl GetJsonTypeName for f32 {
    const NAME: &'static str = "number";
}

impl GetJsonTypeName for usize {
    const NAME: &'static str = "number";
}

impl GetJsonTypeName for isize {
    const NAME: &'static str = "number";
}

impl GetJsonTypeName for bool {
    const NAME: &'static str = "boolean";
}

impl GetJsonTypeName for String {
    const NAME: &'static str = "string";
}

impl GetJsonTypeName for &'_ str {
    const NAME: &'static str = "string";
}

impl GetJsonTypeName for &'_ String {
    const NAME: &'static str = "string";
}
