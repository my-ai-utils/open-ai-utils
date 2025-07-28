use rust_extensions::StrOrString;

use crate::{FunctionTypeDescription, GetJsonTypeName};

#[async_trait::async_trait]
impl<T: GetJsonTypeName> FunctionTypeDescription for Option<T> {
    async fn get_type_description(
        description: &str,
        default: Option<&str>,
        enum_data: Option<Vec<StrOrString<'static>>>,
    ) -> serde_json::Value {
        generate_description_of_opt_parameter::<T>(description, default, enum_data.as_deref())
    }
}

fn generate_description_of_opt_parameter<T: GetJsonTypeName>(
    description: &str,
    default: Option<&str>,
    enum_data: Option<&[StrOrString<'static>]>,
) -> serde_json::Value {
    let tp = T::NAME;
    if let Some(enum_data) = enum_data {
        let enum_data: Vec<_> = enum_data.iter().map(|itm| itm.as_str()).collect();
        return serde_json::json! {
           {
                "type": tp,
                "default": default,
                "enum": enum_data,
                "description": description
            }
        };
    };

    return serde_json::json! {
       {
             "type": tp,
             "default": default,
             "description": description
       }

    };
}
