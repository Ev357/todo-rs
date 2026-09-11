use dioxus::prelude::*;

use crate::{
    components::input::Input,
    icons::{loader::LoaderIcon, search::SearchIcon},
    todo_list::TodoList,
    Route,
};

#[component]
pub fn Home(search: String) -> Element {
    let mut input_text = use_signal(|| search.clone());
    let mut query = use_signal(|| search.clone());
    let mut debounce_task = use_signal(|| None::<dioxus_core::Task>);
    let nav = use_navigator();

    use_effect(use_reactive!(|(search)| {
        if *query.read() == search {
            return;
        }

        input_text.set(search.clone());
        query.set(search);
    }));

    let is_loading = debounce_task.read().is_some() || *input_text.read() != *query.read();

    rsx! {
        div { class: "flex min-h-screen w-full justify-center px-4 pt-8 pb-4",
            div { class: "flex w-full max-w-4xl flex-col gap-4",
                form {
                    action: "/",
                    method: "GET",
                    class: "relative flex w-full items-center",
                    onsubmit: move |event: FormEvent| {
                        event.prevent_default();
                    },
                    Input {
                        r#type: "search",
                        name: "search",
                        placeholder: "Search...",
                        class: "ps-8",
                        value: input_text,
                        onkeydown: move |event: KeyboardEvent| {
                            if let Key::Enter = event.key() {
                                event.prevent_default();
                            }
                        },
                        oninput: move |event: FormEvent| {
                            let value = event.value();
                            input_text.set(value.clone());

                            if let Some(task) = debounce_task.take() {
                                task.cancel();
                            }

                            let nav = nav;
                            let new_task = spawn(async move {
                                #[cfg(target_arch = "wasm32")]
                                gloo_timers::future::TimeoutFuture::new(300).await;

                                query.set(value.clone());
                                nav.replace(Route::Home { search: value });
                                debounce_task.set(None);
                            });

                            debounce_task.set(Some(new_task));
                        },
                    }
                    div {
                        class: "pointer-events-none absolute inset-y-0 start-0 flex items-center ps-2.5",
                        if is_loading {
                            LoaderIcon {
                                class: "size-4 shrink-0 animate-spin text-muted-foreground",
                            }
                        } else {
                            SearchIcon {
                                class: "size-4 shrink-0 text-muted-foreground",
                            }
                        }
                    }
                }

                TodoList {
                    search: query,
                    onadd: move |_| {
                        if let Some(task) = debounce_task.take() {
                            task.cancel();
                        }
                        input_text.set(String::new());
                        query.set(String::new());
                        nav.replace(Route::Home {
                            search: String::new(),
                        });
                    },
                }
            }
        }
    }
}
