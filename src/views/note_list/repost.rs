use std::collections::HashMap;

use dioxus::prelude::*;
use nostr_sdk::{EventId, FromBech32, JsonUtil};
use regex::Regex;
use web_sys::console;

use crate::{
    components::{icons::*, Avatar},
    nostr::{
        fetch::{get_event_by_id, get_metadata, get_reactions, get_replies},
        multiclient::MultiClient,
    },
    utils::format::{format_content, format_create_at},
    Route,
};
struct RepostProps {

}

#[component]
pub fn Repost(props: RepostProps) -> Element {

    let multiclient = use_context::<Signal<MultiClient>>();

    let mut show_detail = use_signal(|| false);
    let mut detail = use_signal(|| String::new());

    let mut element = use_signal(|| {
        rsx! {
            div {
                class: "pl-52",
               "Loading..."
            }
        }
    });
    let notetext = use_signal(|| props.data.content.clone());
    let sub_name = use_signal(|| props.sub_name.clone());
    let pk = use_signal(|| props.data.event.author().clone());
    let eid = use_signal(|| props.data.event.id().clone());
    let mut emoji = use_signal(|| HashMap::new());
    let optional_str_ref: String = match props.relay_name.clone() {
        Some(s) => s,
        None => String::from("default"),
    };
    let relay_name = use_signal(|| optional_str_ref.clone());
    let _future = use_resource(move || async move {
        let clients = multiclient();
        console::log_1(&"Fetching events...".into());

        let client = clients.get(&relay_name()).unwrap();

        match get_reactions(&client, &eid(), None).await {
            Ok(emojis) => {
                emoji.set(emojis);
            }
            Err(_) => {
                tracing::info!("metadata not found");
            }
        }

        match get_replies(&client, &eid(), None).await {
            Ok(replies) => {
                let mut action_state = note_action_state.write();
                action_state[0].count = replies.len();
            }
            Err(e) => {
                tracing::error!("replies not found: {:?}", e);
            }
        }

        let re = Regex::new(r"(nostr:note[a-zA-Z0-9]{59})").unwrap();

        let data = &notetext();

        let mut parts = Vec::new();
        let mut last_end = 0;

        for mat in re.find_iter(data) {
            if mat.start() > last_end {
                parts.push(&data[last_end..mat.start()]);
            }
            parts.push(mat.as_str());
            last_end = mat.end();
        }

        if last_end < data.len() {
            parts.push(&data[last_end..]);
        }

        let mut elements = vec![];
        for i in parts {
            if i.starts_with("nostr:note") {
                let id = i.strip_prefix("nostr:").unwrap();

                match get_event_by_id(&client, &EventId::from_bech32(id).unwrap(), None).await {
                    Ok(Some(event)) => {
                        let mut action_state = note_action_state.write();
                        action_state[2].count += 1;
                        let pk = event.author();
                        let content = event.content.to_string();
                        let timestamp = event.created_at.as_u64();

                        elements.push(rsx! {
                        div {
                            class: "quote",
                            style: "display: flex; align-items: center;",
                            div {
                                style: "font-weight: bold; width: 52px; display: flex; align-items: center; justify-content: center;",
                                "Qt:"
                            }
                            div {
                                style: "flex: 1; border: 1px solid #333; border-radius: 20px; overflow: hidden; padding: 4px; display: flex; gap: 12px; background: #fff; height: 50px;",
                                Avatar {
                                    pubkey: pk,
                                    timestamp: timestamp,
                                    relay_name: relay_name.clone(),
                                }
                                div {
                                    style: "flex: 1; font-size: 14px; line-height: 20px; border-left: 2px solid #b4b4b4; padding: 0 12px;",
                                    dangerous_inner_html: "{content}"
                                }
                            }
                        }
                        });
                    }
                    Ok(None) => {
                        tracing::info!("event not found");
                    }
                    Err(e) => {
                        tracing::error!("{:?}", e);
                    }
                }
            } else {
                elements.push(rsx! {
                    div {
                        class: "text pl-52",
                        dangerous_inner_html: "{format_content(i)}"
                    }
                });
            }
        }

        element.set(rsx! {
            for element in elements {
                {element}
            }
        });
    });

    let nav = navigator();
    let handle_nav = move |route: Route| {
        nav.push(route);
    };

    rsx! {
        div {
            class: format!("com-post p-6 {}", props.clsname.as_deref().unwrap_or("")),
            id: format!("note-{}", props.data.id),
            // detail modal
            div {
                style: format!("position: fixed; top: 0; left: 0; right: 0; bottom: 0; background-color: rgba(0, 0, 0, 0.5); z-index: 99999999; display: {};", if *show_detail.read() { "block" } else { "none" }),
                div {
                    style: "width: 50%; height: 60%; max-width: 900px; background-color: #fff; position: fixed; top: 50%; left: 50%; transform: translate(-50%, -50%); padding: 20px; border-radius: 10px;",
                    button {
                        class: "btn-icon remove",
                        style: "position: absolute; top: -12px; left: -12px;",
                        onclick: move |_| {
                            show_detail.set(false);
                        },
                        dangerous_inner_html: "{FALSE}",
                    }
                    pre {
                        style: "height: 100%; overflow-y: auto; font-size: 16px;",
                        "{detail}"
                    }
                }
            }
            div {
                class: "note-header flex items-start justify-between",
                Avatar {
                    pubkey: pk.read().clone(),
                    timestamp: props.data.created_at,
                    relay_name: props.relay_name.clone().unwrap_or("default".to_string()),
                }
                // MoreInfo {
                //     on_detail: move |_| {
                //         let json_value: serde_json::Value = serde_json::from_str(&props.data.event.as_json()).unwrap();
                //         let formatted_json = serde_json::to_string_pretty(&json_value).unwrap();
                //         detail.set(formatted_json);
                //         show_detail.set(!show_detail());
                //     },
                // }
            }
            div {
                class: "note-content font-size-16 word-wrap lh-26",
                onclick: move |_| {
                    handle_nav(Route::NoteDetail { sub: urlencoding::encode(&props.sub_name.clone()).to_string(), id: props.data.id.clone() });
                },
                {element}
            }
            
        }
    }
}