#![allow(warnings)]

use open_ai_macros::OpenAiFunctionModel;
use open_ai_utils::my_json::json_writer::EmptyJsonArray;
use rust_extensions::StrOrString;

#[derive(OpenAiFunctionModel)]
pub struct MyRequestModelNoRequired {
    #[function_description(name:"filter_showrooms", description: "Filters company location data from a JSON file based on criteria like city (extracted from address), specific service offered (e.g., Sales, Repairs), and geolocation ranges (latitude/longitude).")]
    #[property(description: "city")]
    pub city: Option<String>,
    #[property(description: "service")]
    pub service: Option<String>,
}

#[cfg(test)]
mod tests {
    use open_ai_utils::FunctionToolCallDescription;

    use crate::model_with_no_required::MyRequestModelNoRequired;

    #[tokio::test]
    async fn test_generation() {
        let description = MyRequestModelNoRequired::get_description().await.build();

        assert_eq!(description, "{\"type\":\"object\",\"properties\":{\"city\":{\"type\":\"string\",\"description\":\"city\",\"default\":null},\"service\":{\"type\":\"string\",\"description\":\"service\",\"default\":null}},\"required\":[],\"additionalProperties\":false}");
    }
}
