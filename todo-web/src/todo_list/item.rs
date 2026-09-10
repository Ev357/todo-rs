use dioxus::prelude::*;
use todo_server::db::todo::{Todo, Uuid};

use crate::{
    components::{
        button::{Button, ButtonSize, ButtonVariant},
        checkbox::Checkbox,
    },
    icons::trash::TrashIcon,
};

#[component]
pub fn TodoItem(todo: Todo, ontoggle: EventHandler<Uuid>, ondelete: EventHandler<Uuid>) -> Element {
    let todo_id = todo.id;
    let is_completed = todo.is_completed;
    let title = &todo.title;

    let title_class = if is_completed {
        "line-through text-muted-foreground"
    } else {
        ""
    };

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
                class: "absolute inset-0 size-full rounded-xl outline-none focus-visible:ring-2 focus-visible:ring-ring",
                onclick: move |event| {
                    event.prevent_default();
                    ontoggle.call(todo_id);
                },
            }

            div { class: "flex min-w-0 grow items-center gap-4",
                Checkbox {
                    checked: is_completed,
                    class: "pointer-events-none",
                    tabindex: "-1",
                    "aria-hidden": "true",
                    title: if is_completed { "Mark incomplete" } else { "Mark complete" },
                }
                p { class: title_class, {title.as_str()} }
            }

            Button {
                r#type: "submit",
                name: "_method",
                value: "DELETE",
                "aria-label": "Delete {title}",
                variant: ButtonVariant::Destructive,
                size: ButtonSize::IconSm,
                class: "invisible z-10 group-hover:visible group-focus-within:visible focus-visible:visible",
                onclick: move |event: MouseEvent| {
                    event.prevent_default();
                    event.stop_propagation();
                    ondelete.call(todo_id);
                },
                TrashIcon {}
            }
        }
    }
}
