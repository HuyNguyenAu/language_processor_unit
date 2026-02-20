use miniserde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIEmbeddingsRequest {
    pub model: String,
    pub input: String,
    pub encoding_format: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIEmbeddingsResponseEmbedding {
    pub object: String,
    pub embedding: Vec<f32>,
    pub index: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIEmbeddingsResponse {
    pub object: String,
    pub data: Vec<OpenAIEmbeddingsResponseEmbedding>,
}
