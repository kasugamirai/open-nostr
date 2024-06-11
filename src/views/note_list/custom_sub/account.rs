use dioxus::prelude::*;

use crate::components::icons::{FALSE, TRUE};
use crate::store::subscription::Account;
use crate::utils::format::format_public_key;

#[derive(PartialEq, Clone, Props)]
pub struct AccountInputProps {
    on_change: EventHandler<Account>,
    account: Account,
    #[props(default = false)]
    edit: bool,
    #[props(default = 0)]
    index: usize,
}

#[component]
pub fn AccountInput(props: AccountInputProps) -> Element {
    let allow_edit = use_context::<Signal<bool>>();
    let mut value = use_signal(|| props.account.clone());
    let mut bak = use_signal(|| props.account);
    let mut edit = use_signal(|| *allow_edit.read() && props.edit);

    use_effect(move || {
        if !allow_edit() {
            edit.set(false);
        }
    });

    rsx! {
        div {
            class: "relative",
            div {
                class:"sub-shadow",
                onclick: move |_| {
                    let v = edit();
                    if v {
                        edit.set(false);
                    } else if allow_edit() {
                        edit.set(true);
                    }
                    props.on_change.call(value.read().clone());
                },
                if value().alt_name.is_empty() {
                    "{format_public_key(&value().npub, None)}"
                } else {
                    "{value().alt_name}"
                }
            }
            div {
                class: "show-{edit} add-pop-up-style",
                label {
                    class:"flex-box-center",
                    input {
                        r#type: "text",
                        class:"add-input",
                        placeholder: "pubkey/npub",
                        value: "{value().npub}",
                        oninput: move |event| {
                            value.write().npub = event.value();
                        }
                    }
                    input {
                        r#type: "text",
                        class:"add-input add-input-width",
                        placeholder: "alt name",
                        value: "{value().alt_name}",
                        oninput: move |event| {
                            value.write().alt_name = event.value();
                        }
                    }
                    button {
                        class: "btn-circle btn-circle-true",
                        onclick: move |_| {
                            // TODO: Get 'alt name' if 'value.alt_name' is empty
                            bak.set(value());
                            edit.set(false);
                            props.on_change.call(value.read().clone());
                        },
                        dangerous_inner_html: "{TRUE}"
                    }
                    button {
                        class: "btn-circle btn-circle-false",
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
}
