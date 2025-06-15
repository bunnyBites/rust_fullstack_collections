use anchor_client::anchor_lang::AnchorDeserialize;
use serde::Serialize;

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
