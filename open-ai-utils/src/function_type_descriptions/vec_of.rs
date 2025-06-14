use crate::{FunctionTypeDescription, GetJsonTypeName};

impl<T: GetJsonTypeName> FunctionTypeDescription for Vec<T> {
    fn get_type_description(
        description: &str,
        default: Option<&str>,
        enum_data: Option<&[&str]>,
    ) -> serde_json::Value {
        generate_description_of_vec_parameter::<T>(description, default, enum_data)
    }
}

fn generate_description_of_vec_parameter<Tp: GetJsonTypeName + FunctionTypeDescription>(
    description: &str,
    default: Option<&str>,
    enum_data: Option<&[&str]>,
) -> serde_json::Value {
    let item_description = Tp::get_type_description(description, default, enum_data);

    if let Some(enum_data) = enum_data {
        return serde_json::json! {
           {
                "type": "array",
                "enum": enum_data,
                "items": item_description,
                "description": description
           }

        };
    };

    return serde_json::json! {

       {
           "type": "array",
            "items": item_description,
            "description": description
        }

    };
}
