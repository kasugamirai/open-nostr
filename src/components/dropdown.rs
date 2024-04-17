use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct DropdowntProps {
    #[props(default = "left".to_string())]
    pos: String,
    trigger: Element,
    children: Element,
}

#[component]
pub fn Dropdown(props: DropdowntProps) -> Element {
    let style = if props.pos == "left" {
        "left: 0;"
    } else {
        "right: 0;"
    };
    rsx! {
        div {
            class: "com-dropdown",
            { props.trigger }
            div {
                class: "com-dropdown-content",
                style: "{style}",
                { props.children }
            }
        }
    }
}
