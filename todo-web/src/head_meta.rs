use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const GEIST_FONT: Asset = asset!("/assets/fonts/Geist.woff2");

#[component]
pub fn HeadMeta() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: TAILWIND_CSS }
        document::Style {
             r#"
             @font-face {{
                 font-family: 'Geist Variable';
                 src: url('{GEIST_FONT}') format('woff2');
                 font-weight: 100 900;
                 font-style: normal;
                 font-display: swap;
             }}
             "#
        }
    }
}
