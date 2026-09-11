use dioxus::prelude::*;

const BASE_STYLES: &str = "\
    h-9 w-full min-w-0 rounded-md border border-input bg-transparent px-2.5 py-1 \
    text-base shadow-xs transition-[color,box-shadow] md:text-sm \
    outline-none placeholder:text-muted-foreground \
    focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50 \
    aria-invalid:border-destructive aria-invalid:ring-3 aria-invalid:ring-destructive/20 \
    dark:bg-input/30 dark:aria-invalid:border-destructive/50 dark:aria-invalid:ring-destructive/40 \
    disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50";

#[component]
pub fn Input(
    class: Option<String>,
    #[props(extends = GlobalAttributes)]
    #[props(extends = input)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let class = match class {
        Some(custom) => format!("{BASE_STYLES} {custom}"),
        None => BASE_STYLES.to_string(),
    };

    rsx! {
        input {
            "data-slot": "input",
            class: class,
            ..attributes,
            {children}
        }
    }
}
