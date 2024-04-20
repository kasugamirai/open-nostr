use dioxus::prelude::*;

use crate::{
    components::icons::{FALSE, TRUE},
    state::subscription::Account,
    utils::format::format_public_key,
};

#[derive(PartialEq, Clone, Props)]
pub struct InputAccountProps {
    on_change: EventHandler<Account>,
    placeholder: Option<(String, String)>,
    #[props(default = false)]
    edit: bool,
    value: Account,
}

#[component]
pub fn InputAccount(props: InputAccountProps) -> Element {
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
                if value().alt_name.is_empty() {
                    "{format_public_key(&value().npub, None)}"
                } else {
                    "{value().alt_name}"
                }
            }
            div {
                class: "com-input_card-input {edit() == true}",
                input {
                    r#type: "text",
                    value: "{value().npub}",
                    placeholder: p1,
                    oninput: move |event| {
                        value.write().npub = event.value();
                    }
                }
                input {
                    r#type: "text",
                    value: "{value().alt_name}",
                    placeholder: p2,
                    oninput: move |event| {
                        value.write().alt_name = event.value();
                    }
                }
                button {
                    class: "btn btn-true",
                    onclick: move |_| {
                        // TODO: Get 'alt name' if 'value.alt_name' is empty
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
                        value.set(v);
                        edit.set(false);
                        props.on_change.call(value());
                    },
                    dangerous_inner_html: "{FALSE}"
                }
            }
        }
    }
}
