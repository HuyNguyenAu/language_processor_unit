use crate::{
    assembler::opcode::OpCode,
    processor::control_unit::{
        instruction::RType,
        language_logic_unit::openai::{
            OpenAIClient,
            chat_completion_models::{
                OpenAIChatCompletionRequest, OpenAIChatCompletionRequestText,
            },
            embeddings_models::OpenAIEmbeddingsRequest,
            model_config::{ModelConfig, ModelEmbeddingsConfig, ModelTextConfig},
        },
        registers::Value,
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
        Self {
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
                ]
                .to_vec(),
                timings_per_token: false,
            }),
            embeddings_model: ModelConfig::Embeddings(ModelEmbeddingsConfig {
                model: "Qwen3-Embedding-0.6B-Q4_1-imat.gguf".to_string(),
                encoding_format: "float".to_string(),
            }),
        }
    }

    fn clean_string(&self, value: &str) -> String {
        value.trim().replace("\n", "").to_string()
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

        let response = self
            .openai_client
            .create_chat_completion(request)
            .map_err(|error| {
                format!("Failed to get chat response from client. Error: {}", error)
            })?;

        let choice = response
            .choices
            .first()
            .ok_or_else(|| "No choices returned from client.".to_string())?;

        Ok(self.clean_string(&choice.message.content))
    }

    fn embeddings(&self, content: &str) -> Result<Vec<f32>, String> {
        let model = match &self.embeddings_model {
            ModelConfig::Embeddings(config) => config,
            _ => {
                return Err(
                    "Embeddings model configuration is required for embeddings.".to_string()
                );
            }
        };

        let request = OpenAIEmbeddingsRequest {
            model: model.model.to_string(),
            input: content.to_string(),
            encoding_format: model.encoding_format.to_string(),
        };

        let response = self
            .openai_client
            .create_embeddings(request)
            .map_err(|error| {
                format!(
                    "Failed to get embeddings response from client. Error: {}",
                    error
                )
            })?;

        let embeddings = response
            .data
            .first()
            .ok_or_else(|| "No embeddings returned from client.".to_string())?;

        Ok(embeddings.embedding.to_owned())
    }

    fn cosine_similarity(&self, value_a: &Value, value_b: &Value) -> Result<u32, String> {
        let value_a = match value_a {
            Value::Text(text) => text,
            Value::Number(number) => &number.to_string(),
            _ => return Err(format!("{:?} requires text value.", OpCode::Similarity)),
        };
        let value_b = match value_b {
            Value::Text(text) => text,
            Value::Number(number) => &number.to_string(),
            _ => return Err(format!("{:?} requires text value.", OpCode::Similarity)),
        };

        let value_a_embeddings = self.embeddings(value_a).map_err(|error| {
            format!("Failed to get embedding for {}. Error: {}", value_a, error)
        })?;

        let value_b_embeddings = self.embeddings(value_b).map_err(|error| {
            format!("Failed to get embedding for {}. Error: {}", value_b, error)
        })?;

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

        Ok(percentage_similarity.round() as u32)
    }

    fn execute(&self, r_type: &RType, value_a: &Value, value_b: &Value) -> Result<String, String> {
        let value_a = match value_a {
            Value::Text(text) => text,
            _ => return Err(format!("{:?} requires text value.", r_type)),
        };
        let value_b = match value_b {
            Value::Text(text) => text,
            _ => return Err(format!("{:?} requires text value.", r_type)),
        };

        let prompt = micro_prompt::search(r_type, value_a, value_b).map_err(|error| {
            format!(
                "Failed to generate micro prompt for {:?}. Error: {}",
                r_type, error
            )
        })?;

        let result = self
            .chat(prompt.as_str())
            .map_err(|error| format!("Failed to perform {:?}. Error: {}", r_type, error))?;

        Ok(result.to_lowercase())
    }

    fn boolean(&self, r_type: &RType, value: &str) -> Result<u32, String> {
        let true_values = micro_prompt::true_values(r_type).map_err(|error| {
            format!(
                "Failed to get true values for {:?}. Error: {}",
                r_type, error
            )
        })?;

        Ok(if true_values.contains(&value.to_uppercase().as_str()) {
            100
        } else {
            0
        })
    }

    pub fn run(&self, r_type: &RType, value_a: &Value, value_b: &Value) -> Result<Value, String> {
        if matches!(r_type, RType::Audit) {
            let value = self.execute(r_type, value_a, value_b).map_err(|error| {
                format!(
                    "Failed to execute {:?} for boolean operation. Error: {}",
                    r_type, error
                )
            })?;

            return self.boolean(r_type, &value).map(Value::Number);
        }

        if matches!(r_type, RType::Similarity) {
            return self.cosine_similarity(value_a, value_b).map(Value::Number);
        }

        self.execute(r_type, value_a, value_b).map(Value::Text)
    }
}
