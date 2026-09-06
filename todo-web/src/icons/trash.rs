use dioxus::prelude::*;

#[component]
pub fn TrashIcon(
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
                d: "M10 11v6",
            }
            path {
                d: "M14 11v6",
            }
            path {
                d: "M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6",
            }
            path {
                d: "M3 6h18",
            }
            path {
                d: "M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2",
            }
        }
    }
}
