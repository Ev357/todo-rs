use dioxus::prelude::*;

#[component]
pub fn LoaderIcon(
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
                d: "M12 2v4",
            }
            path {
                d: "m16.2 7.8 2.9-2.9",
            }
            path {
                d: "M18 12h4",
            }
            path {
                d: "m16.2 16.2 2.9 2.9",
            }
            path {
                d: "M12 18v4",
            }
            path {
                d: "m4.9 19.1 2.9-2.9",
            }
            path {
                d: "M2 12h4",
            }
            path {
                d: "m4.9 4.9 2.9 2.9",
            }
        }
    }
}
