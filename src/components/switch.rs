use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct SwitchProps {
    on_change: EventHandler<bool>,
    value: bool,
    close: Option<Element>,
    open: Option<Element>,
}

#[component]
pub fn Switch(props: SwitchProps) -> Element {
    let mut value = use_signal(|| props.value);
    rsx! {
        div {
            class: "com-switch",
            if let Some(close) = props.close {
                { close }
            }
            label {
                class: "com-switch-label",
                input {
                    hidden: true,
                    r#type: "checkbox",
                    checked: "{value()}",
                    oninput: move |event| {
                        let is_enabled = event.value() == "true";
                        value.set(is_enabled);
                        props.on_change.call(is_enabled);
                    }
                }
                div {
                    class: "com-switch-slider",
                }
            }
            if let Some(open) = props.open {
                { open }
            }
        }
    }
}