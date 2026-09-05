use axum::Router;

use crate::api_context::ApiContext;

pub mod todo;

pub fn router() -> Router<ApiContext> {
    Router::new().nest("/todo", todo::router())
}
