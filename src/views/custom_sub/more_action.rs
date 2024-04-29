use dioxus::prelude::*;
use serde_json::Value;

use crate::{
    components::icons::MORE,
    state::subscription::{CustomAccounts, CustomEvents, CustomFilter, CustomHashTag, FilterTemp},
};

#[derive(PartialEq, Clone, Props)]
pub struct AddFilterProps {
    on_click: EventHandler<FilterTemp>,
    #[props(default = 0)]
    index: usize,
}

#[component]
pub fn MoreAction(props: AddFilterProps) -> Element {
    let mut edit = use_signal(|| false);

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

    let cn = format!("custom-sub-more-action-wapper-{}", props.index);

    click_outside(cn.clone());

    rsx! {
        div {
            class: "{cn}",
            style: "position: relative; width: 32px; height: 32px;",
            button {
                class: "btn-circle",
                onclick: move |_| {
                    edit.set(!edit());
                },
                dangerous_inner_html: "{MORE}"
            }
            div {
                class: "show-{edit}",
                style: "position: absolute; right: 0; background-color: var(--bgc-0); border-radius: var(--radius-1); display: flex; flex-direction: column; gap: 10px; padding: 10px; 20px; border: 1px solid var(--boc-1); z-index: 100;",
                div {
                    style: "display: flex; flex-direction: column; gap: 10px; padding: 10px; 20px;",
                    button {
                        class: "btn-add-item",
                        onclick: move |_| {
                            props.on_click.call(FilterTemp::HashTag(CustomHashTag::empty()));
                            edit.set(false);
                        },
                        "Only Tags"
                    }
                    button {
                        class: "btn-add-item",
                        onclick: move |_| {
                            props.on_click.call(FilterTemp::Accounts(CustomAccounts::empty()));
                            edit.set(false);
                        },
                        "Follow People"
                    }
                    button {
                        class: "btn-add-item",
                        onclick: move |_| {
                            props.on_click.call(FilterTemp::Events(CustomEvents::empty()));
                            edit.set(false);
                        },
                        "Follow Notes"
                    }
                    button {
                        class: "btn-add-item",
                        onclick: move |_| {
                            props.on_click.call(FilterTemp::Customize(CustomFilter::empty()));
                            edit.set(false);
                        },
                        "Customize"
                    }
                }
            }
        }
    }
}