use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use sqlx::QueryBuilder;
use uuid::Uuid;

use crate::{
    api_context::ApiContext,
    db::todo::{CreateTodo, Todo, TodoQuery},
};

pub mod id;

pub fn router() -> Router<ApiContext> {
    Router::new()
        .route("/", get(get_todos).post(post_todo).query(query_todos))
        .nest("/{id}", id::router())
}

#[axum::debug_handler]
async fn get_todos(
    State(ApiContext { db }): State<ApiContext>,
) -> Result<Json<Vec<Todo>>, StatusCode> {
    let todos = sqlx::query_as!(
        Todo,
        r#"
        SELECT id as "id!: Uuid", title, is_completed, created_at
        FROM todos
        ORDER BY is_completed ASC, created_at DESC
        "#
    )
    .fetch_all(&db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(todos))
}

#[axum::debug_handler]
async fn query_todos(
    State(ApiContext { db }): State<ApiContext>,
    Json(payload): Json<TodoQuery>,
) -> Result<Json<Vec<Todo>>, StatusCode> {
    let mut builder =
        QueryBuilder::new("SELECT id, title, is_completed, created_at FROM todos WHERE 1 = 1 ");

    if let Some(ref search) = payload.search {
        builder.push(" AND title LIKE ");
        builder.push_bind(format!("%{search}%"));
    }

    if let Some(completed) = payload.is_completed {
        builder.push(" AND is_completed = ");
        builder.push_bind(completed);
    }

    builder.push(" ORDER BY is_completed ASC, created_at DESC");

    let todos = builder
        .build_query_as::<Todo>()
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
    let title = payload.title.trim();
    if title.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let id = Uuid::new_v4();
    let todo = sqlx::query_as!(
        Todo,
        r#"
        INSERT INTO todos (id, title, is_completed)
        VALUES (?, ?, ?)
        RETURNING id as "id!: Uuid", title, is_completed, created_at
        "#,
        id,
        title,
        payload.is_completed
    )
    .fetch_one(&db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(todo)))
}
