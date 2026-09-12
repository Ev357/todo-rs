pub mod delete;
pub mod get;
pub mod patch;
pub mod post;
pub mod query;

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
