use axum::Router;

use crate::api_context::ApiContext;

pub mod api;

pub fn router() -> Router<ApiContext> {
    Router::new().nest("/api", api::router())
}
