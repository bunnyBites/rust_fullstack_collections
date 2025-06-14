use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Task {
    user: String,
    bump: u8,
    todos: Vec<Todo>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Todo {
    content: String,
    is_completed: bool,
}
