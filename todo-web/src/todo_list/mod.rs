use dioxus::prelude::*;
use indexmap::IndexMap;
use todo_api::api::todo::{
    id::{delete_todo, patch_todo},
    post_todos, query_todos,
};
use todo_server::db::todo::{CreateTodo, PatchTodo, Todo, TodoQuery, Uuid};

use crate::{
    components::skeleton::Skeleton,
    todo_list::{add::AddItem, item::TodoItem},
};

pub mod add;
mod item;

#[component]
pub fn TodoList(search: ReadSignal<String>) -> Element {
    let mut add_action = use_action(post_todos);

    let handle_add = use_callback(move |title: String| {
        add_action.call(CreateTodo {
            title,
            ..Default::default()
        });
    });

    rsx! {
        div {
            class: "flex flex-col gap-2",
            role: "list",
            "aria-label": "Todo list",
            AddItem { onadd: handle_add },
            SuspenseBoundary {
                fallback: |_| rsx! {
                    for index in 0..4 {
                        Skeleton { key: "{index}", class: "h-16 rounded-xl" }
                    }
                },
                TodoListData { search, add_action }
            }
        }
    }
}

#[component]
fn TodoListData(search: ReadSignal<String>, add_action: Action<(CreateTodo,), Todo>) -> Element {
    let mut todos = use_server_future(move || {
        let _ = add_action.value();

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
                .collect::<IndexMap<Uuid, Todo>>()
        }
    })?;

    let mut update_todo = use_action(patch_todo);
    let mut remove_todo = use_action(delete_todo);

    let handle_toggle = use_callback(move |todo_id: Uuid| {
        let current_completed = todos
            .read()
            .as_ref()
            .and_then(|map| map.get(&todo_id).map(|t| t.is_completed));

        let next_completed = match current_completed {
            Some(completed) => !completed,
            None => return,
        };

        let fut = update_todo.call(
            todo_id,
            PatchTodo {
                is_completed: Some(next_completed),
                ..Default::default()
            },
        );

        spawn(async move {
            fut.await;
            if matches!(update_todo.value(), Some(Ok(_))) {
                todos.restart();
            }
        });
    });

    let handle_delete = use_callback(move |todo_id: Uuid| {
        let fut = remove_todo.call(todo_id);
        spawn(async move {
            fut.await;
            if matches!(remove_todo.value(), Some(Ok(_))) {
                todos.restart();
            }
        });
    });

    let todos = todos.suspend()?;
    let todos = todos.read();
    rsx! {
        if !todos.is_empty() {
            for (id, todo) in todos.iter() {
                TodoItem {
                    key: "{id}",
                    todo: todo.clone(),
                    ontoggle: handle_toggle,
                    ondelete: handle_delete,
                }
            }
        } else {
            p { class: "text-muted-foreground text-sm", "No todos found." }
        }
    }
}
