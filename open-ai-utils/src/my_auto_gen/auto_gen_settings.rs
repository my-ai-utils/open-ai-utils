use rust_extensions::StrOrString;

#[derive(Debug, Clone)]
pub struct AutoGenSettings {
    pub url: StrOrString<'static>,
    pub api_key: Option<String>,
    pub do_not_reuse_connection: Option<bool>,
}

impl AutoGenSettings {
    pub fn create_as_open_ai(api_key: Option<String>, do_not_reuse_connection: bool) -> Self {
        Self {
            url: "https://api.openai.com/v1/chat/completions".into(),
            api_key,
            do_not_reuse_connection: Some(do_not_reuse_connection),
        }
    }
}
