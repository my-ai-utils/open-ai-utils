pub trait FunctionTypeDescription {
    fn get_type_description(
        description: &str,
        default: Option<&str>,
        enum_data: Option<&[&str]>,
    ) -> serde_json::Value;
}

impl FunctionTypeDescription for Option<String> {
    fn get_type_description(
        description: &str,
        default: Option<&str>,
        enum_data: Option<&[&str]>,
    ) -> serde_json::Value {
        generate_description_of_opt_parameter("string", description, default, enum_data)
    }
}

impl FunctionTypeDescription for Option<f64> {
    fn get_type_description(
        description: &str,
        default: Option<&str>,
        enum_data: Option<&[&str]>,
    ) -> serde_json::Value {
        generate_description_of_opt_parameter("number", description, default, enum_data)
    }
}

impl FunctionTypeDescription for Option<f32> {
    fn get_type_description(
        description: &str,
        default: Option<&str>,
        enum_data: Option<&[&str]>,
    ) -> serde_json::Value {
        generate_description_of_opt_parameter("number", description, default, enum_data)
    }
}

impl FunctionTypeDescription for Option<u32> {
    fn get_type_description(
        description: &str,
        default: Option<&str>,
        enum_data: Option<&[&str]>,
    ) -> serde_json::Value {
        generate_description_of_opt_parameter("number", description, default, enum_data)
    }
}

impl FunctionTypeDescription for Option<i32> {
    fn get_type_description(
        description: &str,
        default: Option<&str>,
        enum_data: Option<&[&str]>,
    ) -> serde_json::Value {
        generate_description_of_opt_parameter("number", description, default, enum_data)
    }
}

impl FunctionTypeDescription for Option<u64> {
    fn get_type_description(
        description: &str,
        default: Option<&str>,
        enum_data: Option<&[&str]>,
    ) -> serde_json::Value {
        generate_description_of_opt_parameter("number", description, default, enum_data)
    }
}

impl FunctionTypeDescription for Option<i64> {
    fn get_type_description(
        description: &str,
        default: Option<&str>,
        enum_data: Option<&[&str]>,
    ) -> serde_json::Value {
        generate_description_of_opt_parameter("number", description, default, enum_data)
    }
}

fn generate_description_of_opt_parameter(
    tp: &str,
    description: &str,
    default: Option<&str>,
    enum_data: Option<&[&str]>,
) -> serde_json::Value {
    let Some(enum_data) = enum_data else {
        if let Some(default) = default {
            return serde_json::json! {

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

            };
        } else {
            return serde_json::json! {

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

            };
        }
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
