use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct ButtonProps {
    on_click: EventHandler<MouseEvent>,
    children: Element,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    rsx! {
        button {
            class: "com-button",
            onclick: move |evt| props.on_click.call(evt),
            { props.children }
        }
    }
}
