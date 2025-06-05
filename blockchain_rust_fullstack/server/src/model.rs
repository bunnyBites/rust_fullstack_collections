use anchor_lang::{AnchorDeserialize, prelude::Pubkey, prelude::borsh};
use serde::Serialize;

#[derive(Clone, Debug, AnchorDeserialize, Serialize)]
pub struct Todo {
    content: String,
    is_completed: bool,
}

#[derive(AnchorDeserialize, Serialize, Clone, Debug)]
pub struct Task {
    pub user: Pubkey,
    pub bump: u8,
    pub todos: Vec<Todo>,
}
