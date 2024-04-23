use dioxus::prelude::*;
use serde_json::Value;

use crate::{
    components::icons::{ADD, FALSE, LEFT, RIGHT, TRUE},
    state::subscription::RelaySet,
};

#[derive(PartialEq, Clone, Props)]
pub struct RelaysInputProps {
    on_change: EventHandler<RelaySet>,
    relay_set: RelaySet,
    #[props(default = false)]
    edit: bool,
}

#[component]
pub fn RelaysInput(props: RelaysInputProps) -> Element {
    let mut value = use_signal(|| props.relay_set.clone());
    let mut new_relay = use_signal(String::new);
    let mut bak = use_signal(|| props.relay_set);
    let mut edit = use_signal(|| props.edit);

    let handle_export = move |text: String| {
        let eval = eval(
            r#"
                let c = navigator.clipboard;
                if (!c) {
                    console.error('Clipboard not supported');
                    return false;
                }
                let msg = await dioxus.recv();
                console.log(msg);
                await c.writeText(msg);
                alert("Copied to clipboard");
                return true;
            "#,
        );
        eval.send(text.into()).unwrap();
    };

    let handle_import = move |_| {
        spawn(async move {
            let mut eval = eval(
                r#"
                    let c = navigator.clipboard;
                    if (!c) {
                        console.error('Clipboard not supported');
                        return false;
                    }
                    let msg = await c.readText();
                    console.log(msg);
                    await dioxus.send(msg);
                    return true;
                "#,
            );
            eval.send("".into()).unwrap();
            if let Value::String(res) = eval.recv().await.unwrap() {
                let rs: Vec<String> = res.split(',').map(|s| s.trim().to_string()).collect();
                let mut v = value.write();
                v.relays = rs;
            }
        });
    };

    rsx! {
        div {
            style: "position: relative;",
            div {
                style: "background-color: var(--bgc-3); height: 42px; padding: 10px 20px; border-radius: var(--radius-circle); cursor: pointer; display: flex; align-items: center; justify-content: center; white-space: nowrap;",
                onclick: move |_| {
                    edit.set(!edit());
                    props.on_change.call(value.read().clone());
                },
                "{value().name}"
            }
            div {
                class: "show-{edit}",
                style: "position: absolute; background-color: var(--bgc-0); border-radius: var(--radius-1); display: flex; flex-direction: column; gap: 10px; padding: 10px; 20px; border: 1px solid var(--boc-1); z-index: 100;",
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
