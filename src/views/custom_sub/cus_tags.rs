use dioxus::prelude::*;

use crate::{
    components::icons::{FALSE, TRUE},
    state::subscription::Tag,
};

#[derive(PartialEq, Clone, Props)]
pub struct CusTagProps {
    on_change: EventHandler<Tag>,
    placeholder: Option<(String, String)>,
    #[props(default = false)]
    edit: bool,
    value: Tag,
}

#[component]
pub fn InputCusTag(props: CusTagProps) -> Element {
    let mut value = use_signal(|| props.value.clone());
    let mut bak = use_signal(|| props.value);
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
                "#{value().tag} | {value().value}"
            }
            div {
                class: "com-input_card-input {edit() == true}",
                input {
                    style: "max-width: 50px",
                    r#type: "text",
                    value: "{value().tag}",
                    placeholder: p1,
                    oninput: move |event| {
                        value.write().tag = event.value();
                    }
                }
                input {
                    r#type: "text",
                    style: "max-width: 100px",
                    value: "{value().value}",
                    placeholder: p2,
                    oninput: move |event| {
                        value.write().value = event.value();
                    }
                }
                button {
                    class: "btn btn-true",
                    onclick: move |_| {
                        bak.set(value());
                        edit.set(false);
                        props.on_change.call(value());
                    },
                    dangerous_inner_html: "{TRUE}"
                }
                button {
                    class: "btn btn-false",
                    onclick: move |_| {
                        let v = bak();
                        value.set(v.clone());
                        edit.set(false);
                        props.on_change.call(value());
                    },
                    dangerous_inner_html: "{FALSE}"
                }
            }
        }
    }
}
