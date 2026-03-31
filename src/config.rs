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
        use crate::constants;

        fn env_opt_bool(key: &str) -> Option<bool> {
            std::env::var(key).ok().map(|v| v == "true")
        }

        fn env_opt<T: std::str::FromStr>(key: &str) -> Option<T> {
            std::env::var(key).ok().and_then(|v| v.parse().ok())
        }

        TextModelOverrides {
            stream: env_opt_bool(constants::TEXT_MODEL_STREAM_ENV),
            return_progress: env_opt_bool(constants::TEXT_MODEL_RETURN_PROGRESS_ENV),
            reasoning_format: std::env::var(constants::TEXT_MODEL_REASONING_FORMAT_ENV).ok(),
            temperature: env_opt(constants::TEXT_MODEL_TEMPERATURE_ENV),
            dynatemp_range: env_opt(constants::TEXT_MODEL_DYNATEMP_RANGE_ENV),
            dynatemp_exponent: env_opt(constants::TEXT_MODEL_DYNATEMP_EXPONENT_ENV),
            top_k: env_opt(constants::TEXT_MODEL_TOP_K_ENV),
            top_p: env_opt(constants::TEXT_MODEL_TOP_P_ENV),
            min_p: env_opt(constants::TEXT_MODEL_MIN_P_ENV),
            xtc_probability: env_opt(constants::TEXT_MODEL_XTC_PROBABILITY_ENV),
            xtc_threshold: env_opt(constants::TEXT_MODEL_XTC_THRESHOLD_ENV),
            typ_p: env_opt(constants::TEXT_MODEL_TYP_P_ENV),
            max_tokens: env_opt(constants::TEXT_MODEL_MAX_TOKENS_ENV),
            repeat_last_n: env_opt(constants::TEXT_MODEL_REPEAT_LAST_N_ENV),
            repeat_penalty: env_opt(constants::TEXT_MODEL_REPEAT_PENALTY_ENV),
            presence_penalty: env_opt(constants::TEXT_MODEL_PRESENCE_PENALTY_ENV),
            frequency_penalty: env_opt(constants::TEXT_MODEL_FREQUENCY_PENALTY_ENV),
            dry_multiplier: env_opt(constants::TEXT_MODEL_DRY_MULTIPLIER_ENV),
            dry_base: env_opt(constants::TEXT_MODEL_DRY_BASE_ENV),
            dry_allowed_length: env_opt(constants::TEXT_MODEL_DRY_ALLOWED_LENGTH_ENV),
            dry_penalty_last_n: env_opt(constants::TEXT_MODEL_DRY_PENALTY_LAST_N_ENV),
            timings_per_token: env_opt_bool(constants::TEXT_MODEL_TIMINGS_PER_TOKEN_ENV),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub text_model: String,
    pub embedding_model: String,
    pub text_model_overrides: TextModelOverrides,
    pub debug_build: bool,
    pub debug_run: bool,
    pub debug_chat: bool,
}
