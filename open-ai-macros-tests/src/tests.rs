#![allow(warnings)]

use open_ai_macros::OpenAiFunctionModel;
use rust_extensions::StrOrString;

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

    #[property(enum: "get_other_condition_enum", description: "Vehicle condition (NEW/CPO). Defaults to None")]
    pub other_condition: String,

    #[property(description: "Minimal condition")]
    pub min_condition: Option<i64>,
}

async fn get_other_condition_enum() -> Option<Vec<StrOrString<'static>>> {
    None
}
#[cfg(test)]
mod tests {
    use open_ai_utils::{
        FunctionDescriptionJsonModel, FunctionToolCallDescription, OpenAiRequestBodyBuilder,
    };

    use crate::tests::MyRequestModel;

    #[tokio::test]
    async fn test_generation() {
        use open_ai_utils::FunctionToolCallDescription;
        let description = MyRequestModel::get_description().await;

        let json = serde_json::to_string_pretty(&description).unwrap();

        println!("{}", json);
    }

    #[tokio::test]
    async fn test_builder_and_model() {
        let mut builder = OpenAiRequestBodyBuilder::new_with_system_prompt(
            "test_system_prompt_data",
            open_ai_utils::LlmModel::Gpt4o,
        );

        let func_json_description = FunctionDescriptionJsonModel {
            name: "filter_showrooms".to_string(),
            description: "Filters company location data from a JSON file based on criteria like city (extracted from address), specific service offered (e.g., Sales, Repairs), and geolocation ranges (latitude/longitude).".to_string(),
            parameters: MyRequestModel::get_description().await,
            strict: None,
        };

        let func_json_description = serde_json::to_value(&func_json_description).unwrap();

        builder
            .add_tools_call_description(func_json_description)
            .await;

        let model = builder.get_model(&Default::default()).await;

        let json_str = serde_json::to_string_pretty(&model).unwrap();

        println!("{}", json_str);
    }
}
