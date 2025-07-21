#[derive(Debug, Clone, Default)]
pub struct OtherRequestData {
    pub n: Option<u32>,
    pub presence_penalty: Option<f64>,
    pub frequency_penalty: Option<f64>,
    pub top_p: Option<f64>,
    pub top_k: Option<u32>,
    pub temperature: Option<f64>,
}
