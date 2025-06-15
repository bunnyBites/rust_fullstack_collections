use anchor_client::anchor_lang::AnchorDeserialize;
use serde::{Deserialize, Serialize};

#[derive(AnchorDeserialize, Serialize, Debug, Clone)]
pub struct TaskAPIResponse {
    pub user: String,
    pub bump: u8,
    pub todos: Vec<TodoAPIResponse>,
}

#[derive(AnchorDeserialize, Serialize, Debug, Clone)]
pub struct TodoAPIResponse {
    pub content: String,
    pub is_completed: bool,
}

#[derive(Deserialize)]
pub struct CreateContent {
    pub user_public_key: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct CreateContentResponse {
    pub transaction: String,
}
