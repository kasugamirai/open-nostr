use std::{collections::HashMap, time::Duration};

use dioxus::prelude::*;
use nostr_sdk::{prelude::*, JsonUtil};
use serde_json::to_string_pretty;

use crate::{
    components::{icons::FALSE, Button, Post, PostData},
    state::subscription::CustomSub,
};

#[component]
pub fn Home() -> Element {
    let mut all_events = use_context::<Signal<HashMap<String, Vec<nostr_sdk::Event>>>>();
    let cur = use_context::<Signal<usize>>();
    let subs = use_context::<Signal<Vec<CustomSub>>>();
    let client = use_context::<Signal<Client>>();

    let mut post_datas = use_signal(Vec::<PostData>::new);
    let mut btn_text = use_signal(|| String::from("Get Events"));

    let mut get_events = move || {
        let index = cur();
        let subs = subs();
        if index < subs.len() {
            let sub = subs[index].clone();
            let filters = sub.to_sub();
            btn_text.set("Loading ...".to_string());
            spawn(async move {
                let events_result = client
                    .read()
                    .get_events_of(filters, Some(Duration::from_secs(30)))
                    .await;

                match events_result {
                    Ok(events) => {
                        let mut als = all_events.write();
                        if let Some(all_events) = als.get_mut(&sub.name) {
                            all_events.extend(events.clone());
                        } else {
                            als.insert(sub.name.clone(), events.clone());
                            post_datas.clear();
                        }
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

    let handle_get_events = move |_| {
        get_events();
    };

    let mut show_detail = use_signal(String::new);

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

    use_effect(use_reactive(
        (&post_datas, &subs, &all_events),
        move |(mut post_datas, subs, all_events)| {
            tracing::info!("======== update post_datas {}", cur());
            let index = cur();
            let subs = subs();
            if index < subs.len() {
                let sub = subs[index].clone();
                post_datas.clear();

                if let Some(events) = all_events().get(&sub.name) {
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
                } else {
                    get_events();
                }
            }
        },
    ));

    rsx! {
        ul {
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
