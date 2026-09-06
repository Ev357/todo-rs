use dioxus::prelude::*;

#[component]
pub fn CheckIcon(
    #[props(default = 24)] size: u32,
    #[props(default = "currentColor".to_string())] color: String,
    #[props(default = String::new())] class: String,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: color,
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            class: class,

            path {
                d: "M20 6 9 17l-5-5",
            }
        }
    }
}
