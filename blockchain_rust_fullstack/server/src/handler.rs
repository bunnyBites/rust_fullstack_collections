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
use std::{str::FromStr, sync::Arc};

use crate::{
    PROGRAM_ID,
    model::{TaskAPIResponse, TodoAPIResponse},
};
use blockchain::Todo;

type SharedProgram = State<Arc<Program<Arc<Keypair>>>>;

pub async fn get_todos(
    Path(user_pubkey): Path<String>,
    program_state: SharedProgram,
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
