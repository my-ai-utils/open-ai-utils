#![allow(warnings)]

use open_ai_macros::OpenAiFunctionModel;
use open_ai_utils::my_json::json_writer::EmptyJsonArray;
use rust_extensions::StrOrString;

#[derive(OpenAiFunctionModel)]
pub struct MyRequestModelNoRequired {
    #[property(description: "city description")]
    pub city: Option<String>,
    #[property(description: "service")]
    pub service: Option<String>,
}

#[cfg(test)]
mod tests {
    use open_ai_utils::{my_json, FunctionToolCallDescription};

    use crate::model_with_no_required::MyRequestModelNoRequired;

    #[tokio::test]
    async fn test_generation() {
        let description = MyRequestModelNoRequired::get_description().await.build();

        let result = my_json::j_path::j_path(description.as_bytes(), "type")
            .unwrap()
            .unwrap();

        assert_eq!(result.as_str().unwrap().as_str(), "object");

        let result = my_json::j_path::j_path(description.as_bytes(), "properties.city.type")
            .unwrap()
            .unwrap();

        assert_eq!(result.as_str().unwrap().as_str(), "string");

        let result = my_json::j_path::j_path(description.as_bytes(), "properties.city.description")
            .unwrap()
            .unwrap();

        assert_eq!(result.as_str().unwrap().as_str(), "city description");

        let result = my_json::j_path::j_path(description.as_bytes(), "properties.city.default")
            .unwrap()
            .unwrap();

        assert!(result.as_str().is_none());

        assert_eq!(description, "{\"type\":\"object\",\"properties\":{\"city\":{\"type\":\"string\",\"description\":\"city description\",\"default\":null},\"service\":{\"type\":\"string\",\"description\":\"service\",\"default\":null}},\"required\":[],\"additionalProperties\":false}");
    }
}
