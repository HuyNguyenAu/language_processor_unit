use reqwest::blocking::Client;
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

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIEmbeddingsRequest {
    #[serde(rename = "model")]
    pub model: String,
    #[serde(rename = "input")]
    pub input: String,
    #[serde(rename = "encoding_format")]
    pub encoding_format: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIEmbeddingsResponseEmbedding {
    #[serde(rename = "object")]
    pub object: String,
    #[serde(rename = "embedding")]
    pub embedding: Vec<f32>,
    #[serde(rename = "index")]
    pub index: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIEmbeddingsResponse {
    #[serde(rename = "object")]
    pub object: String,
    #[serde(rename = "data")]
    pub data: Vec<OpenAIEmbeddingsResponseEmbedding>,
}

pub struct OpenAIClient {
    base_url: &'static str,
    chat_endpoint: &'static str,
    embeddings_endpoint: &'static str,
}

impl OpenAIClient {
    pub fn new() -> Self {
        return OpenAIClient {
            base_url: "http://127.0.0.1:8080",
            chat_endpoint: "v1/chat/completions",
            embeddings_endpoint: "v1/embeddings",
        };
    }

    pub fn chat(&self, request: OpenAIChatRequest) -> Result<OpenAIChatResponse, String> {
        let client = Client::new();
        let url = format!("{}/{}", self.base_url, self.chat_endpoint);

        let result = client.post(url).json(&request).send();
        let response = match result {
            Ok(response) => response,
            Err(error) => return Err(format!("Failed to send chat request. Error: {}", error)),
        };

        return match response.json::<OpenAIChatResponse>() {
            Ok(parsed_response) => Ok(parsed_response),
            Err(error) => Err(format!("Failed to parse chat response JSON. Error: {}", error)),
        };
    }

    pub fn embeddings(
        &self,
        request: OpenAIEmbeddingsRequest,
    ) -> Result<OpenAIEmbeddingsResponse, String> {
        let client = Client::new();
        let url = format!("{}/{}", self.base_url, self.embeddings_endpoint);

        let result = client.post(url).json(&request).send();
        let response = match result {
            Ok(response) => response,
            Err(error) => return Err(format!("Failed to send embedding request. Error: {}", error)),
        };

        return match response.json::<OpenAIEmbeddingsResponse>() {
            Ok(parsed_response) => Ok(parsed_response),
            Err(error) => Err(format!("Failed to parse embedding response JSON. Error: {}", error)),
        };
    }
}
