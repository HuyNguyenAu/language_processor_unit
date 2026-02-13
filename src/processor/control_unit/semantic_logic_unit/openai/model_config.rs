pub struct ModelConfig {
    pub model: &'static str,
    pub role: Option<&'static str>,
    pub stream: bool,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub min_p: Option<f32>,
    pub repetition_penalty: Option<f32>,
    pub encoding_format: Option<&'static str>,
}