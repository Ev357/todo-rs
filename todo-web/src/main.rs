use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{head_meta::HeadMeta, home::Home};

mod components;
mod head_meta;
mod home;
mod icons;
#[cfg(feature = "server")]
mod method_spoofing_layer;
mod todo_list;

fn main() {
    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        use dioxus::{server, server::axum::Router};
        use tower::Layer;

        use crate::method_spoofing_layer::MethodSpoofingLayer;

        let app_router = server::router(App);

        let spoofed_service = Layer::layer(&MethodSpoofingLayer, app_router);

        Ok(Router::new().fallback_service(spoofed_service))
    });

    #[cfg(not(feature = "server"))]
    dioxus::launch(App);
}

#[derive(Clone, Routable, Debug, PartialEq, Serialize, Deserialize)]
pub enum Route {
    #[route("/?:search")]
    Home { search: String },
}

#[component]
fn App() -> Element {
    rsx! {
        HeadMeta {}
        Router::<Route> {}
    }
}
