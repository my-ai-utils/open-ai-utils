#![allow(warnings)]

use open_ai_macros::OpenAiFunctionModel;

#[derive(OpenAiFunctionModel)]
pub struct MyRequestModel {
    #[function_description(name:"filter_showrooms", description: "Filters company location data from a JSON file based on criteria like city (extracted from address), specific service offered (e.g., Sales, Repairs), and geolocation ranges (latitude/longitude).")]
    #[property(description: "city")]
    pub city: String,
    #[property(description: "service")]
    pub service: Option<String>,
    #[property(description: "Address of dealer")]
    pub addr: Option<String>,
    #[property(enum:["NEW", "CPO"], description: "Vehicle condition (NEW/CPO). Defaults to None")]
    pub condition: Option<String>,

    #[property(enum:["NEW", "CPO"], description: "Vehicle condition (NEW/CPO). Defaults to None")]
    pub other_condition: String,

    #[property(description: "Minimal condition")]
    pub min_condition: Option<i64>,
}

impl MyRequestModel {
    fn get_test_description() -> serde_json::Value {
        use open_ai_utils::FunctionTypeDescription;
        let mut params = serde_json::Map::new();
        params.insert("type".into(), "object".into());
        let mut properties = serde_json::Map::new();
        properties.insert(
            "city".into(),
            String::get_type_description("city", None, None),
        );
        properties.insert(
            "service".into(),
            Option::<String>::get_type_description("service", None, None),
        );
        properties.insert(
            "addr".into(),
            Option::<String>::get_type_description("Address of dealer", None, None),
        );
        properties.insert(
            "condition".into(),
            Option::<String>::get_type_description(
                "Vehicle condition (NEW/CPO). Defaults to None",
                None,
                Some(&["NEW", "CPO"]),
            ),
        );
        properties.insert(
            "other_condition".into(),
            String::get_type_description(
                "Vehicle condition (NEW/CPO). Defaults to None",
                None,
                Some(&["NEW", "CPO"]),
            ),
        );
        properties.insert(
            "min_condition".into(),
            Option::<i64>::get_type_description("Minimal condition", None, None),
        );
        params.insert("properties".into(), properties.into());
        params.insert("required".into(), serde_json::Value::Array(vec![]));
        params.insert("additionalProperties".into(), false.into());

        serde_json::Value::Object(params)
    }
}

#[cfg(test)]
mod tests {
    use open_ai_utils::{
        FunctionDescriptionJsonModel, FunctionToolCallDescription, OpenAiRequestBodyBuilder,
    };

    use crate::tests::MyRequestModel;

    #[test]
    fn test_generation_() {
        use open_ai_utils::FunctionToolCallDescription;
        let description = MyRequestModel::get_test_description();

        let json = serde_json::to_string_pretty(&description).unwrap();

        println!("{}", json);
    }

    #[test]
    fn test_generation() {
        use open_ai_utils::FunctionToolCallDescription;
        let description = MyRequestModel::get_description();

        let json = serde_json::to_string_pretty(&description).unwrap();

        println!("{}", json);
    }
    #[test]
    fn test_builder_and_model() {
        let mut builder = OpenAiRequestBodyBuilder::new(
            "test_system_prompt_data",
            open_ai_utils::LlmModel::Gpt4o,
        );

        let func_json_description = FunctionDescriptionJsonModel {
            name: "filter_showrooms".to_string(),
            description: "Filters company location data from a JSON file based on criteria like city (extracted from address), specific service offered (e.g., Sales, Repairs), and geolocation ranges (latitude/longitude).".to_string(),
            parameters: MyRequestModel::get_description(),
        };

        builder.add_tool_calls(func_json_description);

        let json_str = serde_json::to_string_pretty(builder.get_model()).unwrap();

        println!("{}", json_str);
    }
}
