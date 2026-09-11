use dioxus::prelude::*;
use indexmap::IndexMap;
use todo_api::api::todo::{
    id::{delete_todo, patch_todo},
    post_todos, query_todos,
};
use todo_server::db::todo::{CreateTodo, PatchTodo, Todo, TodoQuery, Uuid};

use crate::{
    components::skeleton::Skeleton,
    icons::inbox::InboxIcon,
    todo_list::{add::AddItem, item::TodoItem},
};

pub mod add;
mod item;

#[component]
pub fn TodoList(search: ReadSignal<String>, onadd: EventHandler<()>) -> Element {
    let mut add_action = use_action(post_todos);

    let handle_add = use_callback(move |title: String| {
        add_action.call(CreateTodo {
            title,
            ..Default::default()
        });
        onadd.call(());
    });

    rsx! {
        div {
            class: "flex flex-col gap-2",
            role: "list",
            "aria-label": "Todo list",
            AddItem {
                onadd: handle_add,
                pending: add_action.pending(),
            },
            SuspenseBoundary {
                fallback: |_| rsx! {
                    for index in 0..4 {
                        Skeleton { key: "{index}", class: "h-[66px] rounded-xl" }
                    }
                },
                TodoListData { search, add_action }
            }
        }
    }
}

#[component]
fn TodoListData(search: ReadSignal<String>, add_action: Action<(CreateTodo,), Todo>) -> Element {
    let mut cached_resource = use_signal(|| None::<Resource<IndexMap<Uuid, Todo>>>);
    let mut updating_id = use_signal(|| None::<Uuid>);
    let mut deleting_id = use_signal(|| None::<Uuid>);

    let res = use_server_future(move || {
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
    });

    let mut todos = match res {
        Ok(todos) => {
            cached_resource.set(Some(todos));
            todos
        }
        Err(error) => match *cached_resource.read() {
            Some(todos) => todos,
            None => return Err(error),
        },
    };

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

        updating_id.set(Some(todo_id));
        let future = update_todo.call(
            todo_id,
            PatchTodo {
                is_completed: Some(next_completed),
                ..Default::default()
            },
        );

        spawn(async move {
            future.await;
            if matches!(update_todo.value(), Some(Ok(_))) {
                todos.restart();
            }
            updating_id.set(None);
        });
    });

    let handle_delete = use_callback(move |todo_id: Uuid| {
        deleting_id.set(Some(todo_id));
        let future = remove_todo.call(todo_id);
        spawn(async move {
            future.await;
            if matches!(remove_todo.value(), Some(Ok(_))) {
                todos.restart();
            }
            deleting_id.set(None);
        });
    });

    let todos_read = todos.read();
    let items = todos_read.as_ref();
    let active_updating = *updating_id.read();
    let active_deleting = *deleting_id.read();

    rsx! {
        if let Some(items) = items {
            if !items.is_empty() {
                for (id, todo) in items.iter() {
                    TodoItem {
                        key: "{id}",
                        todo: todo.clone(),
                        ontoggle: handle_toggle,
                        ondelete: handle_delete,
                        is_updating: active_updating == Some(*id),
                        is_deleting: active_deleting == Some(*id),
                    }
                }
            } else {
                div {
                    class: "flex flex-col items-center justify-center gap-2 py-12 text-center text-muted-foreground",
                    InboxIcon {
                        class: "size-10 text-muted-foreground/60",
                    }
                    p {
                        class: "text-sm font-medium",
                        "No todos found"
                    }
                }
            }
        }
    }
}
