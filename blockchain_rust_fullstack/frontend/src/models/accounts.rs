use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Task {
    pub user: String,
    pub bump: u8,
    pub todos: Vec<Todo>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Todo {
    pub content: String,
    pub is_completed: bool,
}
