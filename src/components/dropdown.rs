use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct DropdowntProps {
    trigger: Element,
    children: Element,
}

#[component]
pub fn Dropdown(props: DropdowntProps) -> Element {
    rsx! {
        div {
            class: "com-dropdown",
            { props.trigger }
            div {
                class: "com-dropdown-content",
                { props.children }
            }
        }
    }
}
