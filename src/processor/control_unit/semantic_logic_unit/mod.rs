use base64::{Engine, prelude::BASE64_STANDARD};

use crate::processor::control_unit::semantic_logic_unit::{
    microcode::Microcode,
    openai::{
        OpenAIClient,
        audio_speech::OpenAIAudioSpeechRequest,
        chat_completion_models::{OpenAIChatCompletionRequest, OpenAIChatCompletionRequestText},
        embeddings_models::OpenAIEmbeddingsRequest,
    },
    value::Value,
};

mod microcode;
mod openai;
pub mod value;

pub struct SemanticLogicUnit {
    micro_code: Microcode,
    openai_client: OpenAIClient,
    model: &'static str,
    role: &'static str,
    stream: bool,
    temperature: f32,
    min_probability: f32,
    repetition_penalty: f32,
    encoding_format: &'static str,
}

impl SemanticLogicUnit {
    pub fn new() -> Self {
        return SemanticLogicUnit {
            micro_code: Microcode::new(),
            openai_client: OpenAIClient::new(),
            model: "LFM2-2.6B-Q5_K_M.gguf",
            role: "user",
            stream: false,
            temperature: 0.3,
            min_probability: 0.15,
            repetition_penalty: 1.05,
            encoding_format: "float",
        };
    }

    fn clean_string(&self, value: &str) -> String {
        return value.trim().replace("\n", "").to_string();
    }

    fn chat(&self, content: &str) -> Result<String, String> {
        let request = OpenAIChatCompletionRequest {
            model: self.model.to_string(),
            stream: self.stream,
            messages: vec![OpenAIChatCompletionRequestText {
                role: self.role.to_string(),
                content: content.to_string(),
            }],
            temperature: self.temperature,
            top_p: self.min_probability,
            presence_penalty: self.repetition_penalty,
        };

        let response = &self.openai_client.create_chat_completion(request);

        let choice = match response {
            Ok(response) => response.choices.iter().nth(0),
            Err(error) => {
                return Err(format!(
                    "Failed to get chat response from client. Error: {}",
                    error
                ));
            }
        };

        return match choice {
            Some(choice) => Ok(self.clean_string(&choice.message.content)),
            None => Err("No choices returned from client.".to_string()),
        };
    }

    fn embeddings(&self, content: &str) -> Result<Vec<f32>, String> {
        let request = OpenAIEmbeddingsRequest {
            model: self.model.to_string(),
            input: content.to_string(),
            encoding_format: self.encoding_format.to_string(),
        };

        let response = &self.openai_client.create_embeddings(request);

        let embeddings = match response {
            Ok(response) => response.data.iter().nth(0),
            Err(error) => {
                return Err(format!(
                    "Failed to get embeddings response from client. Error: {}",
                    error
                ));
            }
        };

        return match embeddings {
            Some(value) => Ok(value.embedding.clone()),
            None => Err("No embeddings returned from client.".to_string()),
        };
    }

    fn speech(&self, content: &str) -> Result<Vec<u8>, String> {
        let request = OpenAIAudioSpeechRequest {
            input: content.to_string(),
            model: self.model.to_string(),
            voice: "alba".to_string(),
        };

        let response = &self.openai_client.create_speech(request);

        let speech_response = match &response {
            Ok(response) => response,
            Err(error) => {
                return Err(format!(
                    "Failed to get speech response from client. Error: {}",
                    error
                ));
            }
        };
        let audio_bytes = match Engine::decode(&BASE64_STANDARD, &speech_response.audio) {
            Ok(bytes) => bytes,
            Err(error) => {
                return Err(format!(
                    "Failed to decode audio data from client. Error: {}",
                    error
                ));
            }
        };

        return Ok(audio_bytes);
    }

    pub fn addition(&self, value_a: &Value, value_b: &Value) -> String {
        let value_a = match value_a {
            Value::Text(text) => text,
            _ => panic!("Addition requires text value."),
        };
        let value_b = match value_b {
            Value::Text(text) => text,
            _ => panic!("Addition requires text value."),
        };

        let content = self.micro_code.addition(value_a, value_b);

        return match &self.chat(content.as_str()) {
            Ok(choice) => choice.to_lowercase(),
            Err(error) => panic!("Failed to perform addition. Error: {}", error),
        };
    }

    pub fn subtract(&self, value_a: &Value, value_b: &Value) -> String {
        let value_a = match value_a {
            Value::Text(text) => text,
            _ => panic!("Subtraction requires text value."),
        };
        let value_b = match value_b {
            Value::Text(text) => text,
            _ => panic!("Subtraction requires text value."),
        };

        let content = self.micro_code.subtract(value_a, value_b);

        return match &self.chat(content.as_str()) {
            Ok(choice) => choice.to_lowercase(),
            Err(error) => panic!("Failed to perform subtraction. Error: {}", error),
        };
    }

    pub fn similarity(&self, value_a: &Value, value_b: &Value) -> u8 {
        let value_a = match value_a {
            Value::Text(text) => text,
            _ => panic!("Similarity requires text value."),
        };
        let value_b = match value_b {
            Value::Text(text) => text,
            _ => panic!("Similarity requires text value."),
        };

        let first_embedding_result = self.embeddings(value_a);
        let first_embedding = match &first_embedding_result {
            Ok(embedding) => embedding,
            Err(error) => panic!("Failed to get first embedding. Error: {}", error),
        };

        let second_embedding_result = self.embeddings(value_b);
        let second_embedding = match &second_embedding_result {
            Ok(embedding) => embedding,
            Err(error) => panic!("Failed to get second embedding. Error: {}", error),
        };

        // Compute cosine similarity.
        let dot_product: f32 = first_embedding
            .iter()
            .zip(second_embedding.iter())
            .map(|(a, b)| a * b)
            .sum();
        let x_euclidean_length: f32 = first_embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        let y_euclidean_length: f32 = second_embedding.iter().map(|y| y * y).sum::<f32>().sqrt();
        let similarity = dot_product / (x_euclidean_length * y_euclidean_length);
        let percentage_similarity = similarity.clamp(0.0, 1.0) * 100.0;

        return (percentage_similarity.round()) as u8;
    }

    pub fn text_to_speech(&self, value_a: &Value) -> Vec<u8> {
        let text = match value_a {
            Value::Text(text) => text,
            _ => panic!("Text to speech requires a text value."),
        };

        let speech_result = self.speech(text);

        return match &speech_result {
            Ok(audio_bytes) => audio_bytes.clone(),
            Err(error) => panic!("Failed to perform text to speech. Error: {}", error),
        };
    }
}
