use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};

use crate::{
    api_context::ApiContext,
    db::todo::{CreateTodo, Todo},
};

mod id;

pub fn router() -> Router<ApiContext> {
    Router::new()
        .route("/", get(get_todos))
        .route("/", post(post_todo))
        .nest("/{id}", id::router())
}

#[axum::debug_handler]
async fn get_todos(
    State(ApiContext { db }): State<ApiContext>,
) -> Result<Json<Vec<Todo>>, StatusCode> {
    let todos = sqlx::query_as!(
        Todo,
        r#"
        SELECT id, title, is_completed, created_at
        FROM todos
        ORDER BY created_at
        "#
    )
    .fetch_all(&db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(todos))
}

#[axum::debug_handler]
async fn post_todo(
    State(ApiContext { db }): State<ApiContext>,
    Json(payload): Json<CreateTodo>,
) -> Result<(StatusCode, Json<Todo>), StatusCode> {
    let todo = sqlx::query_as!(
        Todo,
        r#"
        INSERT INTO todos (title, is_completed, created_at)
        VALUES (?, ?, ?)
        RETURNING id, title, is_completed, created_at
        "#,
        payload.title,
        payload.is_completed,
        payload.created_at
    )
    .fetch_one(&db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(todo)))
}
