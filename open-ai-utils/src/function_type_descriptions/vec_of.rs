use rust_extensions::StrOrString;

use crate::{FunctionTypeDescription, GetJsonTypeName};

#[async_trait::async_trait]
impl<T: GetJsonTypeName> FunctionTypeDescription for Vec<T> {
    async fn get_type_description(
        description: &str,
        default: Option<&str>,
        enum_data: Option<Vec<StrOrString<'static>>>,
    ) -> serde_json::Value {
        generate_description_of_vec_parameter::<T>(description, default, enum_data).await
    }
}

async fn generate_description_of_vec_parameter<Tp: GetJsonTypeName + FunctionTypeDescription>(
    description: &str,
    default: Option<&str>,
    enum_data: Option<Vec<StrOrString<'static>>>,
) -> serde_json::Value {
    let item_description = Tp::get_type_description(description, default, enum_data.clone()).await;

    /*
       if let Some(enum_data) = enum_data.as_ref() {
           let enum_data: Vec<_> = enum_data.iter().map(|itm| itm.as_str()).collect();
           return serde_json::json! {
              {
                   "type": "array",

                   "items": item_description,
                   "description": description
              }

           };
       };
    */

    return serde_json::json! {

       {
           "type": "array",
            "items": item_description,
            "description": description
        }

    };
}
