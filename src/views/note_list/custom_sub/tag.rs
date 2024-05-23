use dioxus::prelude::*;

use crate::{
    components::icons::{FALSE, TRUE},
    store::subscription::Tag,
};

#[derive(PartialEq, Clone, Props)]
pub struct TagInputProps {
    on_change: EventHandler<Tag>,
    tag: Tag,
    #[props(default = false)]
    edit: bool,
    #[props(default = 0)]
    index: usize,
}

#[component]
pub fn TagInput(props: TagInputProps) -> Element {
    let allow_edit = use_context::<Signal<bool>>();
    let mut value = use_signal(|| props.tag.clone());
    let mut bak = use_signal(|| props.tag.clone());
    let mut edit = use_signal(|| *allow_edit.read() && props.edit);

    use_effect(move || {
        if !allow_edit() {
            edit.set(false);
        }
    });

    use_effect(use_reactive((&props.tag,), move |(tag,)| {
        value.set(tag.clone());
        bak.set(tag);
    }));

    let cn = format!("custom-sub-tag-wapper-{}", props.index);

    rsx! {
        div {
            class: "{cn} relative",
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
                "#{value().tag} | {value().value}"
            }
            div {
                class: "show-{edit} add-pop-up-style",
                label {
                    class:"display-align-gap",
                    input {
                        r#type: "text",
                        class:"add-input add-input-76",
                        placeholder: "tag",
                        value: "{value().tag}",
                        oninput: move |event| {
                            value.write().tag = event.value();
                        }
                    }
                    input {
                        r#type: "text",
                        class:"add-input add-input-76",
                        placeholder: "value",
                        value: "{value().value}",
                        oninput: move |event| {
                            value.write().value = event.value();
                        }
                    }
                    button {
                        class: "btn-circle btn-circle-true close-{cn}",
                        onclick: move |_| {
                            // TODO: Get 'alt name' if 'value.alt_name' is empty
                            bak.set(value());
                            edit.set(false);
                            props.on_change.call(value.read().clone());
                        },
                        dangerous_inner_html: "{TRUE}"
                    }
                    button {
                        class: "btn-circle btn-circle-false close-{cn}",
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
