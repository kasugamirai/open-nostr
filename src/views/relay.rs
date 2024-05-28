use dioxus::prelude::*;
use crate::components::icons::{ADD, FALSE, TRUE, BOTTOMRIGHT, UPPERRIGHT};
#[component]
pub fn Relay() -> Element {
    rsx! {
       div{
        class:"relay-contnet",
        div{
          class:"built-in-function text-center font-size-16",
          "built-in function"
          div{
            class:"built-li radius-26 text-center mt-12 font-size-14 line-height-28 text-overflow",
            "DM"
          }
          div{
            class:"built-li radius-26 text-center mt-12 font-size-14 line-height-28 text-overflow",
            "Channel"
          }
          div{
            class:"built-li radius-26 text-center mt-12 font-size-14 line-height-28 text-overflow",
            "Community"
          }
          div{
            class:"built-li radius-26 text-center mt-12 font-size-14 line-height-28 text-overflow",
            "Group"
          }
          div{
            class:"separate mt-20"
          }
          "Supscription"
          div{
            class:"separate mb-12"
          }
          div{
            class:"built-li radius-26 text-center mb-28 font-size-14 line-height-28 text-overflow built-li-checked",
            "#steakstr"
          }
          div{
            class:"built-li radius-26 text-center mb-28 font-size-14 line-height-28 text-overflow",
            "Movie #my collection"
          }
        }
        div{
          class:"set-content ml-78 px-18 py-18 radius-26",
          div {
            class: "modal-content relay-edit--content z-100 relative radius-26 flex",
            div{
                class:"relay-name-list p-10",
                div{
                    class: "relay-name-list--content overflow-y-auto",
                    // for (i, relay) in relay_sets.read().iter().enumerate() {
                        div{
                            // class: format!("mb-8 cursor-pointer radius-15 relay-name-item w-full px-8 text-overflow {}", if i == relay_curent_index() { "relay-name-item--active" } else { "" }),
                            onclick: move |_| {
                            },
                            // "{relay.name}",
                        }
                    // }
                }
                // Add new relay set
                div {
                    class: "relay-name-add w-full radius-15 px-8 cursor-pointer",
                    onclick: move |_| {

                    },
                    "new relay set"
                }
            }
            div{
                class:"relay-urls ml-41",
                div {
                    class:"relay-actions-bar flex mb-15",
                    input {
                        class:"relay-name-ipt",
                        r#type: "text",
                        placeholder: "Default",
                        oninput: move |_| {

                        }
                    }
                    button {
                        class: "btn-circle btn-circle-true ml-24",
                        onclick: move |_| {
                            // 1. todo save relays
                            // 2.
                            // bak.set(value());
                            // props.on_change.call(relay_sets.read().get(relay_curent_index()).unwrap().clone());
                            // edit.set(false);
                        },
                        dangerous_inner_html: "{TRUE}"
                    }
                    button {
                        class: "btn-circle btn-circle-false ml-9",
                        onclick: move |_| {
                            // handle_save();
                            // 1. todo save relays
                            // 2.
                            // bak.set(value());
                            // props.on_change.call(relay_sets.read().get(relay_curent_index()).unwrap().clone());
                            // edit.set(false);
                        },
                        dangerous_inner_html: "{FALSE}"
                    }
                    button {
                        class: "btn-circle btn-circle-success ml-9",
                        onclick: move |_| {
                            // handle_export(value.read().relays.join(",\n"));
                        },
                        div {
                            dangerous_inner_html: "{BOTTOMRIGHT}"
                        }
                    }
                    button {
                        class: "btn-circle btn-circle-success ml-9",
                        onclick: move |_| {
                            // handle_import();
                        },
                        div {
                        dangerous_inner_html: "{UPPERRIGHT}"
                        }
                    }
                }
                // for (i, relay_url) in current_relay_set.relays.iter().enumerate() {
                    div {
                        class:"relay-url-item mb-10 flex items-center",
                        input {
                            class: "relay-ipt mr-10",
                            r#type: "text",
                            // value: "{relay_url}",
                            placeholder: "wss://",
                            oninput: move |event| {
                            }
                        }
                        button {
                            class: "btn-circle btn-circle-false relay-url-del",
                            onclick: move |_| {
                              
                            },
                            div {
                                dangerous_inner_html: "{FALSE}"
                            }
                        }
                    }
                // }
                div {
                    class:"relay-url-item flex items-center",
                    input {
                        class: "relay-ipt mr-10",
                        r#type: "text",
                        placeholder: "wss://",
                        oninput: move |_| {
                            // tracing::info!("new_relay: {:?}", event.value());
                        }
                    }
                    button {
                        class: "btn-icon add relay-url-add",
                        onclick: move |_| {
                            
                        },
                        div {
                            dangerous_inner_html: "{ADD}"
                        }
                    }
                }
            }
        }
    
        }
       }
    }
}
