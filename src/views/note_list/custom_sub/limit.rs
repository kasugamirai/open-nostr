use dioxus::prelude::*;
use serde_json::Value;

use crate::components::icons::{FALSE, TRUE};

#[derive(PartialEq, Clone, Props)]
pub struct LimitInputProps {
    on_change: EventHandler<usize>,
    limit: usize,
    #[props(default = false)]
    edit: bool,
    #[props(default = 0)]
    index: usize,
}

#[component]
pub fn LimitInput(props: LimitInputProps) -> Element {
    let mut value = use_signal(|| props.limit);
    let mut bak = use_signal(|| props.limit);
    let mut edit = use_signal(|| props.edit);

    let click_outside = move |cn: String| {
        spawn(async move {
            let mut eval = eval(
                r#"
                    // Listens for clicks on the 'document' element
                    let eid = await dioxus.recv()
                    let ceid = `close-${eid}`
                    const handle = (e) => {
                        let target = e.target
                        while (true && target) {
                            if (target.classList.contains(ceid)) {
                                // Clicked on the close button
                                dioxus.send(false)
                                return
                            } else if (target.classList.contains(eid)) {
                                // The element is a child of the dropdown
                                dioxus.send(true)
                                return
                            } else {
                                if (target === document.documentElement) {
                                    break
                                }
                            }
                            target = target.parentNode
                        }
                        
                        // The element is outside the dropdown
                        dioxus.send(false)

                        // Remove the event listener
                        // document.removeEventListener('click', handle)
                    }
                    document.addEventListener('click', handle)
                "#,
            );
            eval.send(cn.into()).unwrap();
            if let Value::Bool(res) = eval.recv().await.unwrap() {
                edit.set(res);
            }
        });
    };

    let cn = format!("custom-sub-limit-wapper-{}", props.index);

    click_outside(cn.clone());

    rsx! {
        div {
            class: "{cn} relative",
            div {
                class:"sub-shadow",
                onclick: move |_| {
                    edit.set(!edit());
                },
                "{value}",
            }
            div {
                class: "show-{edit} add-pop-up-style",
                label {
                    class:"display-align-gap",
                    input {
                        r#type: "text",
                        class:"add-input add-input-76",
                        placeholder: "limit",
                        value: "{value}",
                        oninput: move |event| {
                            let v = event.value().parse::<usize>().unwrap_or(0);
                            value.set(v);
                        }
                    }
                    button {
                        class: "btn-circle btn-circle-true close-{cn}",
                        onclick: move |_| {
                            // TODO: Get 'alt name' if 'value.alt_name' is empty
                            bak.set(value());
                            edit.set(false);
                            props.on_change.call(*value.read());
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
