use std::time::Duration;

use dioxus::prelude::*;
use nostr_sdk::{Client, Event, EventId, Filter, Keys};

use crate::{
    components::icons::*,
    state::subscription::CustomSub,
    utils::format::{format_content, format_create_at},
};

#[component]
pub fn Note(id: String) -> Element {
    let custom_sub_global = use_context::<Signal<CustomSub>>();
    let mut data = use_signal(Vec::<Event>::new);
    let get_events = move |id: String| {
        spawn(async move {
            let pk = "nsec1dmvtj7uldpeethalp2ttwscy32jx36hr9jslskwdqreh2yk70anqhasx64";
            // pk to hex
            let my_keys = Keys::parse(pk).unwrap();

            let client = Client::new(&my_keys);

            for i in custom_sub_global.read().relay_set.relays.iter() {
                client.add_relay(i.clone().as_str()).await.unwrap();
            }

            client.connect().await;

            let mut filter: Filter = Filter::new();
            filter = filter.limit(1);
            filter = filter.id(EventId::from_hex(id).unwrap());

            let events = client
                .get_events_of(vec![filter], Some(Duration::from_secs(30)))
                .await
                .unwrap();
            data.set(events);

            let _ = client.disconnect().await;
        });
    };

    rsx! {
        style {
            r#"
                .note-item {{
                    background-color: var(--bgc-0);
                    width: 100%;
                    border: 1px solid var(--boc-1);
                    border-radius: var(--radius-2);
                    padding: 10px;
                    display: flex;
                    flex-direction: column;
                    gap: 10px;
                    position: relative;
                }}
                .note-item .header {{
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    gap: 10px;
                }}
                .note-item .header .avatar {{
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    gap: 10px;
                }}
                .note-item .header .info {{
                    display: flex;
                    flex-direction: column;
                    justify-content: space-around;
                }}
                .note-item .main {{
                }}
                .note-item .footer {{
                    display: flex;
                    gap: 10px;
                }}
                .note-item .footer .left {{
                    display: flex;
                    gap: 20px;
                }}
                .note-item .footer .left .btn {{
                    display: flex;
                    align-items: center;
                    gap: 6px;
                }}
                .note-item .footer .left .btn svg {{
                    width: 20px;
                    height: 20px;
                }}
                .note-item .footer .left .btn .data {{
                    transform: translateY(-4px);
                }}
            "#
        }
        div {
            style: "max-width: 800px; white-space: wrap;",
            onmounted: move |_cx| {
                get_events(id.clone());
            },
            for i in data() {
                Item {
                    event: i.clone(),
                    reply: false,
                    index: 2,
                }
                Item {
                    event: i,
                    reply: true,
                    index: 1,
                }
            }
            for i in data() {
                Item {
                    event: i.clone(),
                    reply: false,
                    index: 2,
                }
                Item {
                    event: i,
                    reply: true,
                    index: 1,
                }
            }
        }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct ItemProps {
    event: Event,
    reply: bool,
    index: usize,
}

#[component]
fn Item(props: ItemProps) -> Element {
    let reply_style = if props.reply {
        format!("transform: translateY(-16px); z-index: {};", props.index)
    } else {
        format!("z-index: {}; border: 2px solid var(--boc-1);", props.index)
    };

    rsx! {
        div {
            class: "note-item",
            style: reply_style,
            div {
                class: "header",
                div {
                    class: "avatar",
                    div {
                        img {
                            style: "width: 50px; height: 50px; border-radius: var(--radius-circle);",
                            src: "https://file.service.ahriknow.com/avatar.jpg"
                        }
                    }
                    div {
                        class: "info",
                        div {
                            "Username"
                        }
                        div {
                            style: "font-size: 12px; color: var(--txt-3);",
                            "{format_create_at(props.event.created_at().as_u64())}"
                        }
                    }
                }
                div {

                }
            }
            div {
                class: "main",
                dangerous_inner_html: "{format_content(&props.event.content.to_string())}",
            }
            div {
                class: "footer",
                div {
                    class: "left",
                    div {
                        class: "btn",
                        span {
                            dangerous_inner_html: "{TURN_LEFT}",
                        }
                        span {
                            class: "data",
                            "5"
                        }
                    }
                    div {
                        class: "btn",
                        span {
                            dangerous_inner_html: "{TURN_RIGHT}",
                        }
                        span {
                            class: "data",
                            "2"
                        }
                    }
                    div {
                        class: "btn",
                        span {
                            dangerous_inner_html: "{MARKS}",
                        }
                        span {
                            class: "data",
                            "2"
                        }
                    }
                    div {
                        class: "btn",
                        span {
                            dangerous_inner_html: "{FLASH}",
                        }
                        span {
                            class: "data",
                            "40k"
                        }
                    }
                    div {
                        class: "btn",
                        span {
                            dangerous_inner_html: "{ADD}",
                        }
                    }
                }
            }
        }
    }
}
