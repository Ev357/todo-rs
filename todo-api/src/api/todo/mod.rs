use dioxus::prelude::*;
use todo_server::db::todo::{CreateTodo, Todo, TodoQuery};

#[cfg(feature = "server")]
use crate::client::get_client;

pub mod id;

#[get("/api/todo")]
pub async fn get_todos() -> Result<Vec<Todo>, ServerFnError> {
    let api = get_client().await?;

    let response = api
        .client
        .get(api.url("/api/todo"))
        .send()
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?
        .error_for_status()
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    let todos = response
        .json::<Vec<Todo>>()
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    Ok(todos)
}

#[query("/api/todo")]
pub async fn query_todos(query: TodoQuery) -> Result<Vec<Todo>, ServerFnError> {
    let api = get_client().await?;

    let response = api
        .client
        .request(reqwest::Method::QUERY, api.url("/api/todo"))
        .json(&query)
        .send()
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?
        .error_for_status()
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    let todos = response
        .json::<Vec<Todo>>()
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    Ok(todos)
}

#[post("/api/todo")]
pub async fn post_todos(data: CreateTodo) -> Result<Todo, ServerFnError> {
    if data.title.trim().is_empty() {
        return Err(ServerFnError::new("Title cannot be empty"));
    }

    let api = get_client().await?;

    let response = api
        .client
        .post(api.url("/api/todo"))
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
