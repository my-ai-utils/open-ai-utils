#![allow(warnings)]

use open_ai_macros::OpenAiFunctionModel;

#[derive(OpenAiFunctionModel)]
pub struct MyRequestModel {
    #[function_description(name:"filter_showrooms", description: "Filters company location data from a JSON file based on criteria like city (extracted from address), specific service offered (e.g., Sales, Repairs), and geolocation ranges (latitude/longitude).")]
    #[property(description: "city")]
    pub city: Option<String>,
    #[property(description: "service")]
    pub service: Option<String>,
    #[property(description: "Address of dealer")]
    pub addr: Option<String>,
}

#[derive(OpenAiFunctionModel)]
pub struct MyRequestModels {
    #[function_description(name:"filter_showrooms", description: "Filters company location data from a JSON file based on criteria like city (extracted from address), specific service offered (e.g., Sales, Repairs), and geolocation ranges (latitude/longitude).")]
    #[property(description: "city")]
    pub city: Option<String>,
    #[property(description: "city")]
    pub service: Option<String>,
    #[property(description: "city")]
    pub addr: Option<String>,
}
impl MyRequestModel {
    pub fn get_description_() -> serde_json::Value {
        use open_ai_utils::FunctionTypeDescription;
        let mut result = serde_json::Map::new();

        result.insert("type".into(), "function".to_owned().into());

        let mut function = serde_json::Map::new();

        function.insert("name".into(), "filter_showrooms".to_owned().into());

        let mut params = serde_json::Map::new();
        params.insert("type".into(), "object".into());

        let mut properties = serde_json::Map::new();

        properties.insert(
            "city".into(),
            Option::<String>::get_type_description("city", None),
        );

        properties.insert(
            "service".into(),
            Option::<String>::get_type_description("service", None),
        );

        params.insert("properties".into(), properties.into());

        params.insert("required".into(), serde_json::Value::Array(vec![]));
        params.insert("additionalProperties".into(), false.into());

        function.insert("parameters".into(), params.into());

        function.insert("description".into(), "Filters company location data from a JSON file based on criteria like city (extracted from address), specific service offered (e.g., Sales, Repairs), and geolocation ranges (latitude/longitude).".to_owned().into());

        result.insert("function".into(), function.into());

        serde_json::Value::Object(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::MyRequestModel;

    #[test]
    fn test_generation() {
        let description = MyRequestModel::get_description();

        let json = serde_json::to_string_pretty(&description).unwrap();

        println!("{}", json);
    }
}
