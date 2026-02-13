use crate::processor::control_unit::{
    registers::Value,
    semantic_logic_unit::{
        microcode::Microcode,
        openai::{
            OpenAIClient,
            chat_completion_models::{
                OpenAIChatCompletionRequest, OpenAIChatCompletionRequestText,
            },
            embeddings_models::OpenAIEmbeddingsRequest, model_config::ModelConfig,
        },
    },
};

mod microcode;
mod openai;

pub struct SemanticLogicUnit {
    micro_code: Microcode,
    openai_client: OpenAIClient,
    text_model: ModelConfig,
    embeddings_model: ModelConfig,
}

impl SemanticLogicUnit {
    pub fn new() -> Self {
        return SemanticLogicUnit {
            micro_code: Microcode::new(),
            openai_client: OpenAIClient::new(),
            text_model: ModelConfig {
                model: "LFM2-2.6B-Q5_K_M",
                role: Some("user"),
                stream: false,
                temperature: Some(0.3),
                top_p: None,
                min_p: Some(0.15),
                repetition_penalty: Some(1.05),
                encoding_format: None,
            },
            embeddings_model: ModelConfig {
                model: "Qwen3-Embedding-0.6B-Q4_1-imat",
                role: None,
                stream: false,
                temperature: None,
                top_p: None,
                min_p: None,
                repetition_penalty: None,
                encoding_format: Some("float"),
            },
        };
    }

    fn clean_string(&self, value: &str) -> String {
        return value.trim().replace("\n", "").to_string();
    }

    fn chat(&self, content: &str) -> Result<String, String> {
        let request = OpenAIChatCompletionRequest {
            model: self.text_model.model.to_string(),
            stream: self.text_model.stream,
            messages: vec![OpenAIChatCompletionRequestText {
                role: self
                    .text_model
                    .role
                    .expect("Role is required for chat completion.")
                    .to_string(),
                content: content.to_string(),
            }],
            temperature: self.text_model.temperature,
            top_p: self.text_model.top_p,
            min_p: self.text_model.min_p,
            presence_penalty: self.text_model.repetition_penalty,
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
            model: self.embeddings_model.model.to_string(),
            input: content.to_string(),
            encoding_format: self
                .embeddings_model
                .encoding_format
                .expect("Encoding format is required for embeddings.")
                .to_string(),
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
            Some(value) => Ok(value.embedding.to_owned()),
            None => Err("No embeddings returned from client.".to_string()),
        };
    }

    pub fn addition(&self, value_a: &Value, value_b: &Value) -> Result<String, String> {
        let value_a = match value_a {
            Value::Text(text) => text,
            _ => return Err("Addition requires text value.".to_string()),
        };
        let value_b = match value_b {
            Value::Text(text) => text,
            _ => return Err("Addition requires text value.".to_string()),
        };

        let content = self.micro_code.addition(value_a, value_b);

        return match &self.chat(content.as_str()) {
            Ok(choice) => Ok(choice.to_lowercase()),
            Err(error) => Err(format!("Failed to perform addition. Error: {}", error)),
        };
    }

    pub fn subtract(&self, value_a: &Value, value_b: &Value) -> Result<String, String> {
        let value_a = match value_a {
            Value::Text(text) => text,
            _ => return Err("Subtraction requires text value.".to_string()),
        };
        let value_b = match value_b {
            Value::Text(text) => text,
            _ => return Err("Subtraction requires text value.".to_string()),
        };

        let content = self.micro_code.subtract(value_a, value_b);

        return match &self.chat(content.as_str()) {
            Ok(choice) => Ok(choice.to_lowercase()),
            Err(error) => Err(format!("Failed to perform subtraction. Error: {}", error)),
        };
    }

    pub fn multiply(&self, value_a: &Value, value_b: &Value) -> Result<String, String> {
        let value_a = match value_a {
            Value::Text(text) => text,
            _ => return Err("Multiplication requires text value.".to_string()),
        };
        let value_b = match value_b {
            Value::Text(text) => text,
            _ => return Err("Multiplication requires text value.".to_string()),
        };

        let content = self.micro_code.multiply(value_a, value_b);

        return match &self.chat(content.as_str()) {
            Ok(choice) => Ok(choice.to_lowercase()),
            Err(error) => Err(format!(
                "Failed to perform multiplication. Error: {}",
                error
            )),
        };
    }

    pub fn divide(&self, value_a: &Value, value_b: &Value) -> Result<String, String> {
        let value_a = match value_a {
            Value::Text(text) => text,
            _ => return Err("Division requires text value.".to_string()),
        };
        let value_b = match value_b {
            Value::Text(text) => text,
            _ => return Err("Division requires text value.".to_string()),
        };

        let content = self.micro_code.divide(value_a, value_b);

        return match &self.chat(content.as_str()) {
            Ok(choice) => Ok(choice.to_lowercase()),
            Err(error) => Err(format!("Failed to perform division. Error: {}", error)),
        };
    }

    pub fn similarity(&self, value_a: &Value, value_b: &Value) -> Result<u32, String> {
        let value_a = match value_a {
            Value::Text(text) => text,
            _ => return Err("Similarity requires text value.".to_string()),
        };
        let value_b = match value_b {
            Value::Text(text) => text,
            _ => return Err("Similarity requires text value.".to_string()),
        };

        let value_a_embeddings = match self.embeddings(value_a) {
            Ok(embedding) => embedding,
            Err(error) => return Err(format!("Failed to get first embedding. Error: {}", error)),
        };

        let value_b_embeddings = match self.embeddings(value_b) {
            Ok(embedding) => embedding,
            Err(error) => return Err(format!("Failed to get second embedding. Error: {}", error)),
        };

        // Compute cosine similarity.
        let dot_product: f32 = value_a_embeddings
            .iter()
            .zip(value_b_embeddings.iter())
            .map(|(a, b)| a * b)
            .sum();
        let x_euclidean_length: f32 = value_a_embeddings.iter().map(|x| x * x).sum::<f32>().sqrt();
        let y_euclidean_length: f32 = value_b_embeddings.iter().map(|y| y * y).sum::<f32>().sqrt();
        let similarity = dot_product / (x_euclidean_length * y_euclidean_length);
        let percentage_similarity = similarity.clamp(0.0, 1.0) * 100.0;

        return Ok(percentage_similarity.round() as u32);
    }
}
