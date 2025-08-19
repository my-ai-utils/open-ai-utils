#[derive(Debug, Clone, Default)]
pub struct OtherRequestData {
    pub n: Option<i64>,
    pub presence_penalty: Option<f64>,
    pub frequency_penalty: Option<f64>,
    pub top_p: Option<f64>,
    pub top_k: Option<i64>,
    pub temperature: Option<f64>,
    pub think: bool, // Uses for qwen3
}
