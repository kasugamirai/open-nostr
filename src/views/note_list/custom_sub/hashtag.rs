use dioxus::prelude::*;

use crate::components::{
    icons::{ADD, FALSE, TRUE},
    ClickOutside,
};

#[derive(PartialEq, Clone, Props)]
pub struct HashTagInputProps {
    on_change: EventHandler<String>,
    tag: String,
    #[props(default = false)]
    edit: bool,
}

#[component]
pub fn HashTagInput(props: HashTagInputProps) -> Element {
    // is ro not allow editing
    let allow_edit = use_context::<Signal<bool>>();
    let mut edit = use_signal(|| *allow_edit.read() && props.edit);

    // current value
    let mut value = use_signal(|| props.tag.clone());

    // backup value to restore value when cancel editing
    let mut bak = use_signal(|| props.tag.clone());

    // cancel editing status when the parent does not allow editing
    use_effect(use_reactive((&edit,), move |(mut edit,)| {
        if !allow_edit() {
            edit.set(false);
        }
    }));

    // update value and cancel editing when parent data has changed
    use_effect(use_reactive(
        (&value, &props.tag, &edit),
        move |(mut value, tag, mut edit)| {
            value.set(tag.clone());
            bak.set(tag.clone());
            edit.set(false);
        },
    ));

    rsx! {
        ClickOutside {
            on_click: move |_| {
                edit.set(false);
            },
            div {
                style: "position: relative;",
                div {
                    class: "sub-shadow",
                    onclick: move |_| {
                        if allow_edit() {
                            edit.set(true);
                        }
                    },
                    "{props.tag}"
                }
                div {
                    class: "show-{edit} addPopUpStyle",
                    label {
                        style: "display: flex; align-items: center; gap: 10px;",
                        input {
                            r#type: "text",
                            class:"addInput",
                            placeholder: "hashtag",
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

#[derive(PartialEq, Clone, Props)]
pub struct HashTagAddProps {
    on_change: EventHandler<String>,
}

#[component]
pub fn HashTagAdd(props: HashTagAddProps) -> Element {
    // is ro not allow editing
    let allow_edit = use_context::<Signal<bool>>();
    let mut edit = use_signal(|| *allow_edit.read());

    // current value
    let mut value = use_signal(String::new);

    // cancel editing status when the parent does not allow editing
    use_effect(use_reactive((&edit,), move |(mut edit,)| {
        if !allow_edit() {
            edit.set(false);
        }
    }));

    rsx! {
        ClickOutside {
            on_click: move |_| {
                edit.set(false);
                if !value.read().is_empty() {
                    props.on_change.call(value.read().clone());
                    value.set(String::new());
                }
            },
            div {
                style: "position: relative;",
                button {
                    class: "btn-add",
                    onclick: move |_| {
                        if allow_edit() {
                            edit.set(true);
                        }
                    },
                    dangerous_inner_html: "{ADD}",
                }
                div {
                    class: "show-{edit} addPopUpStyle",
                    label {
                        style: "display: flex; align-items: center; gap: 10px;",
                        input {
                            r#type: "text",
                            class: "addInput",
                            placeholder: "hashtag",
                            value: "{value}",
                            oninput: move |event| {
                                value.set(event.value());
                            }
                        }
                        button {
                            class: "btn-circle btn-circle-true",
                            onclick: move |_| {
                                edit.set(false);
                                if !value.read().is_empty() {
                                    props.on_change.call(value.read().clone());
                                    value.set(String::new());
                                }
                            },
                            dangerous_inner_html: "{TRUE}"
                        }
                        button {
                            class: "btn-circle btn-circle-false",
                            onclick: move |_| {
                                value.set(String::new());
                                edit.set(false);
                            },
                            dangerous_inner_html: "{FALSE}"
                        }
                    }
                }
            }
        }
    }
}
