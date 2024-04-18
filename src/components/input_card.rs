use dioxus::prelude::*;

use crate::components::icons::{FALSE, TRUE};

#[derive(PartialEq, Clone, Props)]
pub struct InputCardProps {
    on_change: EventHandler<String>,
    placeholder: Option<String>,
    #[props(default = false)]
    edit: bool,
    value: String,
}

#[component]
pub fn InputCard(props: InputCardProps) -> Element {
    let mut value = use_signal(|| props.value);
    let mut bak = use_signal(|| String::new());
    let mut edit = use_signal(|| props.edit);
    rsx! {
        div {
            class: "com-input_card",
            div {
                class: "com-input_card-content {edit() == false}",
                onclick: move |_| {
                    bak.set(value());
                    edit.set(true);
                },
                "{value}",
            }
            div {
                class: "com-input_card-input {edit() == true}",
                input {
                    r#type: "text",
                    value: "{value}",
                    placeholder: props.placeholder.unwrap_or_default(),
                    oninput: move |event| {
                        let v = event.value();
                        value.set(v.clone());
                        props.on_change.call(v);
                    }
                }
                button {
                    class: "btn btn-true",
                    onclick: move |_| {
                        bak.set(value());
                        edit.set(false);
                    },
                    dangerous_inner_html: "{TRUE}"
                }
                button {
                    class: "btn btn-false",
                    onclick: move |_| {
                        let v = bak();
                        value.set(v.clone());
                        props.on_change.call(v);
                        edit.set(false);
                    },
                    dangerous_inner_html: "{FALSE}"
                }
            }
        }
    }
}
