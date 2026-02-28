use crate::{
    assembler::roles,
    processor::{
        control_unit::language_logic_unit::openai::{
            OpenAIClient,
            chat_completion_models::{
                OpenAIChatCompletionRequest, OpenAIChatCompletionRequestText,
            },
            embeddings_models::OpenAIEmbeddingsRequest,
            model_config::{ModelEmbeddingsConfig, ModelTextConfig},
        },
        registers::ContextMessage,
    },
};

mod openai;

const SYSTEM_PROMPT: &str =
    "Provide exactly the requested output. Follow structural markers strictly.";

pub struct LanguageLogicUnit;

impl LanguageLogicUnit {
    fn default_text_model() -> ModelTextConfig {
        ModelTextConfig {
            stream: false,
            return_progress: false,
            model: "Generative".to_string(),
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
            samplers: vec![
                "penalties".to_string(),
                "dry".to_string(),
                "top_n_sigma".to_string(),
                "top_k".to_string(),
                "typ_p".to_string(),
                "top_p".to_string(),
                "min_p".to_string(),
                "xtc".to_string(),
                "temperature".to_string(),
            ],
            timings_per_token: false,
        }
    }

    fn default_embeddings_model() -> ModelEmbeddingsConfig {
        ModelEmbeddingsConfig {
            model: "Embedding".to_string(),
            encoding_format: "float".to_string(),
        }
    }

    fn clean_string(value: &str) -> String {
        value.trim().replace("\n", "").to_string()
    }

    fn chat(content: &str, context: Vec<ContextMessage>) -> Result<String, String> {
        let model = Self::default_text_model();
        let messages = std::iter::once(OpenAIChatCompletionRequestText {
            role: roles::SYSTEM_ROLE.to_string(),
            content: SYSTEM_PROMPT.to_string(),
        })
        .chain(
            context
                .into_iter()
                .map(|message| OpenAIChatCompletionRequestText {
                    role: message.role,
                    content: message.content,
                }),
        )
        .chain(std::iter::once(OpenAIChatCompletionRequestText {
            role: roles::USER_ROLE.to_string(),
            content: content.to_string(),
        }))
        .collect::<Vec<OpenAIChatCompletionRequestText>>();

        let request = OpenAIChatCompletionRequest {
            messages,
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

        let response = OpenAIClient::chat_completion(request).map_err(|error| {
            format!("Failed to get chat response from client. Error: {}", error)
        })?;

        let choice = response
            .choices
            .first()
            .ok_or_else(|| "No choices returned from client.".to_string())?;
        let result = Self::clean_string(&choice.message.content);

        Ok(result)
    }

    fn embeddings(content: &str) -> Result<Vec<f32>, String> {
        let model = Self::default_embeddings_model();
        let request = OpenAIEmbeddingsRequest {
            model: model.model.to_string(),
            input: content.to_string(),
            encoding_format: model.encoding_format.to_string(),
        };

        let response = OpenAIClient::embeddings(request).map_err(|error| {
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

    pub fn cosine_similarity(value_a: &str, value_b: &str) -> Result<u32, String> {
        let value_a_embeddings = Self::embeddings(value_a).map_err(|error| {
            format!("Failed to get embedding for {}. Error: {}", value_a, error)
        })?;

        let value_b_embeddings = Self::embeddings(value_b).map_err(|error| {
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

    pub fn string(micro_prompt: &str, context: Vec<ContextMessage>) -> Result<String, String> {
        let result = Self::chat(micro_prompt, context)
            .map_err(|error| format!("Failed to execute string operation. Error: {}", error))?;

        Ok(result)
    }

    pub fn boolean(
        micro_prompt: &str,
        true_values: Vec<&str>,
        false_values: Vec<&str>,
        context: Vec<ContextMessage>,
    ) -> Result<u32, String> {
        let value = Self::string(micro_prompt, context)
            .map_err(|error| format!("Failed to execute boolean operation. Error: {}", error))?;

        let mut true_scores = Vec::<u32>::new();

        for true_value in &true_values {
            if let Ok(score) =
                Self::cosine_similarity(&value.to_lowercase(), &true_value.to_lowercase())
            {
                true_scores.push(score);
            }
        }

        let mut false_scores = Vec::<u32>::new();

        for false_value in &false_values {
            if let Ok(score) =
                Self::cosine_similarity(&value.to_lowercase(), &false_value.to_lowercase())
            {
                false_scores.push(score);
            }
        }

        let max_true_score = true_scores.into_iter().max().unwrap_or(0);
        let max_false_score = false_scores.into_iter().max().unwrap_or(0);

        if max_true_score > max_false_score {
            return Ok(100);
        }
        Ok(0)
    }
}
