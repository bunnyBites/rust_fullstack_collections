use axum::Json;
use reqwest::{Client, StatusCode};

use crate::model::{ApiResponse, OpenAiMessage, OpenAiPayload};

pub async fn ask_openai(prompt: &str, api_key: &str) -> (StatusCode, Json<ApiResponse>) {
    // create a reqwest client for communicating with openai
    let client = Client::new();

    let openai_url = "https://api.openai.com/v1/chat/completions";

    let openai_payload = OpenAiPayload {
        model: "gpt-4.1-nano",
        temperature: 0.7,
        messages: vec![
            OpenAiMessage {
                role: "assistant",
                content: "You are a AI Assistent, who is professional and funny at the same time",
            },
            OpenAiMessage {
                role: "user",
                content: prompt,
            },
        ],
    };

    match client
        .post(openai_url)
        .bearer_auth(api_key)
        .json(&openai_payload)
        .send()
        .await
    {
        Ok(response) => match response.text().await {
            Ok(response_text) => (
                StatusCode::OK,
                Json(ApiResponse {
                    response: response_text,
                }),
            ),
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    response: "Failed to read response text".to_string(),
                }),
            ),
        },
        Err(_err) => {
            eprintln!("Failed to communicate with openai");

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    response: "Failed to communicate with openai".to_string(),
                }),
            )
        }
    }
}
