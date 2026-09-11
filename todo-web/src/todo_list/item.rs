use dioxus::prelude::*;
use todo_server::db::todo::{Todo, Uuid};

use crate::{
    components::{
        button::{Button, ButtonSize, ButtonVariant},
        checkbox::Checkbox,
    },
    icons::{loader::LoaderIcon, trash::TrashIcon},
};

#[component]
pub fn TodoItem(
    todo: Todo,
    ontoggle: EventHandler<Uuid>,
    ondelete: EventHandler<Uuid>,
    onedit: EventHandler<(Uuid, String)>,
    #[props(default = false)] is_updating: bool,
    #[props(default = false)] is_deleting: bool,
) -> Element {
    let todo_id = todo.id;
    let is_completed = todo.is_completed;
    let initial_title = todo.title.clone();
    let mut current_title = use_signal(|| initial_title.clone());

    let title_class = if is_completed {
        "line-through text-muted-foreground"
    } else {
        "text-foreground"
    };

    let delete_visibility = if is_deleting { "visible" } else { "invisible" };

    let commit_edit = use_callback(move |()| {
        let value = current_title.read().trim().to_string();
        if !value.is_empty() && value != initial_title {
            onedit.call((todo_id, value));
        } else {
            current_title.set(initial_title.clone());
        }
    });

    rsx! {
        form {
            action: "/api/todo/{todo_id}",
            method: "POST",
            role: "listitem",
            class: "group flex items-center gap-4 rounded-xl border border-border bg-card p-4 transition hover:bg-accent/40 focus-within:bg-accent/40",
            onsubmit: move |event: FormEvent| {
                event.prevent_default();
                commit_edit.call(());
            },

            input {
                r#type: "hidden",
                name: "_method",
                value: "PATCH",
            }

            input {
                r#type: "hidden",
                name: "data[is_completed]",
                value: if is_completed { "true" } else { "false" },
            }

            div { class: "flex shrink-0 items-center",
                if !is_updating {
                    Checkbox {
                        checked: is_completed,
                        r#type: "submit",
                        name: "data[is_completed]",
                        value: if is_completed { "false" } else { "true" },
                        title: if is_completed { "Mark incomplete" } else { "Mark complete" },
                        disabled: is_updating || is_deleting,
                        onclick: move |event: MouseEvent| {
                            event.prevent_default();
                            ontoggle.call(todo_id);
                        },
                    }
                } else {
                    LoaderIcon {
                        class: "size-4 shrink-0 animate-spin text-muted-foreground",
                    }
                }
            }

            input {
                r#type: "text",
                name: "data[title]",
                value: current_title,
                class: "min-w-0 grow bg-transparent text-sm outline-none border-none p-0 focus:ring-0 placeholder:text-muted-foreground {title_class}",
                oninput: move |event: FormEvent| current_title.set(event.value()),
                onblur: move |_| commit_edit.call(()),
                onkeydown: move |event: KeyboardEvent| {
                    if let Key::Enter = event.key() {
                        event.prevent_default();
                        commit_edit.call(());
                    }
                },
            }

            Button {
                r#type: "submit",
                name: "_method",
                value: "DELETE",
                "aria-label": "Delete {todo.title}",
                disabled: is_updating || is_deleting,
                variant: ButtonVariant::Destructive,
                size: ButtonSize::IconSm,
                class: "z-10 group-hover:visible group-focus-within:visible focus-visible:visible {delete_visibility}",
                onclick: move |event: MouseEvent| {
                    event.prevent_default();
                    event.stop_propagation();
                    ondelete.call(todo_id);
                },
                if !is_deleting {
                    TrashIcon {}
                } else {
                    LoaderIcon {
                        class: "size-4 shrink-0 animate-spin",
                    }
                }
            }
        }
    }
}
