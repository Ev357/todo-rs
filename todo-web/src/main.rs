use dioxus::prelude::*;

use crate::{components::input::Input, head_meta::HeadMeta, todo_list::TodoList};

mod components;
mod head_meta;
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

#[component]
fn App() -> Element {
    let mut search_term = use_signal(String::new);

    rsx! {
        HeadMeta {}

        div { class: "flex min-h-screen w-full justify-center px-4 pt-8 pb-4",
            div { class: "flex w-full max-w-4xl flex-col gap-4",
                Input {
                    oninput: move |e: FormEvent| search_term.set(e.value()),
                    placeholder: "Search...",
                    value: search_term,
                }

                SuspenseBoundary {
                    fallback: |_| rsx! { p { class: "text-muted-foreground text-sm", "Loading..." } },
                    TodoList { search: search_term }
                }
            }
        }
    }
}
