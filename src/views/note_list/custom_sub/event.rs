use dioxus::prelude::*;

use crate::components::icons::{FALSE, TRUE};
use crate::store::subscription::Event;
use crate::utils::format::format_public_key;

#[derive(PartialEq, Clone, Props)]
pub struct EventInputProps {
    on_change: EventHandler<Event>,
    event: Event,
    #[props(default = false)]
    edit: bool,
    #[props(default = 0)]
    index: usize,
}

#[component]
pub fn EventInput(props: EventInputProps) -> Element {
    let mut value = use_signal(|| props.event.clone());
    let mut bak = use_signal(|| props.event);
    let mut edit = use_signal(|| props.edit);

    rsx! {
        div {
            class: "relative",
            div {
                class:"sub-shadow",
                onclick: move |_| {
                    edit.set(!edit());
                    props.on_change.call(value.read().clone());
                },
                if value().alt_name.is_empty() {
                    "{format_public_key(&value().nevent, None)}"
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
                        placeholder: "id/nevent",
                        value: "{value().nevent}",
                        oninput: move |event| {
                            value.write().nevent = event.value();
                        }
                    }
                    input {
                        r#type: "text",
                        class:"add-input",
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
