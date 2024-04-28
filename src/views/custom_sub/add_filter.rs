use dioxus::prelude::*;

use crate::{
    components::icons::ADD,
    state::subscription::{CustomAccounts, CustomEvents, CustomFilter, CustomHashTag, FilterTemp},
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
    rsx! {
        div {
            style: "position: relative;",
            button {
                class: "btn-circle btn-circle-true",
                style: format!("display: {}", if *allow_edit.read() { "block" } else { "none" }),
                onclick: move |_| {
                    edit.set(!edit());
                },
                dangerous_inner_html: "{ADD}"
            }
            div {
                class: "show-{edit}",
                style: "position: absolute; background-color: var(--bgc-0); border-radius: var(--radius-1); display: flex; flex-direction: column; gap: 10px; padding: 10px; 20px; border: 1px solid var(--boc-1); z-index: 100;",
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
