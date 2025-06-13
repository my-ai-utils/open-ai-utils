pub trait FunctionTypeDescription {
    fn get_type_description(description: &str, default: Option<&str>) -> serde_json::Value;
}

impl FunctionTypeDescription for Option<String> {
    fn get_type_description(description: &str, default: Option<&str>) -> serde_json::Value {
        generate_description_of_opt_parameter("string", description, default)
    }
}

impl FunctionTypeDescription for Option<f64> {
    fn get_type_description(description: &str, default: Option<&str>) -> serde_json::Value {
        generate_description_of_opt_parameter("number", description, default)
    }
}

impl FunctionTypeDescription for Option<f32> {
    fn get_type_description(description: &str, default: Option<&str>) -> serde_json::Value {
        generate_description_of_opt_parameter("number", description, default)
    }
}

impl FunctionTypeDescription for Option<u32> {
    fn get_type_description(description: &str, default: Option<&str>) -> serde_json::Value {
        generate_description_of_opt_parameter("number", description, default)
    }
}

impl FunctionTypeDescription for Option<i32> {
    fn get_type_description(description: &str, default: Option<&str>) -> serde_json::Value {
        generate_description_of_opt_parameter("number", description, default)
    }
}

impl FunctionTypeDescription for Option<u64> {
    fn get_type_description(description: &str, default: Option<&str>) -> serde_json::Value {
        generate_description_of_opt_parameter("number", description, default)
    }
}

impl FunctionTypeDescription for Option<i64> {
    fn get_type_description(description: &str, default: Option<&str>) -> serde_json::Value {
        generate_description_of_opt_parameter("number", description, default)
    }
}

fn generate_description_of_opt_parameter(
    tp: &str,
    description: &str,
    default: Option<&str>,
) -> serde_json::Value {
    if let Some(default) = default {
        serde_json::json! {

           {
                "anyOf": [
                    {
                        "type": tp
                    },
                    {
                        "type": "null"
                    }
                ],
                "default": default,
                "description": description,

            }

        }
    } else {
        serde_json::json! {

           {
                "anyOf": [
                    {
                        "type": tp
                    },
                    {
                        "type": "null"
                    }
                ],
                "default": null,
                "description": description
            }

        }
    }
}
