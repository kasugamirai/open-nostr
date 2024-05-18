use std::collections::HashMap;

use dioxus::prelude::*;
use nostr_sdk::{EventId, FromBech32, JsonUtil};
use regex::Regex;

use crate::{
    components::{icons::*, Avatar},
    nostr::{
        fetch::{get_event_by_id, get_metadata, get_reactions, get_replies},
        multiclient::MultiClient,
    },
    utils::format::{format_content, format_create_at},
    Route,
};

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
    let notetext = use_signal(|| props.data.content.clone());
    let sub_name = use_signal(|| props.sub_name.clone());
    let pk = use_signal(|| props.data.event.author().clone());
    let eid = use_signal(|| props.data.event.id().clone());
    let mut root_avatar = use_signal(|| None);
    let mut root_nickname = use_signal(|| None);
    let mut emoji = use_signal(|| HashMap::new());
    // let optional_str_ref: Option<&str> = props.relay_name;
    let _future = use_resource(move || async move {
        let clients = multiclient();
        // if props.relay_name != None {
        //     return;
        // }
        
        // TODO: relay_name
        let client = clients.get("default").unwrap();

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

        match get_metadata(&client, &pk(), None).await {
            Ok(metadata) => {
                root_avatar.set(metadata.picture);
                root_nickname.set(metadata.name);
            }
            Err(_) => {
                tracing::info!("metadata not found");
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

                        let mut nickname = "".to_string();
                        let mut avatar = "".to_string();

                        match get_metadata(&client, &pk, None).await {
                            Ok(metadata) => {
                                nickname = metadata.name.unwrap_or("Default".to_string());
                                avatar = match metadata.picture {
                                    Some(picture) => {
                                        if picture.is_empty() {
                                            "https://avatars.githubusercontent.com/u/1024025?v=4"
                                                .to_string()
                                        } else {
                                            picture
                                        }
                                    }
                                    None => "https://avatars.githubusercontent.com/u/1024025?v=4"
                                        .to_string(),
                                }
                            }
                            Err(_) => {
                                tracing::info!("metadata not found");
                            }
                        }

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
                                div {
                                    style: "width: 140px; display: flex; align-items: center; gap: 12px;",
                                    img {
                                        class: "square-40 radius-20 mr-12",
                                        src: avatar,
                                        alt: "avatar",
                                    }
                                    div {
                                        class: "profile flex flex-col",
                                        span {
                                            class: "nickname font-size-16 txt-1",
                                            {nickname}
                                        }
                                        span {
                                            class: "created txt-3 font-size-12",
                                            {format_create_at(timestamp)}
                                        }
                                    }
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
                    pubkey: props.data.author.clone(),
                    timestamp: props.data.created_at,
                    avatar: root_avatar(),
                    nickname: root_nickname(),
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
                    handle_nav(Route::NoteDetail { sub: props.sub_name.clone(), id: props.data.id.clone() });
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
