use dioxus::prelude::*;

use crate::{
    components::button::{Button, ButtonSize, ButtonVariant},
    icons::{loader::LoaderIcon, plus::PlusIcon},
};

#[component]
pub fn AddItem(onadd: EventHandler<String>, pending: bool) -> Element {
    let mut title = use_signal(String::new);

    rsx! {
        form {
            action: "/api/todo",
            method: "POST",
            class: "group flex items-center gap-4 rounded-xl border border-dashed border-border bg-card p-4 transition focus-within:border-foreground/30",
            onsubmit: move |event: FormEvent| {
                event.prevent_default();
                if pending {
                    return;
                }
                let trimmed = title.read().trim().to_string();
                if !trimmed.is_empty() {
                    onadd.call(trimmed);
                    title.set(String::new());
                }
            },

            input {
                r#type: "hidden",
                name: "_redirect",
                value: "/",
            }

            input {
                r#type: "hidden",
                name: "data[is_completed]",
                value: "false",
            }

            input {
                r#type: "text",
                name: "data[title]",
                placeholder: "Add a new task...",
                value: title,
                required: true,
                class: "min-w-0 grow bg-transparent text-sm placeholder:text-muted-foreground outline-none",
                oninput: move |event: FormEvent| title.set(event.value()),
            }

            Button {
                r#type: "submit",
                "aria-label": "Add task",
                variant: ButtonVariant::Outline,
                size: ButtonSize::IconSm,
                disabled: pending,
                class: "invisible group-focus-within:visible group-hover:visible focus-visible:visible",
                if !pending {
                    PlusIcon {}
                } else {
                    LoaderIcon {
                        class: "size-4 shrink-0 animate-spin",
                    }
                }
            }
        }
    }
}
