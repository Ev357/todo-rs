use dioxus::prelude::*;

const BASE_STYLES: &str = "\
    rounded-md border border-transparent bg-clip-padding text-sm font-medium \
    focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50 \
    active:not-aria-[haspopup]:translate-y-px \
    aria-invalid:border-destructive aria-invalid:ring-3 aria-invalid:ring-destructive/20 \
    dark:aria-invalid:border-destructive/50 dark:aria-invalid:ring-destructive/40 \
    [&_svg:not([class*='size-'])]:size-4 group/button inline-flex shrink-0 items-center justify-center \
    whitespace-nowrap transition-all outline-none select-none \
    disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0";

#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[allow(dead_code)]
pub enum ButtonVariant {
    #[default]
    Default,
    Outline,
    Secondary,
    Ghost,
    Destructive,
    Link,
}

impl ButtonVariant {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "bg-primary text-primary-foreground hover:bg-primary/80",
            Self::Outline => "\
                border-border bg-background shadow-xs hover:bg-muted hover:text-foreground \
                aria-expanded:bg-muted aria-expanded:text-foreground \
                dark:border-input dark:bg-input/30 dark:hover:bg-input/50",
            Self::Secondary => "\
                bg-secondary text-secondary-foreground \
                hover:bg-[color-mix(in_oklch,var(--secondary),var(--foreground)_5%)] \
                aria-expanded:bg-secondary aria-expanded:text-secondary-foreground",
            Self::Ghost => "\
                hover:bg-muted hover:text-foreground aria-expanded:bg-muted \
                aria-expanded:text-foreground dark:hover:bg-muted/50",
            Self::Destructive => "\
                bg-destructive/10 text-destructive hover:bg-destructive/20 \
                focus-visible:border-destructive/40 focus-visible:ring-destructive/20 \
                dark:bg-destructive/20 dark:hover:bg-destructive/30 dark:focus-visible:ring-destructive/40",
            Self::Link => "text-primary underline-offset-4 hover:underline",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[allow(dead_code)]
pub enum ButtonSize {
    #[default]
    Default,
    Xs,
    Sm,
    Lg,
    Icon,
    IconXs,
    IconSm,
    IconLg,
}

impl ButtonSize {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "\
                h-9 gap-1.5 px-2.5 in-data-[slot=button-group]:rounded-md \
                has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2",
            Self::Xs => "\
                h-6 gap-1 rounded-[min(var(--radius-md),8px)] px-2 text-xs \
                in-data-[slot=button-group]:rounded-md has-data-[icon=inline-end]:pr-1.5 \
                has-data-[icon=inline-start]:pl-1.5 [&_svg:not([class*='size-'])]:size-3",
            Self::Sm => "\
                h-8 gap-1 rounded-[min(var(--radius-md),10px)] px-2.5 \
                in-data-[slot=button-group]:rounded-md has-data-[icon=inline-end]:pr-1.5 \
                has-data-[icon=inline-start]:pl-1.5",
            Self::Lg => "\
                h-10 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2",
            Self::Icon => "size-9",
            Self::IconXs => "\
                size-6 rounded-[min(var(--radius-md),8px)] in-data-[slot=button-group]:rounded-md \
                [&_svg:not([class*='size-'])]:size-3",
            Self::IconSm => "\
                size-8 rounded-[min(var(--radius-md),10px)] in-data-[slot=button-group]:rounded-md",
            Self::IconLg => "size-10",
        }
    }
}

#[component]
pub fn Button(
    class: Option<String>,
    #[props(default)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(default = false)] disabled: bool,
    #[props(default = "button".to_string())] r#type: String,
    onclick: Option<EventHandler<MouseEvent>>,
    onfocus: Option<EventHandler<FocusEvent>>,
    onblur: Option<EventHandler<FocusEvent>>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    #[props(extends = GlobalAttributes)]
    #[props(extends = button)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let variant_classes = variant.as_str();
    let size_classes = size.as_str();

    let computed_class = match class {
        Some(custom) => format!("{BASE_STYLES} {variant_classes} {size_classes} {custom}"),
        None => format!("{BASE_STYLES} {variant_classes} {size_classes}"),
    };

    let button_type = r#type;

    rsx! {
        button {
            r#type: button_type,
            "data-slot": "button",
            disabled: disabled,
            class: computed_class,
            onclick: move |e| _ = onclick.map(|cb| cb(e)),
            onfocus: move |e| _ = onfocus.map(|cb| cb(e)),
            onblur: move |e| _ = onblur.map(|cb| cb(e)),
            onkeydown: move |e| _ = onkeydown.map(|cb| cb(e)),
            ..attributes,

            {children}
        }
    }
}
