use dioxus::prelude::*;
use indexmap::IndexMap;
use todo_api::api::todo::{
    id::{delete_todo, patch_todo},
    query_todos,
};
use todo_server::db::todo::{PatchTodo, Todo, TodoQuery};

use crate::todo_list::item::TodoItem;

mod item;

#[component]
pub fn TodoList(search: ReadSignal<String>) -> Element {
    let mut todos = use_server_future(move || {
        let query_str = search.read().clone();
        let search_arg = (!query_str.trim().is_empty()).then_some(query_str);

        async move {
            let list = query_todos(TodoQuery {
                search: search_arg,
                is_completed: None,
            })
            .await
            .unwrap_or_default();

            list.into_iter()
                .map(|todo| (todo.id, todo))
                .collect::<IndexMap<i64, Todo>>()
        }
    })?;

    let handle_toggle = move |todo_id: i64| {
        let prev_completed = mutate_todo(&mut todos, todo_id, |item| {
            let prev = item.is_completed;
            item.is_completed = !prev;
            prev
        });

        if let Some(prev) = prev_completed {
            let next_completed = !prev;

            spawn(async move {
                let update = PatchTodo {
                    is_completed: Some(next_completed),
                    ..Default::default()
                };

                match patch_todo(todo_id, update).await {
                    Ok(authoritative) => mutate_todo(&mut todos, todo_id, |t| *t = authoritative),
                    Err(_) => mutate_todo(&mut todos, todo_id, |t| t.is_completed = prev),
                };
            });
        }
    };

    let handle_delete = move |todo_id: i64| {
        let removed = todos.write().as_mut().and_then(|map| {
            let index = map.get_index_of(&todo_id)?;
            let (_, todo) = map.shift_remove_entry(&todo_id)?;
            Some((index, todo))
        });

        if let Some((original_index, snapshot)) = removed {
            spawn(async move {
                if delete_todo(todo_id).await.is_err() {
                    if let Some(map) = todos.write().as_mut() {
                        let idx = original_index.min(map.len());
                        map.shift_insert(idx, todo_id, snapshot);
                    }
                }
            });
        }
    };

    let todos_lock = todos.read();
    match todos_lock.as_ref() {
        Some(items) if !items.is_empty() => rsx! {
            div {
                class: "flex flex-col gap-2",
                role: "list",
                "aria-label": "Todo list",
                for (id, todo) in items.iter() {
                    TodoItem {
                        key: "{id}",
                        todo: todo.clone(),
                        ontoggle: handle_toggle,
                        ondelete: handle_delete,
                    }
                }
            }
        },
        Some(_) => rsx! { p { class: "text-muted-foreground text-sm", "No todos found." } },
        None => rsx! { p { class: "text-muted-foreground text-sm", "Loading..." } },
    }
}

fn mutate_todo<R>(
    todos: &mut Resource<IndexMap<i64, Todo>>,
    id: i64,
    f: impl FnOnce(&mut Todo) -> R,
) -> Option<R> {
    todos.write().as_mut()?.get_mut(&id).map(f)
}
