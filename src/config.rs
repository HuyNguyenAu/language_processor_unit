use crate::{constants, env};

#[derive(Debug, Clone, Default)]
pub struct TextModelOverrides {
    pub stream: Option<bool>,
    pub return_progress: Option<bool>,
    pub reasoning_format: Option<String>,
    pub temperature: Option<f32>,
    pub dynatemp_range: Option<f32>,
    pub dynatemp_exponent: Option<f32>,
    pub top_k: Option<u32>,
    pub top_p: Option<f32>,
    pub min_p: Option<f32>,
    pub xtc_probability: Option<f32>,
    pub xtc_threshold: Option<f32>,
    pub typ_p: Option<f32>,
    pub max_tokens: Option<i32>,
    pub repeat_last_n: Option<u32>,
    pub repeat_penalty: Option<f32>,
    pub presence_penalty: Option<f32>,
    pub frequency_penalty: Option<f32>,
    pub dry_multiplier: Option<f32>,
    pub dry_base: Option<f32>,
    pub dry_allowed_length: Option<u32>,
    pub dry_penalty_last_n: Option<i32>,
    pub timings_per_token: Option<bool>,
}

impl TextModelOverrides {
    pub fn from_env() -> Self {
        TextModelOverrides {
            stream: env::opt_bool(constants::TEXT_MODEL_STREAM_ENV),
            return_progress: env::opt_bool(constants::TEXT_MODEL_RETURN_PROGRESS_ENV),
            reasoning_format: env::opt(constants::TEXT_MODEL_REASONING_FORMAT_ENV),
            temperature: env::opt(constants::TEXT_MODEL_TEMPERATURE_ENV),
            dynatemp_range: env::opt(constants::TEXT_MODEL_DYNATEMP_RANGE_ENV),
            dynatemp_exponent: env::opt(constants::TEXT_MODEL_DYNATEMP_EXPONENT_ENV),
            top_k: env::opt(constants::TEXT_MODEL_TOP_K_ENV),
            top_p: env::opt(constants::TEXT_MODEL_TOP_P_ENV),
            min_p: env::opt(constants::TEXT_MODEL_MIN_P_ENV),
            xtc_probability: env::opt(constants::TEXT_MODEL_XTC_PROBABILITY_ENV),
            xtc_threshold: env::opt(constants::TEXT_MODEL_XTC_THRESHOLD_ENV),
            typ_p: env::opt(constants::TEXT_MODEL_TYP_P_ENV),
            max_tokens: env::opt(constants::TEXT_MODEL_MAX_TOKENS_ENV),
            repeat_last_n: env::opt(constants::TEXT_MODEL_REPEAT_LAST_N_ENV),
            repeat_penalty: env::opt(constants::TEXT_MODEL_REPEAT_PENALTY_ENV),
            presence_penalty: env::opt(constants::TEXT_MODEL_PRESENCE_PENALTY_ENV),
            frequency_penalty: env::opt(constants::TEXT_MODEL_FREQUENCY_PENALTY_ENV),
            dry_multiplier: env::opt(constants::TEXT_MODEL_DRY_MULTIPLIER_ENV),
            dry_base: env::opt(constants::TEXT_MODEL_DRY_BASE_ENV),
            dry_allowed_length: env::opt(constants::TEXT_MODEL_DRY_ALLOWED_LENGTH_ENV),
            dry_penalty_last_n: env::opt(constants::TEXT_MODEL_DRY_PENALTY_LAST_N_ENV),
            timings_per_token: env::opt_bool(constants::TEXT_MODEL_TIMINGS_PER_TOKEN_ENV),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    // Model.
    pub text_model: String,
    pub embedding_model: String,
    pub text_model_overrides: TextModelOverrides,
    // OpenAI API configuration.
    pub base_url: String,
    pub chat_completion_endpoint: String,
    pub embeddings_endpoint: String,
    pub timeout_secs: u64,
    // Debug flags.
    pub debug_build: bool,
    pub debug_run: bool,
    pub debug_chat: bool,
}
