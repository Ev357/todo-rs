use dioxus::prelude::*;
use todo_server::db::todo::{PatchTodo, Todo, Uuid};

#[cfg(feature = "server")]
use crate::client::get_client;

#[patch("/api/todo/{id}")]
pub async fn patch_todo(id: Uuid, data: PatchTodo) -> Result<Todo, ServerFnError> {
    let api = get_client().await?;

    let response = api
        .client
        .patch(api.url(&format!("/api/todo/{id}")))
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
pub async fn delete_todo(id: Uuid) -> Result<(), ServerFnError> {
    let api = get_client().await?;

    api.client
        .delete(api.url(&format!("/api/todo/{id}")))
        .send()
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?
        .error_for_status()
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    Ok(())
}
