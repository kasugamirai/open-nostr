use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct CheckboxProps {
    on_change: EventHandler<bool>,
    value: bool,
    children: Element,
}

#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
    let mut value = use_signal(|| props.value);
    rsx! {
        label {
            class: "com-checkbox",
            input {
                r#type: "checkbox",
                checked: "{value()}",
                oninput: move |event| {
                    let is_enabled = event.value() == "true";
                    value.set(is_enabled);
                    props.on_change.call(is_enabled);
                }
            }
        }
    }
}
