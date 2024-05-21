use std::collections::HashMap;

use dioxus::prelude::*;
use nostr_sdk::{Event, EventId, FromBech32, JsonUtil, Kind};
use regex::Regex;
use web_sys::console;

use crate::{
    components::{icons::*, Avatar},
    nostr::{
        fetch::{get_event_by_id, get_metadata, get_reactions, get_replies},
        multiclient::MultiClient,
        utils::is_note_address,
    },
    utils::format::{format_content, format_create_at},
    Route,
};

use super::quote::Quote;

#[derive(PartialEq, Clone)]
pub struct NoteData {
    pub id: String,
    pub author: String,
    pub created_at: u64,
    pub kind: String,
    pub tags: Vec<String>,
    pub content: String,
    pub index: usize,
    pub event: nostr_sdk::Event,
}

impl NoteData {
    pub fn from(event: &nostr_sdk::Event, index: usize) -> Self {
        NoteData {
            id: event.id().to_hex(),
            author: event.author().to_hex(),
            created_at: event.created_at().as_u64(),
            kind: "".to_string(),
            tags: vec![],
            content: event.content.to_string(),
            index,
            event: event.clone(),
        }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct NoteProps {
    pub sub_name: String,
    pub data: NoteData,
    pub clsname: Option<String>,
    #[props(default = EventHandler::default())]
    pub on_expand: EventHandler<()>,
    pub is_expand: Option<bool>,
    pub relay_name: Option<String>,
}
enum NoteAction {
    Replay,
    Share,
    Qoute,
    Zap,
}
struct NoteActionState {
    action: NoteAction,
    count: usize,
}
#[component]
pub fn Note(props: NoteProps) -> Element {
    let mut note_action_state = use_signal(|| {
        vec![
            NoteActionState {
                action: NoteAction::Replay,
                count: 0,
            },
            NoteActionState {
                action: NoteAction::Share,
                count: 0,
            },
            NoteActionState {
                action: NoteAction::Qoute,
                count: 0,
            },
            NoteActionState {
                action: NoteAction::Zap,
                count: 0,
            },
        ]
    });

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
    tracing::info!("note data: {:#?}", props.data.event.tags());
    let notetext = use_signal(|| props.data.content.clone());
    let repost_text = use_signal(|| if props.data.event.kind() == Kind::Repost {
        match Event::from_json(&props.data.content) {
            Ok(event) => event.content.to_string(),
            Err(e) => {
                tracing::error!("parse event error: {:?}", e);
                // props.data.content.clone()
                String::new()
            }
        }
    } else {
        String::new()
    });
    let sub_name = use_signal(|| props.sub_name.clone());
    let pk = use_signal(|| props.data.event.author().clone());
    let eid = use_signal(|| props.data.event.id().clone());
    let mut emoji = use_signal(|| HashMap::new());
    let optional_str_ref: String = match props.relay_name.clone() {
        Some(s) => s,
        None => String::from("default"),
    };
    let relay_name = use_signal(|| optional_str_ref.clone());
    let is_repost = props.data.event.kind() == Kind::Repost;
    let _future = use_resource(move || async move {
        let clients = multiclient();
        console::log_1(&"Fetching events...".into());

        let client = clients.get(&relay_name()).unwrap();
        // let repost_evnet =

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

        let re: Regex = Regex::new(r"(nostr:note[a-zA-Z0-9]{59})").unwrap();

        let data = if is_repost { repost_text().clone() } else { notetext().clone() };

        let mut parts = Vec::new();
        let mut last_end = 0;

        for mat in re.find_iter(&data) {
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
                let is_note = is_note_address(i);
                if is_note {
                    let mut action_state = note_action_state.write();
                    action_state[2].count += 1;
                    elements.push(rsx! {
                        Quote {
                            event_id: EventId::from_bech32(id).unwrap().clone(),
                            relay_name: relay_name.clone(),
                            quote_nostr: i.to_string(),
                        }
                    })
                } else {
                    elements.push(rsx! {
                        span {
                            "{i}"
                        }
                    });
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
        // nav.push(route);
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
                    repost_event: match props.data.event.kind() {
                        Kind::Repost => {
                            let repost_event = Event::from_json(&props.data.content).unwrap();
                            Some(repost_event)
                        }
                        _=> None
                    },
                }
                MoreInfo {
                    on_detail: move |_| {
                        let json_value: serde_json::Value = serde_json::from_str(&props.data.event.as_json()).unwrap();
                        let formatted_json = serde_json::to_string_pretty(&json_value).unwrap();
                        detail.set(formatted_json);
                        show_detail.set(!show_detail());
                    },
                }
            }
            div {
                class: "note-content font-size-16 word-wrap lh-26",
                onclick: move |_| {
                    handle_nav(Route::NoteDetail { sub: urlencoding::encode(&props.sub_name.clone()).to_string(), id: props.data.id.clone() });
                },
                {element}
            }

            div {
                class: "note-action-wrapper flex items-center justify-between pl-52 pr-12",
                div {
                    class: "note-action flex items-center",
                    {note_action_state.iter().map(|_state| {
                        rsx! {
                            div {
                                class: "note-action-item cursor-pointer flex items-center",
                                span {
                                    class: "note-action-icon",
                                    dangerous_inner_html: match _state.action {
                                        NoteAction::Replay => TURN_LEFT.to_string(),
                                        NoteAction::Share => TURN_RIGHT.to_string(),
                                        NoteAction::Qoute => QUTE.to_string(),
                                        NoteAction::Zap => ZAP.to_string(),
                                    }
                                }
                                span {
                                    class: "note-action-count font-size-12 txt-1",
                                    {format!("{}", _state.count)}
                                }
                            }
                        }
                    })}
                    span{
                        style: "height: 24px; width: 3px; background-color: var(--txt-3); margin-left: 10px;",
                    }
                    div {
                        class: "note-action-item cursor-pointer flex items-center",
                        span {
                            class: "note-action-icon",
                            dangerous_inner_html: "{ADD}"
                        }
                    }
                    for (k, v) in emoji().iter() {
                        div {
                            class: "note-action-item cursor-pointer flex items-center",
                            span {
                                class: "note-action-icon",
                                "{k}"
                            }
                            span {
                                class: "note-action-count font-size-12 txt-1",
                                "{v}"
                            }
                        }
                    }
                }

                if props.is_expand.unwrap_or(false) {
                    div {
                        "data-expand": props.data.id.clone(),
                        class: "note-action-expand cursor-pointer",
                        onclick: move |_| {
                            props.on_expand.call(());
                        },
                        span {
                            dangerous_inner_html: "{DOWN}",
                        }
                    }
                }

            }
        }
    }
}

#[component]
pub fn MoreInfo(on_detail: EventHandler<()>) -> Element {
    let mut edit = use_signal(|| false);

    // close when click outside
    let root_click_pos = use_context::<Signal<(f64, f64)>>();
    let mut pos: Signal<(f64, f64)> = use_signal(|| root_click_pos());
    use_effect(use_reactive((&pos,), move |(pos,)| {
        // The coordinates of root element
        let root_pos = root_click_pos();

        // The coordinates of current element
        let current_pos = pos();

        // Determine if two coordinates are the same
        if current_pos.0 != root_pos.0 || current_pos.1 != root_pos.1 {
            edit.set(false);
        }
    }));

    rsx! {
        div {
            onclick: move |event| {
                // Save the coordinates of the event relative to the screen
                pos.set(event.screen_coordinates().to_tuple());
            },
            style: "position: relative;",
            div {
                class: "more-trigger",
                div {
                    onclick: move |_| {
                        edit.set(!edit());
                    },
                    dangerous_inner_html: "{MORE}"
                }
            }
            div {
                class: "show-{edit}",
                style: "position: absolute; right: 0; background-color: var(--bgc-0); border-radius: var(--radius-1); display: flex; flex-direction: column; gap: 10px; padding: 10px; 20px; border: 1px solid var(--boc-1); z-index: 100;",
                div {
                    style: "display: flex; flex-direction: column; gap: 10px; padding: 10px; 20px; width: 140px;",
                    div {
                        style: "display: flex; align-items: center; gap: 5px; cursor: pointer;",
                        onclick: move |_| {
                            edit.set(false);
                        },
                        div {
                            dangerous_inner_html: "{SHARE}"
                        }
                        "Share"
                    }
                    div {
                        style: "display: flex; align-items: center; gap: 5px; cursor: pointer;",
                        onclick: move |_| {
                            edit.set(false);
                        },
                        div {
                            dangerous_inner_html: "{STAR}"
                        }
                        "Book Mark"
                    }
                    div {
                        style: "display: flex; align-items: center; gap: 5px; cursor: pointer;",
                        onclick: move |_| {
                            edit.set(false);
                        },
                        div {
                            dangerous_inner_html: "{STATION}"
                        }
                        "Broadcast"
                    }
                    div {
                        style: "display: flex; align-items: center; gap: 5px; cursor: pointer;",
                        onclick: move |_| {
                            on_detail.call(());
                            edit.set(false);
                        },
                        div {
                            dangerous_inner_html: "{INFO}"
                        }
                        "Details"
                    }
                }
            }
        }
    }
}
