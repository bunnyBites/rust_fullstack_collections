use anchor_client::{
    Program,
    solana_sdk::{pubkey::Pubkey, signature::Keypair},
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use base64::Engine;
use std::{str::FromStr, sync::Arc};

use crate::{
    PROGRAM_ID,
    model::{CreateContent, CreateContentResponse, TaskAPIResponse, TodoAPIResponse},
};
use blockchain::{self, Todo};

type SharedProgram = State<Arc<Program<Arc<Keypair>>>>;

pub async fn get_todos(
    State(program_state): SharedProgram,
    Path(user_pubkey): Path<String>,
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

    let fetched_response =
        tokio::task::spawn_blocking(move || program_state.account::<Todo>(todo_pda))
            .await
            .expect("Failed to get account data");

    match fetched_response {
        Ok(fetched_response) => {
            let api_response = TaskAPIResponse {
                user: fetched_response.user.to_string(),
                bump: fetched_response.bump,
                todos: fetched_response
                    .todos
                    .into_iter()
                    .map(|todo| TodoAPIResponse {
                        content: todo.content,
                        is_completed: todo.is_completed,
                    })
                    .collect(),
            };

            Ok(Json(api_response))
        }
        Err(_) => Err((StatusCode::SERVICE_UNAVAILABLE, "Something went wrong")),
    }
}

pub async fn create_content(
    State(program_state): SharedProgram,
    Json(payload): Json<CreateContent>,
) -> Result<Json<CreateContentResponse>, impl IntoResponse> {
    let user_public_key = match Pubkey::from_str(&payload.user_public_key) {
        Ok(public_key) => public_key,
        Err(_) => {
            return Err((StatusCode::NOT_FOUND, "Provide invalid public key"));
        }
    };

    let program_id = match Pubkey::from_str(PROGRAM_ID) {
        Ok(todo_program_id) => todo_program_id,
        Err(_) => {
            return Err((StatusCode::NOT_FOUND, "Provided program id is not allowed"));
        }
    };

    let (todo_pda, _bump) =
        Pubkey::find_program_address(&[b"list", user_public_key.as_ref()], &program_id);

    let unsigned_tx = match program_state
        .request()
        .accounts(blockchain::accounts::AddTask {
            todo: todo_pda,
            user: user_public_key,
        })
        .args(blockchain::instruction::AddTask {
            content: payload.content,
        })
        .transaction()
    {
        Ok(tx) => tx,
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to build transaction",
            ));
        }
    };

    // serialise the transaction
    let serialized_tx = bincode::serialize(&unsigned_tx).unwrap();
    let base64_tx = base64::engine::general_purpose::STANDARD.encode(serialized_tx);
    let prepared_response = CreateContentResponse {
        transaction: base64_tx,
    };

    Ok(Json(prepared_response))
}
