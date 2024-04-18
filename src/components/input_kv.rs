use dioxus::prelude::*;

use crate::{
    components::icons::{FALSE, TRUE},
    utils::format::format_public_key,
};

#[derive(PartialEq, Clone, Props)]
pub struct InputKvProps {
    on_change: EventHandler<(String, String)>,
    placeholder: Option<(String, String)>,
    #[props(default = false)]
    edit: bool,
    value: (String, String),
}

#[component]
pub fn InputKv(props: InputKvProps) -> Element {
    let mut value = use_signal(|| props.value);
    let mut bak = use_signal(|| (String::new(), String::new()));
    let mut edit = use_signal(|| props.edit);
    let (p1, p2) = props.placeholder.unwrap_or_default();
    rsx! {
        div {
            class: "com-input_card",
            div {
                class: "com-input_card-content {edit() == false}",
                onclick: move |_| {
                    bak.set(value());
                    edit.set(true);
                },
                if value().1.is_empty() {
                    "{format_public_key(&value().0, None)}"
                } else {
                    "{value().1}"
                }
            }
            div {
                class: "com-input_card-input {edit() == true}",
                input {
                    r#type: "text",
                    style: "max-width: 110px;",
                    value: "{value().0}",
                    placeholder: p1,
                    oninput: move |event| {
                        let mut tmp = value.write();
                        tmp.0 = event.value();
                    }
                }
                input {
                    r#type: "text",
                    style: "max-width: 100px;",
                    value: "{value().1}",
                    placeholder: p2,
                    oninput: move |event| {
                        let mut tmp = value.write();
                        tmp.1 = event.value();
                    }
                }
                button {
                    class: "btn btn-true",
                    onclick: move |_| {
                        bak.set(value());
                        edit.set(false);
                        props.on_change.call(bak());
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
