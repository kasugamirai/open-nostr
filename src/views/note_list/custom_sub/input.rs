use dioxus::prelude::*;
use regex::Regex;

use crate::{
    components::{
        icons::{FALSE, TRUE},
        ClickOutside,
    },
    utils::{contants::NUM_AND_LETTER_REG, js::alert},
};

#[derive(PartialEq, Clone, Props)]
pub struct InputProps {
    on_change: EventHandler<String>,
    value: String,
    #[props(default = false)]
    edit: bool,
}

#[component]
pub fn Input(props: InputProps) -> Element {
    // is ro not allow editing
    let allow_edit = use_context::<Signal<bool>>();
    let mut edit = use_signal(|| *allow_edit.read() && props.edit);

    // current value
    let mut value = use_signal(|| props.value.clone());

    // backup value to restore value when cancel editing
    let mut bak = use_signal(|| props.value.clone());

    let nostr_name_reg = Regex::new(NUM_AND_LETTER_REG).unwrap();
    // cancel editing status when the parent does not allow editing
    use_effect(use_reactive((&edit,), move |(edit,)| {
        if !edit() {
            value.set(bak());
        }
    }));

    // update value and cancel editing when parent data has changed
    use_effect(use_reactive((&props.value,), move |(v,)| {
        value.set(v.clone());
        bak.set(v.clone());
        edit.set(false);
    }));

    rsx! {
        ClickOutside {
            on_click: move |_| {
                edit.set(false);
            },
            div {
                class: "relative",
                div {
                    class:"sub-input cursor-pointer",
                    onclick: move |_| {
                        edit.set(true);
                    },
                    " {bak} "
                }
                if edit() {
                    div {
                        class: "show-{edit} add-pop-up-style",
                        label {
                            class:"flex-box-center",
                            input {
                                r#type: "text",
                                class:"add-input",
                                placeholder: "Please input",
                                value: "{value}",
                                oninput: move |event| {
                                    value.set(event.value());
                                }
                            }
                            button {
                                class: "btn-circle btn-circle-true",
                                onclick: move |_| {
                                    if value().is_empty() {
                                        spawn(async move {
                                            alert("Please input value".to_string()).await;
                                        });
                                        return;
                                    }
                                    if !nostr_name_reg.is_match(&value()) {
                                        spawn(async move {
                                            alert("Name can only contain letters and numbers".to_string()).await;
                                        });
                                        return;
                                    }
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
    }
}
