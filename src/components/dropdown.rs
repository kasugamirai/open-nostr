use dioxus::prelude::*;
//use serde_json::Value;

#[derive(PartialEq, Clone, Props)]
pub struct DropdownProps {
    #[props(default = "left".to_string())]
    pos: String,
    #[props(default = "hover".to_string())]
    show: String,
    trigger: Element,
    children: Element,
}

#[component]
pub fn Dropdown(props: DropdownProps) -> Element {
    let style = if props.pos == "left" {
        "left: 0;"
    } else {
        "right: 0;"
    };

    let mut show = use_signal(|| false);

    let hash = "com-dropdown".to_string();
    let id = hash;

    rsx! {
        div {
            class: "com-dropdown {props.show}",
            div {
                class: "com-dropdown-trigger",
                onclick: move |_| {
                    show.set(!show());
                },
                { props.trigger }
            }
            div {
                id: "{id}",
                class: "com-dropdown-content {props.show} {show}",
                style: "{style}",
                { props.children }
            }
        }
    }
}
