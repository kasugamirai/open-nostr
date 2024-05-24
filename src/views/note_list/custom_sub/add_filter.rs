use dioxus::prelude::*;
use serde_json::Value;

use crate::{
    components::icons::ADD,
    store::subscription::{CustomAccounts, CustomEvents, CustomFilter, CustomHashTag, FilterTemp},
};

#[derive(PartialEq, Clone, Props)]
pub struct AddFilterProps {
    on_click: EventHandler<FilterTemp>,
    #[props(default = 0)]
    index: usize,
}

#[component]
pub fn AddFilter(props: AddFilterProps) -> Element {
    let allow_edit = use_context::<Signal<bool>>();
    let mut edit = use_signal(|| false);

    use_effect(move || {
        if !allow_edit() {
            edit.set(false);
        }
    });

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

    let cn = format!("custom-sub-add-filter-wapper-{}", props.index);

    click_outside(cn.clone());

    rsx! {
        div {
            class: "{cn} relative",
            button {
                class: format!("btn-circle btn-circle-true {}", if *allow_edit.read() { "display-flex-box" } else { "display-none-important" }),
                onclick: move |_| {
                    edit.set(!edit());
                },
                dangerous_inner_html: "{ADD}"
            }
            div {
                class: "show-{edit} add-filter-more-box",
                div {
                    class: "add-filter-more-mod-box",
                    button {
                        class: "btn-add-item",
                        onclick: move |_| {
                            props.on_click.call(FilterTemp::HashTag(CustomHashTag::empty()));
                            edit.set(false);
                        },
                        "Follow Hash Tags"
                    }
                    button {
                        class: "btn-add-item",
                        onclick: move |_| {
                            props.on_click.call(FilterTemp::Accounts(CustomAccounts::empty()));
                            edit.set(false);
                        },
                        "Follow Accounts"
                    }
                    button {
                        class: "btn-add-item",
                        onclick: move |_| {
                            props.on_click.call(FilterTemp::Events(CustomEvents::empty()));
                            edit.set(false);
                        },
                        "Follow Notes"
                    }
                    // button {
                    //     class: "btn-add-item",
                    //     onclick: move |_| {
                    //         props.on_click.call(FilterTemp::Customize(CustomFilter::empty()));
                    //         edit.set(false);
                    //     },
                    //     "Customize"
                    // }
                }
            }
        }
    }
}
