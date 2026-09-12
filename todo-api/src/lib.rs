#[cfg(feature = "server")]
pub mod client;
#[cfg(feature = "server")]
pub use client::ApiClient;

pub mod api;
