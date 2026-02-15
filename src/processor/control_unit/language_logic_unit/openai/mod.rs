use reqwest::blocking::Client;

use crate::processor::control_unit::language_logic_unit::openai::{
    chat_completion_models::{OpenAIChatCompletionRequest, OpenAIChatCompletionResponse},
    embeddings_models::{OpenAIEmbeddingsRequest, OpenAIEmbeddingsResponse},
};

pub mod chat_completion_models;
pub mod embeddings_models;
pub mod model_config;

pub struct OpenAIClient {
    base_url: &'static str,
    chat_completion_endpoint: &'static str,
    embeddings_endpoint: &'static str,
}

impl OpenAIClient {
    pub fn new() -> Self {
        return OpenAIClient {
            base_url: "http://127.0.0.1:8080",
            chat_completion_endpoint: "v1/chat/completions",
            embeddings_endpoint: "v1/embeddings",
        };
    }

    pub fn create_chat_completion(
        &self,
        request: OpenAIChatCompletionRequest,
    ) -> Result<OpenAIChatCompletionResponse, String> {
        let client = Client::new();
        let url = format!("{}/{}", self.base_url, self.chat_completion_endpoint);

        let body = match serde_json::to_string(&request) {
            Ok(body) => body,
            Err(error) => {
                return Err(format!(
                    "Failed to serialise chat request to JSON. Error: {}",
                    error
                ));
            }
        };
        let result = client.post(url).body(body).send();
        let response = match result {
            Ok(response) => response,
            Err(error) => return Err(format!("Failed to send chat request. Error: {}", error)),
        };

        if !response.status().is_success() {
            return Err(format!(
                "Chat request failed with status code: {}. Response Text: {}",
                response.status(),
                match response.text() {
                    Ok(text) => text,
                    Err(error) => format!("Failed to read error response text. Error: {}", error),
                }
            ));
        }

        let text = match response.text() {
            Ok(text) => text,
            Err(error) => {
                return Err(format!(
                    "Failed to read chat response text. Error: {}",
                    error
                ));
            }
        };

        return match serde_json::from_str::<OpenAIChatCompletionResponse>(&text) {
            Ok(parsed_response) => Ok(parsed_response),
            Err(error) => Err(format!(
                "Failed to deserialise chat response JSON. Error: {}. Response Text: {}",
                error, text
            )),
        };
    }

    pub fn create_embeddings(
        &self,
        request: OpenAIEmbeddingsRequest,
    ) -> Result<OpenAIEmbeddingsResponse, String> {
        let client = Client::new();
        let url = format!("{}/{}", self.base_url, self.embeddings_endpoint);

        let body = match serde_json::to_string(&request) {
            Ok(body) => body,
            Err(error) => {
                return Err(format!(
                    "Failed to serialise embedding request to JSON. Error: {}",
                    error
                ));
            }
        };
        let result = client.post(url).body(body).send();
        let response = match result {
            Ok(response) => response,
            Err(error) => {
                return Err(format!(
                    "Failed to send embedding request. Error: {}",
                    error
                ));
            }
        };
        let text: String = match response.text() {
            Ok(text) => text,
            Err(error) => {
                return Err(format!(
                    "Failed to read embedding response text. Error: {}",
                    error
                ));
            }
        };

        return match serde_json::from_str::<OpenAIEmbeddingsResponse>(&text) {
            Ok(parsed_response) => Ok(parsed_response),
            Err(error) => {
                return Err(format!(
                    "Failed to deserialise embedding response JSON. Error: {}. Response Text: {}",
                    error, text
                ));
            }
        };
    }
}
