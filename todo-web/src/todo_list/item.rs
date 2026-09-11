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
    is_updating: bool,
    is_deleting: bool,
) -> Element {
    let todo_id = todo.id;
    let is_completed = todo.is_completed;
    let title = &todo.title;

    let title_class = if is_completed {
        "line-through text-muted-foreground"
    } else {
        ""
    };

    let delete_visibility = if is_deleting { "visible" } else { "invisible" };

    rsx! {
        form {
            action: "/api/todo/{todo_id}",
            method: "POST",
            role: "listitem",
            class: "group relative flex items-center gap-4 rounded-xl border border-border bg-card p-4 transition hover:bg-accent hover:text-accent-foreground",
            onsubmit: move |event| {
                event.prevent_default();
            },

            input {
                r#type: "hidden",
                name: "data[is_completed]",
                value: if is_completed { "false" } else { "true" },
            }

            button {
                r#type: "submit",
                role: "checkbox",
                "aria-checked": if is_completed { "true" } else { "false" },
                "aria-label": title.as_str(),
                name: "_method",
                value: "PATCH",
                disabled: is_updating || is_deleting,
                class: "absolute inset-0 size-full rounded-xl outline-none focus-visible:ring-2 focus-visible:ring-ring",
                onclick: move |event| {
                    event.prevent_default();
                    ontoggle.call(todo_id);
                },
            }

            div { class: "flex min-w-0 grow items-center gap-4",
                if !is_updating {
                    Checkbox {
                        checked: is_completed,
                        class: "pointer-events-none",
                        tabindex: "-1",
                        "aria-hidden": "true",
                        title: if is_completed { "Mark incomplete" } else { "Mark complete" },
                    }
                } else {
                    LoaderIcon {
                        class: "size-4 shrink-0 animate-spin text-muted-foreground",
                    }
                }
                p { class: title_class, {title.as_str()} }
            }

            Button {
                r#type: "submit",
                name: "_method",
                value: "DELETE",
                "aria-label": "Delete {title}",
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
