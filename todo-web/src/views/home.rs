use dioxus::prelude::*;
use todo_ui::{Echo, Hero};

#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
        Echo {}
    }
}
