use rust_extensions::StrOrString;

#[derive(Debug, Clone)]
pub enum AutoGenSettings {
    HttpRequest(HttpRequestSettingsModel),
    Mock(Vec<String>),
}

impl AutoGenSettings {
    pub fn unwrap_as_http(&self) -> &HttpRequestSettingsModel {
        match self {
            AutoGenSettings::HttpRequest(model) => model,
            AutoGenSettings::Mock(_) => panic!("Can not use mock AutoGenSettings"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HttpRequestSettingsModel {
    pub url: StrOrString<'static>,
    pub api_key: Option<String>,
    pub do_not_reuse_connection: Option<bool>,
}

impl AutoGenSettings {
    pub fn create_as_open_ai(api_key: Option<String>, do_not_reuse_connection: bool) -> Self {
        let model = HttpRequestSettingsModel {
            url: "https://api.openai.com/v1/chat/completions".into(),
            api_key,
            do_not_reuse_connection: Some(do_not_reuse_connection),
        };

        Self::HttpRequest(model)
    }

    pub fn create_as_nebius(api_key: Option<String>, do_not_reuse_connection: bool) -> Self {
        let model = HttpRequestSettingsModel {
            url: "https://api.studio.nebius.com/v1/chat/completions".into(),
            api_key,
            do_not_reuse_connection: Some(do_not_reuse_connection),
        };

        Self::HttpRequest(model)
    }
}
