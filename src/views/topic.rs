use std::time::Duration;

use dioxus::prelude::*;
use nostr_sdk::{Client, Event, Filter, JsonUtil, Keys, Kind};
use serde_json::Value;

use crate::{
    components::icons::*,
    state::subscription::CustomSub,
    utils::format::{format_content, format_create_at},
};

#[component]
pub fn Topic() -> Element {
    let custom_sub_global = use_context::<Signal<CustomSub>>();
    let mut data = use_signal(|| Vec::<Event>::new());
    let get_events = move || {
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
            filter = filter.kind(Kind::TextNote);
            // filter = filter.event(
            //     EventId::from_hex(
            //         "6b5fde93c86ed41b25ad9c133b9117be323d1eeeb4ee46265dd6b6e72de2a25e",
            //     )
            //     .unwrap(),
            // );

            let events = client
                .get_events_of(vec![filter], Some(Duration::from_secs(30)))
                .await
                .unwrap();
            data.set(events);

            let _ = client.disconnect().await;
        });
    };

    let value: Value = serde_json::from_str(
        r#"{
            "id": "e2f29fe3b18a1b7849869b408039b57f35fcaa40fcedcb65d76b1e214da12f49",
            "pubkey": "6d088b653a1bffe728b9b17e5c7afcfc18d85f70502feac83400524eb6a8d5e9",
            "created_at": 1714027891,
            "kind": 1,
            "tags": [
                [
                    "e",
                    "dcb8239faa514f6748fc161e09d94b672256f007d3d1e9099fc7e2ddc0ffdf06",
                    "wss://bostr.lecturify.net/",
                    "root"
                ],
                [
                    "e",
                    "0d8a0c9b6f3755288d942ccbd52c31572168ad8393fadc7706b26ebdd9c99fff",
                    "wss://bostr.lecturify.net/",
                    "reply"
                ],
                [
                    "p",
                    "583d76d7aa93b73a75b0e3911187664ba85e35b15fb31dc07bbb8dce55ead165"
                ],
                [
                    "p",
                    "07f48c2e46883be8e816d452f5bb6a0acd95b75c06bed88d117feea602aa2052"
                ],
                [
                    "p",
                    "6d088b653a1bffe728b9b17e5c7afcfc18d85f70502feac83400524eb6a8d5e9"
                ]
            ],
            "content": "没动力学了😅",
            "sig": "419e29563f70634d15f3f7390933640d53ef487bbf19edf874a1d98b62c394c900f8bffa8f63194d02c0d2566be08607e221c24a2a9543dd1b0f5cd18e5098fb"
        }"#,
    ).unwrap();

    let event = Event::from_value(value).unwrap();

    rsx! {
        style {
            r#"
                .topic-item {{
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
                .topic-item .header {{
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    gap: 10px;
                }}
                .topic-item .header .avatar {{
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    gap: 10px;
                }}
                .topic-item .header .info {{
                    display: flex;
                    flex-direction: column;
                    justify-content: space-around;
                }}
                .topic-item .main {{
                    display: flex;
                    gap: 10px;
                    padding-left: 60px;
                }}
                .topic-item .footer {{
                    display: flex;
                    gap: 10px;
                }}
                .topic-item .footer .left {{
                    display: flex;
                    gap: 20px;
                }}
                .topic-item .footer .left .btn {{
                    display: flex;
                    align-items: center;
                    gap: 6px;
                }}
                .topic-item .footer .left .btn svg {{
                    width: 20px;
                    height: 20px;
                }}
                .topic-item .footer .left .btn .data {{
                    transform: translateY(-4px);
                }}
            "#
        }
        div {
            style: "max-width: 800px; white-space: wrap;",
            onmounted: move |_cx| {
                get_events();
            },
            h1 { "Topic" }
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
            class: "topic-item",
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
