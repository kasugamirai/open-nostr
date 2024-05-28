use dioxus::prelude::*;

use crate::components::{
    icons::{FALSE, TRUE},
    ClickOutside,
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

    // cancel editing status when the parent does not allow editing
    use_effect(use_reactive((&edit,), move |(mut edit,)| {
        if !allow_edit() {
            edit.set(false);
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
                props.on_change.call(value.read().clone());
            },
            div {
                class: "relative",
                div {
                    class:"sub-input",
                    onclick: move |_| {
                        edit.set(true);
                    },
                    " {bak} "
                }
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
