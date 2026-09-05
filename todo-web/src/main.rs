use dioxus::prelude::*;

use crate::{components::input::Input, head_meta::HeadMeta, todo_list::TodoList};

mod components;
mod head_meta;
mod todo_list;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut search_term = use_signal(String::new);

    rsx! {
        HeadMeta {}

        div { class: "flex justify-center pt-8 bg-background text-foreground min-h-screen transition-colors",
            div { class: "flex flex-col items-center max-w-7xl w-full",
                div { class: "flex flex-col items-center max-w-xl w-full p-4 space-y-4",
                    h1 { class: "text-2xl font-bold text-center", "Todo App" }

                    Input {
                        oninput: move |e: FormEvent| search_term.set(e.value()),
                        placeholder: "Search todos...",
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
}
