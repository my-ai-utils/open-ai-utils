use crate::{FunctionTypeDescription, GetJsonTypeName};

impl<T: GetJsonTypeName> FunctionTypeDescription for Option<T> {
    fn get_type_description(
        description: &str,
        default: Option<&str>,
        enum_data: Option<&[&str]>,
    ) -> serde_json::Value {
        generate_description_of_opt_parameter::<T>(description, default, enum_data)
    }
}

fn generate_description_of_opt_parameter<T: GetJsonTypeName>(
    description: &str,
    default: Option<&str>,
    enum_data: Option<&[&str]>,
) -> serde_json::Value {
    let tp = T::NAME;
    let Some(enum_data) = enum_data else {
        if let Some(default) = default {
            return serde_json::json! {

               {
                    "type": [tp, "null"],
                    "default": default,
                    "description": description,

                }

            };
        } else {
            return serde_json::json! {

               {
                    "type": [tp, "null"],
                    "default": null,
                    "description": description
                }

            };
        }
    };

    if let Some(default) = default {
        return serde_json::json! {

           {
                "type": [tp, "null"],
                "default": default,
                "description": description,

            }

        };
    } else {
        return serde_json::json! {

           {

                "type": [tp, "null"],
                "default": null,
                "description": description
            }

        };
    }
}
