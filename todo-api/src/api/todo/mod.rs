use dioxus::prelude::*;
use todo_server::db::todo::{CreateTodo, Todo, TodoQuery};

pub mod id;

#[get("/api/todo")]
pub async fn get_todos() -> Result<Vec<Todo>, ServerFnError> {
    let response = reqwest::get("http://localhost:3000/api/todo")
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    let todos = response
        .json::<Vec<Todo>>()
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    Ok(todos)
}

#[query("/api/todo")]
pub async fn query_todos(query: TodoQuery) -> Result<Vec<Todo>, ServerFnError> {
    let client = reqwest::Client::new();

    let response = client
        .request(reqwest::Method::QUERY, "http://localhost:3000/api/todo")
        .json(&query)
        .send()
        .await
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

    let client = reqwest::Client::new();

    let response = client
        .post("http://localhost:3000/api/todo")
        .json(&data)
        .send()
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    let todo = response
        .json::<Todo>()
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    Ok(todo)
}
