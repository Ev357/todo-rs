#[cfg(feature = "server")]
use dioxus::server::ServerFnError;
use reqwest::Client;

#[derive(Clone, Debug)]
pub struct ApiClient {
    pub client: Client,
    pub base_url: String,
}

impl ApiClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.into().trim_end_matches('/').to_string(),
        }
    }

    pub fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }
}

#[cfg(feature = "server")]
pub async fn get_client() -> Result<ApiClient, ServerFnError> {
    use dioxus::{fullstack::FullstackContext, server::axum::Extension};

    FullstackContext::extract::<Extension<crate::ApiClient>, _>()
        .await
        .map(|Extension(client)| client)
        .map_err(|error| ServerFnError::new(error.to_string()))
}
