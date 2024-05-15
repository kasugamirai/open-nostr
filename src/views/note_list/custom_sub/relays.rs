use dioxus::prelude::*;

use crate::{
    components::{
        icons::{ADD, FALSE, LEFT, RIGHT, TRUE},
        ClickOutside,
    },
    store::subscription::RelaySet,
    utils::js::{export_to_clipboard, import_from_clipboard},
};

#[derive(PartialEq, Clone, Props)]
pub struct RelaysInputProps {
    on_change: EventHandler<RelaySet>,
    relay_set: RelaySet,
    #[props(default = false)]
    edit: bool,
    #[props(default = 0)]
    index: usize,
}

#[component]
pub fn RelaysInput(props: RelaysInputProps) -> Element {
    let mut value = use_signal(|| props.relay_set.clone());
    let mut bak = use_signal(|| props.relay_set.clone());
    let mut new_relay = use_signal(String::new);
    let mut edit = use_signal(|| props.edit);

    // update value and cancel editing when parent data has changed
    use_effect(use_reactive(
        (&value, &props.relay_set, &edit),
        move |(mut value, relay_set, mut edit)| {
            value.set(relay_set.clone());
            bak.set(relay_set.clone());
            edit.set(false);
        },
    ));

    let handle_export = move |text: String| {
        export_to_clipboard(text);
    };

    let handle_import = move |_| {
        spawn(async move {
            let text = import_from_clipboard().await;
            if !text.is_empty() {
                let rs: Vec<String> = text.split(',').map(|s| s.trim().to_string()).collect();
                let mut v = value.write();
                v.relays = rs;
            }
        });
    };

    rsx! {
        ClickOutside {
            on_click: move |_| {
                edit.set(false);
            },
            div {
                style: "position: relative;",
                div {
                    class:"subInput",
                    onclick: move |_| {
                        edit.set(!edit());
                        props.on_change.call(value.read().clone());
                    },
                    " {props.relay_set.name} "
                }
                div {
                    class: "show-{edit} relays-popUp",
                    div {
                        style: "display: flex; gap: 10px; align-items: center;",
                        "Name:"
                        input {
                            style: " border: none; border-bottom: 1px solid var(--boc-1); font-size: 16px;",
                            r#type: "text",
                            value: "{value().name}",
                            oninput: move |event| {
                                let mut v = value.write();
                                v.name = event.value();
                            }
                        }
                        button {
                            class: "btn-circle btn-circle-true",
                            onclick: move |_| {
                                bak.set(value());
                                props.on_change.call(value.read().clone());
                                edit.set(false);
                            },
                            dangerous_inner_html: "{TRUE}"
                        }
                        button {
                            class: "btn-circle btn-circle-success",
                            onclick: move |_| {
                                handle_export(value.read().relays.join(",\n"));
                            },
                            div {
                                dangerous_inner_html: "{RIGHT}"
                            }
                        }
                        button {
                            class: "btn-circle btn-circle-success",
                            onclick: handle_import,
                            div {
                            dangerous_inner_html: "{LEFT}"
                            }
                        }
                    }
                    for (i, relay) in value.read().relays.iter().enumerate() {
                        div {
                            style: "display: flex; gap: 10px;",
                            input {
                                style: "border: none; border-bottom: 1px solid var(--boc-1); font-size: 16px; width: 322px;",
                                r#type: "text",
                                value: "{relay}",
                                oninput: move |event| {
                                    let mut v = value.write();
                                    v.relays[i] = event.value();
                                }
                            }
                            button {
                                class: "btn-circle btn-circle-false",
                                onclick: move |_| {
                                    let mut v = value.write();
                                    v.relays.remove(i);
                                },
                                div {
                                    dangerous_inner_html: "{FALSE}"
                                }
                            }
                        }
                    }
                    div {
                        style: "display: flex; gap: 10px;",
                        input {
                            style: "border: none; border-bottom: 1px solid var(--boc-1); font-size: 16px; width: 322px;",
                            r#type: "text",
                            value: "{new_relay}",
                            oninput: move |event| {
                                new_relay.set(event.value());
                            }
                        }
                        button {
                            class: "btn-icon add",
                            onclick: move |_| {
                                let mut v = value.write();
                                v.relays.push(new_relay());
                                new_relay.set(String::new());
                            },
                            div {
                                dangerous_inner_html: "{ADD}"
                            }
                        }
                    }
                }
            }
        }
    }
}
