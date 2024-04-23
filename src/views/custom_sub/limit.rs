use dioxus::prelude::*;

use crate::components::icons::{FALSE, TRUE};

#[derive(PartialEq, Clone, Props)]
pub struct LimitInputProps {
    on_change: EventHandler<usize>,
    limit: usize,
    #[props(default = false)]
    edit: bool,
}

#[component]
pub fn LimitInput(props: LimitInputProps) -> Element {
    let mut value = use_signal(|| props.limit.clone());
    let mut bak = use_signal(|| props.limit);
    let mut edit = use_signal(|| props.edit);
    rsx! {
        div {
            style: "position: relative;",
            div {
                style: "background-color: var(--bgc-0); height: 42px; padding: 10px 20px; border-radius: var(--radius-circle); cursor: pointer; display: flex; align-items: center; justify-content: center; white-space: nowrap;",
                onclick: move |_| {
                    edit.set(!edit());
                },
                "{value}",
            }
            div {
                class: "show-{edit}",
                style: "position: absolute; bottom: 42px; background-color: var(--bgc-0); border-radius: var(--radius-1); display: flex; flex-direction: column; gap: 10px; padding: 10px; 20px; border: 1px solid var(--boc-1); z-index: 100;",
                label {
                    style: "display: flex; align-items: center; gap: 10px;",
                    input {
                        r#type: "text",
                        style: "border: none; border-bottom: 2px solid var(--boc-1); font-size: 16px;",
                        placeholder: "limit",
                        value: "{value}",
                        oninput: move |event| {
                            let v = event.value().parse::<usize>().unwrap_or(0);
                            value.set(v);
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
