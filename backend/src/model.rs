// <<<< ---- Data structure used for the api ---- >>>>

use serde::{Deserialize, Serialize};

// Deserialize is used because we recieve the JSON from Frontend
// which will be converted to this structure
#[derive(Deserialize)]
pub struct PromptRequest {
    pub prompt: String,
}

// Serialize is used because we want to send the response to the frontend
// we need to convert our JSON to this structure
#[derive(Serialize)]
pub struct ApiResponse {
    pub response: String,
}

// <<<<---- Open AI ---->>>>

#[derive(Serialize)]
pub struct OpenAiMessage<'a> {
    pub role: &'a str,
    pub content: &'a str,
}

#[derive(Serialize)]
pub struct OpenAiPayload<'a> {
    pub model: &'a str,
    pub temperature: f32,
    pub messages: Vec<OpenAiMessage<'a>>,
}

#[derive(Deserialize)]
pub struct OpenAiResponseMessage {
    pub content: String,
}

#[derive(Deserialize)]
pub struct OpenAiChoice {
    pub message: OpenAiResponseMessage,
}

#[derive(Deserialize)]
pub struct OpenAiResponse {
    pub choices: Vec<OpenAiChoice>,
}
