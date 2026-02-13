use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIChatCompletionRequestText {
    #[serde(rename = "role")]
    pub role: String,
    #[serde(rename = "content")]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIChatCompletionRequest {
    #[serde(rename = "model")]
    pub model: String,
    #[serde(rename = "stream")]
    pub stream: bool,
    #[serde(rename = "messages")]
    pub messages: Vec<OpenAIChatCompletionRequestText>,
    #[serde(rename = "temperature")]
    pub temperature: Option<f32>,
    #[serde(rename = "top_p")]
    pub top_p: Option<f32>,
    #[serde(rename = "min_p")]
    pub min_p: Option<f32>,
    #[serde(rename = "frequency_penalty")]
    pub presence_penalty: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIChatCompletionResponseMessage {
    #[serde(rename = "role")]
    pub role: String,
    #[serde(rename = "content")]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIChatCompletionResponseChoice {
    #[serde(rename = "index")]
    pub index: u8,
    #[serde(rename = "message")]
    pub message: OpenAIChatCompletionResponseMessage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIChatCompletionResponse {
    #[serde(rename = "model")]
    pub model: String,
    #[serde(rename = "choices")]
    pub choices: Vec<OpenAIChatCompletionResponseChoice>,
}
