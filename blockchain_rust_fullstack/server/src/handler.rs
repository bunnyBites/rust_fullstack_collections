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

use crate::model::Task;

const PROGRAM_ID: &str = "4sXU7Z9U3N1TFWiTU9ZZ9R5XAoiL633JGsQfCFKdTYQF";

pub async fn get_todos(
    Path(user_public_key): Path<String>,
    State(rpc_client): State<Arc<RpcClient>>,
) -> Result<Json<Task>, impl IntoResponse> {
    println!("Provided user public key: {}", user_public_key);

    let user_pk = match Pubkey::from_str(&user_public_key) {
        Ok(pk) => pk,
        Err(_) => return Err((StatusCode::NOT_FOUND, "Invalid user key format")),
    };

    let program_id = match Pubkey::from_str(PROGRAM_ID) {
        Ok(program_pk) => program_pk,
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Provided invalid program id",
            ));
        }
    };

    let (todo_pda, _bump) = Pubkey::find_program_address(&[b"list", user_pk.as_ref()], &program_id);

    match rpc_client.get_account_data(&todo_pda) {
        Ok(account_data_raw) => match Task::deserialize(&mut account_data_raw.as_slice()) {
            Ok(todo_list) => Ok(Json(todo_list)),
            Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to deserialize")),
        },
        Err(_) => Err((StatusCode::BAD_REQUEST, "Failed to fetch data")),
    }
}
