use rust_extensions::StrOrString;

use crate::*;

#[async_trait::async_trait]
impl<T: GetJsonTypeName> FunctionTypeDescription for T {
    async fn get_type_description(
        description: &str,
        default: Option<&str>,
        enum_data: Option<Vec<StrOrString<'static>>>,
    ) -> serde_json::Value {
        generate_description_of_parameter::<T>(description, default, enum_data)
    }
}

fn generate_description_of_parameter<T: GetJsonTypeName>(
    description: &str,
    _default: Option<&str>,
    enum_data: Option<Vec<StrOrString<'static>>>,
) -> serde_json::Value {
    let tp = T::NAME;
    let Some(enum_data) = enum_data else {
        return serde_json::json! {
            {
                "type": tp,
                "description": description
            }

        };
    };

    let enum_data = enum_data.iter().map(|itm| itm.as_str()).collect::<Vec<_>>();

    return serde_json::json! {
        {
            "type": tp,
            "enum": enum_data,
            "description": description
        }

    };
}
