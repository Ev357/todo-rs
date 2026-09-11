use dioxus::prelude::*;

use crate::icons::{check::CheckIcon, minus::MinusIcon};

const BASE_STYLES: &str = "\
    flex size-4 items-center justify-center rounded-[4px] border border-input shadow-xs \
    transition-shadow group-has-disabled/field:opacity-50 \
    group-has-[:focus-visible]/field-label:ring-0 \
    group-has-[:focus-visible]/field-label:not-data-checked:border-input \
    focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50 \
    aria-invalid:border-destructive aria-invalid:ring-3 aria-invalid:ring-destructive/20 \
    aria-invalid:aria-checked:border-primary \
    dark:bg-input/30 dark:aria-invalid:border-destructive/50 dark:aria-invalid:ring-destructive/40 \
    data-checked:border-primary data-checked:bg-primary data-checked:text-primary-foreground \
    group-has-[:focus-visible]/field-label:data-checked:border-primary \
    dark:data-checked:bg-primary peer relative shrink-0 outline-none \
    after:absolute after:-inset-x-3 after:-inset-y-2 \
    disabled:cursor-not-allowed disabled:opacity-50";

#[component]
pub fn Checkbox(
    class: Option<String>,
    #[props(default = false)] checked: bool,
    #[props(default = false)] indeterminate: bool,
    #[props(default = "button".to_string())] r#type: String,
    #[props(extends = GlobalAttributes)]
    #[props(extends = button)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let class = match class {
        Some(custom) => format!("{BASE_STYLES} {custom}"),
        None => BASE_STYLES.to_string(),
    };

    let data_state = if indeterminate {
        "indeterminate"
    } else if checked {
        "checked"
    } else {
        "unchecked"
    };

    let button_type = r#type;

    rsx! {
        button {
            r#type: button_type,
            role: "checkbox",
            "data-slot": "checkbox",
            "data-state": data_state,
            "data-checked": checked || indeterminate,
            "aria-checked": if indeterminate { "mixed" } else if checked { "true" } else { "false" },
            class: class,
            ..attributes,

            if checked {
                CheckIcon {
                    class: "size-3.5 pointer-events-none",
                }
            } else if indeterminate {
                MinusIcon {
                    class: "size-3.5 pointer-events-none",
                }
            }

            {children}
        }
    }
}
