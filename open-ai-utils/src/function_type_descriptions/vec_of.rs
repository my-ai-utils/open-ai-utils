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
    let tp = Tp::NAME;
    let item_description = Tp::get_type_description(description, default, enum_data);

    let Some(enum_data) = enum_data else {
        return serde_json::json! {
           {
                "type": "array",
                "items": item_description
           }

        };
    };

    if let Some(default) = default {
        return serde_json::json! {

           {
                "anyOf": [
                    {
                        "enum": enum_data,
                        "type": tp
                    },
                    {
                        "type": "null"
                    }
                ],
                "default": default,
                "description": description,

            }

        };
    } else {
        return serde_json::json! {

           {
                "anyOf": [
                    {
                        "enum": enum_data,
                        "type": tp
                    },
                    {
                        "type": "null"
                    }
                ],
                "default": null,
                "description": description
            }

        };
    }
}
