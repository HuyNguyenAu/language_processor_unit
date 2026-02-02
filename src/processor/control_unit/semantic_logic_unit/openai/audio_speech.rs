use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIAudioSpeechRequest {
    #[serde(rename = "input")]
    pub input: String,
    #[serde(rename = "model")]
    pub model: String,
    #[serde(rename = "voice")]
    pub voice: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIAudioSpeechResponse {
    #[serde(rename = "audio")]
    pub audio: String,
    #[serde(rename = "type")]
    pub _type: String,
}
