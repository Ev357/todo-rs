use std::error::Error;

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{head_meta::HeadMeta, home::Home, search_query::SearchQuery};

mod components;
#[cfg(feature = "server")]
mod config;
mod head_meta;
mod home;
mod icons;
#[cfg(feature = "server")]
mod method_spoofing_layer;
mod search_query;
mod todo_list;

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(feature = "server")]
    {
        use dioxus::{
            server,
            server::axum::{Extension, Router},
        };
        use todo_api::ApiClient;
        use tower::Layer;

        use crate::{config::Config, method_spoofing_layer::MethodSpoofingLayer};

        let config = Config::load(".env")?;

        dioxus::serve(move || {
            let api_client = ApiClient::new(config.api_url.clone());

            async move {
                let app_router = server::router(App).layer(Extension(api_client));

                let spoofed_service = Layer::layer(&MethodSpoofingLayer, app_router);

                Ok(Router::new().fallback_service(spoofed_service))
            }
        });
    }

    #[cfg(not(feature = "server"))]
    {
        dioxus::launch(App);
        Ok(())
    }
}

#[derive(Clone, Routable, Debug, PartialEq, Serialize, Deserialize)]
pub enum Route {
    #[route("/?:..search")]
    Home { search: SearchQuery },
}

#[component]
fn App() -> Element {
    rsx! {
        HeadMeta {}
        Router::<Route> {}
    }
}
