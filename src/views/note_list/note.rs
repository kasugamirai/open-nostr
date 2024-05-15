use std::time::Duration;

use dioxus::prelude::*;
use nostr_sdk::{Client, EventId, Filter, JsonUtil};

use crate::{
    components::icons::*,
    components::Avatar,
    utils::format::{format_content, format_create_at, format_public_key},
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
    pub on_detail: Option<EventHandler<()>>,
    //pub metadata: nostr_sdk::Metadata,
    pub is_expand: Option<bool>
}
enum NoteAction {
    Replay,
    Share,
    Qoute,
    Zap,
}
struct NoteActionState {
    action: NoteAction,
    count: u64,
}
#[component]
pub fn Note(props: NoteProps) -> Element {
    let author = props.data.author.clone();
    let e = EventId::from_hex(author).unwrap();

    let future = use_resource(move || async move {
        let client = Client::default();
        // TODO: get metadata
        let filter = Filter::new().event(e);
        match client
            .get_events_of(vec![filter], Some(Duration::from_secs(30)))
            .await
        {
            Ok(events) => {
                if events.is_empty() {
                    Some("https://is.gd/hidYxs")
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    });

    let mut show_detail = use_signal(|| false);
    let mut detail = use_signal(|| String::new());
    let note_action_state = vec![
        NoteActionState {
            action: NoteAction::Replay,
            count: 100,
        },
        NoteActionState {
            action: NoteAction::Share,
            count: 10,
        },
        NoteActionState {
            action: NoteAction::Qoute,
            count: 10,
        },
        NoteActionState {
            action: NoteAction::Zap,
            count: 20,
        },
    ];


    // let 
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
                    nickname: None,
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
                class: "note-content font-size-16 word-wrap lh-26 pl-52",
                dangerous_inner_html: "{format_content(&props.data.content)}",
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
                    // Link {
                    //     class: "note-action-item cursor-pointer",
                    //     to: Route::NoteDetail { sub: props.sub_name, id: props.data.id.clone() },
                    //     span {
                    //         dangerous_inner_html: "{ADD}",
                    //     }
                    // }
                    // emojis
                    
                }

                if props.is_expand.unwrap_or(false) {
                    div {
                        "data-expand": props.data.id.clone(),
                        class: "note-action-expand cursor-pointer",
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
