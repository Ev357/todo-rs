use dioxus::prelude::*;

mod views;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        div {
            p { "Hello World!" }
        }
    }
}
