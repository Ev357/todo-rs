use dioxus::prelude::*;
use todo_server::db::todo::{PatchTodo, Todo};

#[patch("/api/todo/{id}")]
pub async fn patch_todo(id: i64, data: PatchTodo) -> Result<Todo, ServerFnError> {
    let client = reqwest::Client::new();

    let response = client
        .patch(format!("http://localhost:3000/api/todo/{id}"))
        .json(&data)
        .send()
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?
        .error_for_status()
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    let todo = response
        .json::<Todo>()
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    Ok(todo)
}

#[delete("/api/todo/{id}")]
pub async fn delete_todo(id: i64) -> Result<(), ServerFnError> {
    let client = reqwest::Client::new();

    client
        .delete(format!("http://localhost:3000/api/todo/{id}"))
        .send()
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?
        .error_for_status()
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    Ok(())
}
