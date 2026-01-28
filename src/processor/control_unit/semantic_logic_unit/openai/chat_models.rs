use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIChatRequestText {
    #[serde(rename = "role")]
    pub role: String,
    #[serde(rename = "content")]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIChatRequest {
    #[serde(rename = "model")]
    pub model: String,
    #[serde(rename = "stream")]
    pub stream: bool,
    #[serde(rename = "messages")]
    pub messages: Vec<OpenAIChatRequestText>,
    #[serde(rename = "temperature")]
    pub temperature: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIChatResponseMessage {
    #[serde(rename = "role")]
    pub role: String,
    #[serde(rename = "content")]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIChatResponseChoice {
    #[serde(rename = "index")]
    pub index: u8,
    #[serde(rename = "message")]
    pub message: OpenAIChatResponseMessage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIChatResponse {
    #[serde(rename = "model")]
    pub model: String,
    #[serde(rename = "choices")]
    pub choices: Vec<OpenAIChatResponseChoice>,
}