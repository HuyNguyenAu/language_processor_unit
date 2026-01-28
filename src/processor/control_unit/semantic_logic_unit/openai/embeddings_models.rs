use serde::{Deserialize, Serialize};

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