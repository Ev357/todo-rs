use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};

use crate::{
    api_context::ApiContext,
    db::todo::{CreateTodo, PatchTodo, Todo},
};

pub fn router() -> Router<ApiContext> {
    Router::new().route(
        "/",
        get(get_todo)
            .put(put_todo)
            .patch(patch_todo)
            .delete(delete_todo),
    )
}

#[axum::debug_handler]
async fn get_todo(
    State(ApiContext { db }): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<Json<Todo>, StatusCode> {
    let todo = sqlx::query_as!(
        Todo,
        r#"
        SELECT id, title, is_completed, created_at
        FROM todos
        WHERE id = ?
        "#,
        id
    )
    .fetch_optional(&db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(todo))
}

#[axum::debug_handler]
async fn put_todo(
    State(ApiContext { db }): State<ApiContext>,
    Path(id): Path<i64>,
    Json(payload): Json<CreateTodo>,
) -> Result<Json<Todo>, StatusCode> {
    let todo = sqlx::query_as!(
        Todo,
        r#"
        UPDATE todos
        SET
            title = ?,
            is_completed = ?
        WHERE id = ?
        RETURNING id, title, is_completed, created_at
        "#,
        payload.title,
        payload.is_completed,
        id
    )
    .fetch_optional(&db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(todo))
}

#[axum::debug_handler]
async fn patch_todo(
    State(ApiContext { db }): State<ApiContext>,
    Path(id): Path<i64>,
    Json(payload): Json<PatchTodo>,
) -> Result<Json<Todo>, StatusCode> {
    let todo = sqlx::query_as!(
        Todo,
        r#"
        UPDATE todos
        SET
            title = COALESCE(?, title),
            is_completed = COALESCE(?, is_completed)
        WHERE id = ?
        RETURNING id, title, is_completed, created_at
        "#,
        payload.title,
        payload.is_completed,
        id
    )
    .fetch_optional(&db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(todo))
}

#[axum::debug_handler]
async fn delete_todo(
    State(ApiContext { db }): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query!(
        r#"
        DELETE FROM todos
        WHERE id = ?
        "#,
        id
    )
    .execute(&db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}
