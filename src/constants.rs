pub const BUILD_DIR: &str = "build";

pub const HELP_USAGE: &str = "Usage: build <file_path> | run <file_path>";

// Model environment variable names.
pub const TEXT_MODEL_ENV: &str = "TEXT_MODEL";
pub const EMBEDDING_MODEL_ENV: &str = "EMBEDDING_MODEL";

// OpenAI API endpoint environment variable names.
pub const OPENAI_BASE_URL_ENV: &str = "OPENAI_BASE_URL";
pub const OPENAI_CHAT_COMPLETION_ENDPOINT_ENV: &str = "OPENAI_CHAT_COMPLETION_ENDPOINT";
pub const OPENAI_EMBEDDINGS_ENDPOINT_ENV: &str = "OPENAI_EMBEDDINGS_ENDPOINT";

// OpenAI API endpoint defaults.
pub const OPENAI_BASE_URL_DEFAULT: &str = "http://127.0.0.1:8080";
pub const OPENAI_CHAT_COMPLETION_ENDPOINT_DEFAULT: &str = "v1/chat/completions";
pub const OPENAI_EMBEDDINGS_ENDPOINT_DEFAULT: &str = "v1/embeddings";
pub const OPENAI_TIMEOUT_SECS_DEFAULT: u64 = 120;

// OpenAI API timeout environment variable name.
pub const OPENAI_TIMEOUT_SECS_ENV: &str = "OPENAI_TIMEOUT_SECS";

// Debug environment variable names.
pub const DEBUG_BUILD_ENV: &str = "DEBUG_BUILD";
pub const DEBUG_RUN_ENV: &str = "DEBUG_RUN";
pub const DEBUG_CHAT_ENV: &str = "DEBUG_CHAT";

// Optional text model parameter environment variable names.
pub const TEXT_MODEL_STREAM_ENV: &str = "TEXT_MODEL_STREAM";
pub const TEXT_MODEL_RETURN_PROGRESS_ENV: &str = "TEXT_MODEL_RETURN_PROGRESS";
pub const TEXT_MODEL_REASONING_FORMAT_ENV: &str = "TEXT_MODEL_REASONING_FORMAT";
pub const TEXT_MODEL_TEMPERATURE_ENV: &str = "TEXT_MODEL_TEMPERATURE";
pub const TEXT_MODEL_DYNATEMP_RANGE_ENV: &str = "TEXT_MODEL_DYNATEMP_RANGE";
pub const TEXT_MODEL_DYNATEMP_EXPONENT_ENV: &str = "TEXT_MODEL_DYNATEMP_EXPONENT";
pub const TEXT_MODEL_TOP_K_ENV: &str = "TEXT_MODEL_TOP_K";
pub const TEXT_MODEL_TOP_P_ENV: &str = "TEXT_MODEL_TOP_P";
pub const TEXT_MODEL_MIN_P_ENV: &str = "TEXT_MODEL_MIN_P";
pub const TEXT_MODEL_XTC_PROBABILITY_ENV: &str = "TEXT_MODEL_XTC_PROBABILITY";
pub const TEXT_MODEL_XTC_THRESHOLD_ENV: &str = "TEXT_MODEL_XTC_THRESHOLD";
pub const TEXT_MODEL_TYP_P_ENV: &str = "TEXT_MODEL_TYP_P";
pub const TEXT_MODEL_MAX_TOKENS_ENV: &str = "TEXT_MODEL_MAX_TOKENS";
pub const TEXT_MODEL_REPEAT_LAST_N_ENV: &str = "TEXT_MODEL_REPEAT_LAST_N";
pub const TEXT_MODEL_REPEAT_PENALTY_ENV: &str = "TEXT_MODEL_REPEAT_PENALTY";
pub const TEXT_MODEL_PRESENCE_PENALTY_ENV: &str = "TEXT_MODEL_PRESENCE_PENALTY";
pub const TEXT_MODEL_FREQUENCY_PENALTY_ENV: &str = "TEXT_MODEL_FREQUENCY_PENALTY";
pub const TEXT_MODEL_DRY_MULTIPLIER_ENV: &str = "TEXT_MODEL_DRY_MULTIPLIER";
pub const TEXT_MODEL_DRY_BASE_ENV: &str = "TEXT_MODEL_DRY_BASE";
pub const TEXT_MODEL_DRY_ALLOWED_LENGTH_ENV: &str = "TEXT_MODEL_DRY_ALLOWED_LENGTH";
pub const TEXT_MODEL_DRY_PENALTY_LAST_N_ENV: &str = "TEXT_MODEL_DRY_PENALTY_LAST_N";
pub const TEXT_MODEL_TIMINGS_PER_TOKEN_ENV: &str = "TEXT_MODEL_TIMINGS_PER_TOKEN";

// Language logic unit constants.
pub const SYSTEM_PROMPT: &str =
    "Provide exactly the requested output. Follow structural markers strictly.";
