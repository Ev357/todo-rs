use crate::define_entity;

define_entity!(
    CreateTodo,
    PatchTodo,
    Todo {
        id: i64,
        title: String,
        is_completed: bool,
        created_at: i64,
    }
);
