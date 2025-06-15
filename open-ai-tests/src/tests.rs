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

#[cfg(test)]
mod tests {
    use open_ai_utils::OpenAiRequestBodyBuilder;

    use crate::tests::MyRequestModel;

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

        builder.add_tool_calls::<MyRequestModel>();

        let json_str = serde_json::to_string_pretty(builder.get_model()).unwrap();

        println!("{}", json_str);
    }
}
