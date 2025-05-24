use axum::Json;
use reqwest::{Client, StatusCode};

use crate::model::{ApiResponse, OpenAiMessage, OpenAiPayload, OpenAiResponse};

pub async fn ask_openai(prompt: &str, api_key: &str) -> (StatusCode, Json<ApiResponse>) {
    // create a reqwest client for communicating with openai
    let client = Client::new();

    let openai_url = "https://api.openai.com/v1/chat/completions";

    let openai_payload = OpenAiPayload {
        model: "gpt-4.1-nano",
        temperature: 0.3,
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
        Ok(response) => match response.json::<OpenAiResponse>().await {
            Ok(response_json) => {
                if let Some(choice) = response_json.choices.get(0) {
                    return (
                        StatusCode::OK,
                        Json(ApiResponse {
                            response: choice.message.content.to_string(),
                        }),
                    );
                } else {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ApiResponse {
                            response: "Failed to get proper response from openai".to_string(),
                        }),
                    );
                }
            }
            Err(err) => {
                eprintln!("{}", err);

                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse {
                        response: "Failed to read response text".to_string(),
                    }),
                );
            }
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
