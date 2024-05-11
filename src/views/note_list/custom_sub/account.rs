use dioxus::prelude::*;

use crate::{
    components::icons::{FALSE, TRUE},
    store::subscription::Account,
    utils::format::format_public_key,
};

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
            style: "position: relative;",
            div {
                style: "background-color: var(--bgc-0); height: 42px; padding: 10px 20px; border-radius: var(--radius-circle); cursor: pointer; display: flex; align-items: center; justify-content: center; white-space: nowrap;",
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
                class: "show-{edit}",
                style: "position: absolute; bottom: 42px; background-color: var(--bgc-0); border-radius: var(--radius-1); display: flex; flex-direction: column; gap: 10px; padding: 10px; 20px; border: 1px solid var(--boc-1); z-index: 100;",
                label {
                    style: "display: flex; align-items: center; gap: 10px;",
                    input {
                        r#type: "text",
                        style: "border: none; border-bottom: 2px solid var(--boc-1); font-size: 16px;",
                        placeholder: "pubkey/npub",
                        value: "{value().npub}",
                        oninput: move |event| {
                            value.write().npub = event.value();
                        }
                    }
                    input {
                        r#type: "text",
                        style: "border: none; border-bottom: 2px solid var(--boc-1); font-size: 16px; width: 160px;",
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
