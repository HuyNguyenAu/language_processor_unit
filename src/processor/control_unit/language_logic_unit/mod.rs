use crate::{
    assembler::opcode::OpCode,
    processor::control_unit::{
        registers::Value,
        language_logic_unit::openai::{
            OpenAIClient,
            chat_completion_models::{
                OpenAIChatCompletionRequest, OpenAIChatCompletionRequestText,
            },
            embeddings_models::OpenAIEmbeddingsRequest,
            model_config::{ModelConfig, ModelEmbeddingsConfig, ModelTextConfig},
        },
    },
};

mod micro_prompt;
mod openai;

pub struct LanguageLogicUnit {
    system_prompt: &'static str,
    openai_client: OpenAIClient,
    text_model: ModelConfig,
    embeddings_model: ModelConfig,
}

impl LanguageLogicUnit {
    pub fn new() -> Self {
        return Self {
            system_prompt: "Output ONLY the answer. No intro. No fluff. No punctuation unless required. Answer with a single word if appropriate, otherwise a single sentence.",
            openai_client: OpenAIClient::new(),
            text_model: ModelConfig::Text(ModelTextConfig {
                stream: false,
                return_progress: false,
                model: "LFM2-2.6B-Q5_K_M.gguf".to_string(),
                reasoning_format: "auto".to_string(),
                temperature: 0.3,
                max_tokens: -1,
                dynatemp_range: 0.0,
                dynatemp_exponent: 1.0,
                top_k: 40,
                top_p: 0.95,
                min_p: 0.15,
                xtc_probability: 0.0,
                xtc_threshold: 0.1,
                typ_p: 1.0,
                repeat_last_n: 64,
                repeat_penalty: 1.05,
                presence_penalty: 0.0,
                frequency_penalty: 0.0,
                dry_multiplier: 0.0,
                dry_base: 1.75,
                dry_allowed_length: 2,
                dry_penalty_last_n: -1,
                samplers: [
                    "penalties".to_string(),
                    "dry".to_string(),
                    "top_n_sigma".to_string(),
                    "top_k".to_string(),
                    "typ_p".to_string(),
                    "top_p".to_string(),
                    "min_p".to_string(),
                    "xtc".to_string(),
                    "temperature".to_string(),
                ].to_vec(),
                timings_per_token: false,
            }),
            embeddings_model: ModelConfig::Embeddings(ModelEmbeddingsConfig {
                model: "Qwen3-Embedding-0.6B-Q4_1-imat.gguf".to_string(),
                encoding_format: "float".to_string(),
            }),
        };
    }

    fn clean_string(&self, value: &str) -> String {
        return value.trim().replace("\n", "").to_string();
    }

    fn chat(&self, content: &str) -> Result<String, String> {
        let model = match &self.text_model {
            ModelConfig::Text(config) => config,
            _ => return Err("Text model configuration is required for chat.".to_string()),
        };

        let request = OpenAIChatCompletionRequest {
            messages: vec![
                OpenAIChatCompletionRequestText {
                    role: "system".to_string(),
                    content: self.system_prompt.to_string(),
                },
                OpenAIChatCompletionRequestText {
                    role: "user".to_string(),
                    content: content.to_string(),
                },
            ],
            stream: model.stream,
            return_progress: model.return_progress,
            model: model.model.clone(),
            reasoning_format: model.reasoning_format.clone(),
            temperature: model.temperature,
            max_tokens: model.max_tokens,
            dynatemp_range: model.dynatemp_range,
            dynatemp_exponent: model.dynatemp_exponent,
            top_k: model.top_k,
            top_p: model.top_p,
            min_p: model.min_p,
            xtc_probability: model.xtc_probability,
            xtc_threshold: model.xtc_threshold,
            typ_p: model.typ_p,
            repeat_last_n: model.repeat_last_n,
            repeat_penalty: model.repeat_penalty,
            presence_penalty: model.presence_penalty,
            frequency_penalty: model.frequency_penalty,
            dry_multiplier: model.dry_multiplier,
            dry_base: model.dry_base,
            dry_allowed_length: model.dry_allowed_length,
            dry_penalty_last_n: model.dry_penalty_last_n,
            samplers: model.samplers.to_vec(),
            timings_per_token: model.timings_per_token,
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
        let model = match &self.embeddings_model {
            ModelConfig::Embeddings(config) => config,
            _ => {
                return Err("Embeddings model configuration is required for embeddings.".to_string());
            }
        };

        let request = OpenAIEmbeddingsRequest {
            model: model.model.to_string(),
            input: content.to_string(),
            encoding_format: model.encoding_format.to_string(),
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

    fn cosine_similarity(&self, value_a: &Value, value_b: &Value) -> Result<u32, String> {
        let value_a = match value_a {
            Value::Text(text) => text,
            _ => return Err(format!("{:?} requires text value.", OpCode::SIM)),
        };
        let value_b = match value_b {
            Value::Text(text) => text,
            _ => return Err(format!("{:?} requires text value.", OpCode::SIM)),
        };

        let value_a_embeddings = match self.embeddings(&value_a) {
            Ok(embedding) => embedding,
            Err(error) => {
                return Err(format!(
                    "Failed to get embedding for {}. Error: {}",
                    value_a, error
                ));
            }
        };

        let value_b_embeddings = match self.embeddings(&value_b) {
            Ok(embedding) => embedding,
            Err(error) => {
                return Err(format!(
                    "Failed to get embedding for {}. Error: {}",
                    value_b, error
                ));
            }
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

    fn execute(&self, opcode: &OpCode, value_a: &Value, value_b: &Value) -> Result<String, String> {
        let value_a = match value_a {
            Value::Text(text) => text,
            _ => return Err(format!("{:?} requires text value.", opcode)),
        };
        let value_b = match value_b {
            Value::Text(text) => text,
            _ => return Err(format!("{:?} requires text value.", opcode)),
        };
        let prompt = match micro_prompt::search(opcode, value_a, value_b) {
            Ok(prompt) => prompt,
            Err(error) => {
                return Err(format!(
                    "Failed to generate micro prompt for {:?}. Error: {}",
                    opcode, error
                ));
            }
        };

        let value = match &self.chat(prompt.as_str()) {
            Ok(choice) => Ok(choice.to_lowercase()),
            Err(error) => Err(format!("Failed to perform {:?}. Error: {}", opcode, error)),
        };

        return value;
    }

    fn boolean(&self, opcode: &OpCode, value: &str) -> Result<u32, String> {
        let true_values = match micro_prompt::true_values(opcode) {
            Ok(values) => values,
            Err(error) => {
                return Err(format!(
                    "Failed to get true values for {:?}. Error: {}",
                    opcode, error
                ));
            }
        };

        return match true_values.contains(&value.to_uppercase().as_str()) {
            true => Ok(100),
            false => Ok(0),
        };
    }

    pub fn run(&self, opcode: &OpCode, value_a: &Value, value_b: &Value) -> Result<Value, String> {
        if matches!(opcode, OpCode::EQV | OpCode::INT | OpCode::HAL) {
            let value = match self.execute(opcode, value_a, value_b) {
                Ok(value) => value,
                Err(error) => {
                    return Err(format!(
                        "Failed to execute {:?} for boolean operation. Error: {}",
                        opcode, error
                    ));
                }
            };

            return self.boolean(opcode, &value).map(Value::Number);
        }

        if opcode == &OpCode::SIM {
            return self.cosine_similarity(value_a, value_b).map(Value::Number);
        }

        return self.execute(opcode, value_a, value_b).map(Value::Text);
    }
}
