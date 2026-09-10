use dioxus::prelude::*;

const BASE_STYLES: &str = "rounded-md bg-muted animate-pulse";

#[component]
pub fn Skeleton(
    class: Option<String>,
    #[props(extends = GlobalAttributes)]
    #[props(extends = div)]
    attributes: Vec<Attribute>,
) -> Element {
    let computed_class = match class {
        Some(custom) => format!("{BASE_STYLES} {custom}"),
        None => BASE_STYLES.to_string(),
    };

    rsx! {
        div {
            "data-slot": "skeleton",
            class: computed_class,
            ..attributes,
        }
    }
}
