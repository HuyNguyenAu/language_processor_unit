use crate::processor::control_unit::semantic_logic_unit::{
    microcode::Microcode,
    openai::{OpenAIChatRequest, OpenAIChatRequestText, OpenAIClient, OpenAIEmbeddingsRequest},
};

mod microcode;
mod openai;

pub struct SemanticLogicUnit {
    micro_code: Microcode,
    openai_client: OpenAIClient,
    model: &'static str,
    role: &'static str,
    stream: bool,
    temperature: f32,
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
            temperature: 0.8,
            encoding_format: "float",
        };
    }

    fn clean_string(&self, value: &str) -> String {
        return value.trim().replace("\n", "").to_string();
    }

    fn chat(&self, content: &str) -> Result<String, String> {
        let request = OpenAIChatRequest {
            model: self.model.to_string(),
            stream: self.stream,
            messages: vec![OpenAIChatRequestText {
                role: self.role.to_string(),
                content: content.to_string(),
            }],
            temperature: self.temperature,
        };

        let response = &self.openai_client.chat(request);

        let choice = match response {
            Ok(response) => response.choices.iter().nth(0),
            Err(err) => {
                return Err(format!(
                    "Failed to get chat response from client. Error: {}",
                    err
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

        let response = &self.openai_client.embeddings(request);

        let embeddings = match response {
            Ok(response) => response.data.iter().nth(0),
            Err(err) => {
                return Err(format!(
                    "Failed to get embeddings response from client. Error: {}",
                    err
                ));
            }
        };

        return match embeddings {
            Some(value) => Ok(value.embedding.clone()),
            None => Err("No embeddings returned from client.".to_string()),
        };
    }

    pub fn addition(&self, first_operand: &str, second_operand: &str) -> String {
        let content = self.micro_code.addition(first_operand, second_operand);

        return match &self.chat(content.as_str()) {
            Ok(choice) => choice.to_lowercase(),
            Err(error) => panic!("Failed to perform addition. Error: {}", error),
        };
    }

    pub fn subtract(&self, first_operand: &str, second_operand: &str) -> String {
        let content = self.micro_code.subtract(first_operand, second_operand);

        return match &self.chat(content.as_str()) {
            Ok(choice) => choice.to_lowercase(),
            Err(error) => panic!("Failed to perform subtraction. Error: {}", error),
        };
    }

    pub fn similarity(&self, first_operand: &str, second_operand: &str) -> String {
        let first_embedding_result = self.embeddings(first_operand);
        let first_embedding = match &first_embedding_result {
            Ok(embedding) => embedding,
            Err(error) => panic!("Failed to get first embedding. Error: {}", error),
        };

        let second_embedding_result = self.embeddings(second_operand);
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

        return ((similarity * 100.0).round()).to_string();
    }
}
