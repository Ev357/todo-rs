use axum::Router;

use crate::ApiContext;

pub mod api;

pub fn router() -> Router<ApiContext> {
    Router::new().nest("/api", api::router())
}
