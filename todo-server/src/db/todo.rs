use serde::{Deserialize, Serialize};
pub use uuid::Uuid;

use crate::define_entity;

define_entity!(
    CreateTodo,
    PatchTodo,
    Todo {
        id: Uuid,
        title: String,
        is_completed: bool,
        #[server]
        created_at: i64,
    }
);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TodoQuery {
    pub search: Option<String>,
    pub is_completed: Option<bool>,
}
