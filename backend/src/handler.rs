use axum::Json;
use reqwest::StatusCode;
use std::env; // to read environment variables

use crate::{
    model::{ApiResponse, PromptRequest},
    openai_api::ask_openai,
};

pub async fn root_handler() -> &'static str {
    "Hello World !!"
}

// Axum will automatically convert the payload to PromptRequest
// If the payload is not able to deserialize or has type error
// Then 400 error is sent back
pub async fn chat_request(Json(payload): Json<PromptRequest>) -> (StatusCode, Json<ApiResponse>) {
    let prompt = &payload.prompt;

    match env::var("OPENAI_API_KEY") {
        Ok(api_key) => {
            println!("Key fetched successfully !!");
            return ask_openai(&prompt, &api_key).await;
        }
        Err(_err) => {
            eprintln!("OPENAI_API_KEY was not found!!");
            let prepared_error_response = ApiResponse {
                response: "Not able to fetch a valid api key".to_string(),
            };

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(prepared_error_response),
            )
        }
    }
}
