use anchor_lang::prelude::{AnchorDeserialize, Pubkey, borsh};
use serde::Serialize;

#[derive(AnchorDeserialize, Serialize, Debug, Clone)]
pub struct Task {
    pub user: Pubkey,
    pub bump: u8,
    pub todos: Vec<Todo>,
}

#[derive(AnchorDeserialize, Serialize, Debug, Clone)]
pub struct Todo {
    pub content: String,
    pub is_completed: bool,
}
