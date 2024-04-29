use std::time::Duration;

use dioxus::prelude::*;
use nostr_sdk::{prelude::*, JsonUtil};
use serde_json::to_string_pretty;
use tracing::debug;
use tracing_subscriber::field::debug;

use crate::{
    components::{icons::FALSE, Button, Post, PostData},
    state::subscription::CustomSub,
};

#[component]
pub fn Home() -> Element {
    let custom_sub_global = use_context::<Signal<CustomSub>>();
    let client = use_context::<Signal<Client>>();
    let mut post_datas = use_signal(Vec::<PostData>::new);
    let mut btn_text = use_signal(|| String::from("Get Events"));

    let mut get_events = move |filters: Vec<Filter>| {
        if btn_text() == "Get Events" {
            btn_text.set("Loading ...".to_string());
            spawn(async move {
                let events_result = client
                    .read()
                    .get_events_of(filters, Some(Duration::from_secs(30)))
                    .await;

                match events_result {
                    Ok(events) => {
                        post_datas.clear();
                        for (i, event) in events.iter().enumerate() {
                            let post_data = PostData {
                                id: event.id().to_hex(),
                                author: event.author().to_hex(),
                                created_at: event.created_at().as_u64(),
                                kind: "".to_string(),
                                tags: vec![],
                                content: event.content.to_string(),
                                index: i,
                                event: event.clone(),
                            };
                            post_datas.push(post_data);
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to get events: {}", e);
                    }
                }

                btn_text.set("Get Events".to_string());
            });
        }
    };

    use_effect(move || {
        let cus = custom_sub_global();
        debug!("{}", cus.name);
    });

    let handle_get_events = move |_| {
        get_events(custom_sub_global.read().to_sub());
    };

    let mut show_detail = use_signal(|| String::new());

    let json_format = move |data: String| {
        spawn(async move {
            let mut eval = eval(
                r#"
                    let data = await dioxus.recv()
                    let res = JSON.stringify(JSON.parse(data), null, 18)
                    dioxus.send(res)
                "#,
            );
            eval.send(data.into()).unwrap();
            if let Value::String(res) = eval.recv().await.unwrap() {
                show_detail.set(res);
            }
        });
    };

    rsx! {
        ul {
            onmounted: move |_| {
                debug!("onmounted");
                get_events(custom_sub_global.read().to_sub());
            },
            style: "display: flex; flex-direction: column; gap: 10px;",
            for (i, p) in post_datas().iter().enumerate() {
                Post {
                    data: p.clone(),
                    on_detail: move |_| {
                        let data: Value = serde_json::from_str(&post_datas()[i].event.as_json()).expect("Failed to parse JSON");
                        let pretty_json = to_string_pretty(&data).expect("Failed to format JSON");
                        json_format(pretty_json);
                    },
                }
            }
            div {
                style: format!("z-index: 999999999; position: fixed; top: 0; right: 0; bottom: 0; left: 0; background-color: rgba(0, 0, 0, 0.5); {}", if show_detail.read().is_empty() { "display: none;" } else { "display: block;" }),
                div {
                    style: "background-color: var(--bgc-0); border-radius: var(--radius-1); padding: 20px; position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%);",
                    button {
                        style: "position: absolute; top: -10px; left: -10px; border: none; background-color: var(--col-error); border-radius: 50%; width: 32px; height: 32px; cursor: pointer; display: flex; align-items: center; justify-content: center;",
                        onclick: move |_| {
                            show_detail.set(String::new());
                        },
                        dangerous_inner_html: "{FALSE}"
                    }
                    textarea {
                        style: "width: 700px; height: 500px; resize: none;",
                        readonly: true,
                        wrap: "off",
                        value: "{show_detail}",
                    }
                }
            }
        }
        br {}
        Button { on_click: handle_get_events, "{btn_text}" }
    }
}
