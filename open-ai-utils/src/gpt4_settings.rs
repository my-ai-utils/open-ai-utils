#[derive(Debug, Clone, Default, Copy)]
pub struct Gpt4Settings {
    pub n: Option<i64>,
    pub presence_penalty: Option<f64>,
    pub frequency_penalty: Option<f64>,
    pub top_p: Option<f64>,

    pub temperature: Option<f64>,
    pub think: bool, // Uses for qwen3
}
