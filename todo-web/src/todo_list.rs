use dioxus::prelude::*;
use todo_api::api::todo::query_todos;
use todo_server::db::todo::TodoQuery;

#[component]
pub fn TodoList(search: ReadSignal<String>) -> Element {
    let todos = use_server_future(move || {
        let query_str = search.read().clone();
        let search_arg = if query_str.trim().is_empty() {
            None
        } else {
            Some(query_str)
        };

        async move {
            query_todos(TodoQuery {
                search: search_arg,
                is_completed: None,
            })
            .await
            .unwrap_or_default()
        }
    })?;

    let todos_lock = todos.read();

    match todos_lock.as_deref() {
        Some(items) if !items.is_empty() => rsx! {
            ul { class: "w-full space-y-2",
                for todo in items.iter() {
                    li {
                        key: "{todo.id}",
                        class: "flex items-center justify-between p-3 bg-card text-card-foreground border border-border rounded-lg shadow-sm",
                        span {
                            class: if todo.is_completed { "line-through text-muted-foreground" } else { "" },
                            "{todo.title}"
                        }
                    }
                }
            }
        },
        Some(_) => rsx! {
            p { class: "text-muted-foreground text-sm", "No todos found." }
        },
        None => rsx! {
            p { class: "text-muted-foreground text-sm", "Loading..." }
        },
    }
}
