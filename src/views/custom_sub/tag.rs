use dioxus::prelude::*;

use crate::{
    components::icons::{FALSE, TRUE},
    state::subscription::Tag,
};

#[derive(PartialEq, Clone, Props)]
pub struct TagInputProps {
    on_change: EventHandler<Tag>,
    tag: Tag,
    #[props(default = false)]
    edit: bool,
}

#[component]
pub fn TagInput(props: TagInputProps) -> Element {
    let mut value = use_signal(|| props.tag.clone());
    let mut bak = use_signal(|| props.tag);
    let mut edit = use_signal(|| props.edit);
    rsx! {
        div {
            style: "position: relative;",
            div {
                style: "background-color: var(--bgc-0); height: 42px; padding: 10px 20px; border-radius: var(--radius-circle); cursor: pointer; display: flex; align-items: center; justify-content: center; white-space: nowrap;",
                onclick: move |_| {
                    edit.set(!edit());
                    props.on_change.call(value.read().clone());
                },
                "#{value().tag} | {value().value}"
            }
            div {
                class: "show-{edit}",
                style: "position: absolute; bottom: 42px; background-color: var(--bgc-0); border-radius: var(--radius-1); display: flex; flex-direction: column; gap: 10px; padding: 10px; 20px; border: 1px solid var(--boc-1); z-index: 100;",
                label {
                    style: "display: flex; align-items: center; gap: 10px;",
                    input {
                        r#type: "text",
                        style: "border: none; border-bottom: 2px solid var(--boc-1); font-size: 16px;",
                        placeholder: "tag",
                        value: "{value().tag}",
                        oninput: move |event| {
                            value.write().tag = event.value();
                        }
                    }
                    input {
                        r#type: "text",
                        style: "border: none; border-bottom: 2px solid var(--boc-1); font-size: 16px; width: 160px;",
                        placeholder: "value",
                        value: "{value().value}",
                        oninput: move |event| {
                            value.write().value = event.value();
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
