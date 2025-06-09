use serde::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAiToolsModel {
    #[serde(rename = "type")]
    pub r#type: String,

    pub function: OpenAiFunctionDescriptionModel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAiFunctionDescriptionModel {
    pub name: String,
}
