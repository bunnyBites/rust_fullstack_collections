use std::{str::FromStr, sync::Arc};

use anchor_lang::AnchorDeserialize;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;

use crate::model::{Task, TaskAPIResponse};

const PROGRAM_ID: &str = "AUEPaukkyUiQVu5YFb76mJU9yfzXzi78qrKQnbK8H3c1";

pub async fn get_todos(
    Path(user_pubkey): Path<String>,
    State(rpc_client): State<Arc<RpcClient>>,
) -> Result<Json<TaskAPIResponse>, impl IntoResponse> {
    let user_pk = match Pubkey::from_str(&user_pubkey) {
        Ok(user_public_key) => user_public_key,
        Err(_) => {
            return Err((StatusCode::NOT_FOUND, "Provide user is not present"));
        }
    };

    let program_pk = match Pubkey::from_str(PROGRAM_ID) {
        Ok(program_public_key) => program_public_key,
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Provide program id is invalid",
            ));
        }
    };

    let (todo_pda, _bump) = Pubkey::find_program_address(&[b"list", user_pk.as_ref()], &program_pk);

    let fetched_todo_account_data = match rpc_client.get_account_data(&todo_pda) {
        Ok(todo_account) => todo_account,
        Err(_) => return Err((StatusCode::BAD_REQUEST, "Failed to fetch account data")),
    };

    if fetched_todo_account_data.len() < 8 {
        return Err((
            StatusCode::NOT_ACCEPTABLE,
            "Fetched account data size is less than required size",
        ));
    }

    let mut todo_account_data_without_discriminator = &fetched_todo_account_data[8..];

    match Task::deserialize(&mut todo_account_data_without_discriminator) {
        Ok(todo_data) => Ok(Json(TaskAPIResponse {
            user: todo_data.user.to_string(),
            bump: todo_data.bump,
            todos: todo_data.todos,
        })),
        Err(e) => {
            eprintln!("Failed to deserialize: {:?}", e);

            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to Deserialize data",
            ));
        }
    }
}
