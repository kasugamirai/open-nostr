use dioxus::prelude::*;
use nostr_sdk::{Event, JsonUtil, Kind};

use crate::{
    components::{icons::*, Avatar, ModalManager},
    nostr::note::TextNote,
    utils::{format::format_note_content, js::note_srcoll_into_view},
    views::note_list::reply::Reply,
    CustomSub, Route,
};

#[derive(PartialEq, Clone, Props)]
pub struct NoteProps {
    pub sub_name: String,
    pub event: Event,
    pub clsname: Option<String>,
    #[props(default = EventHandler::default())]
    pub on_expand: EventHandler<()>,
    #[props(default = false)]
    pub is_expand: bool,
    pub relay_name: Option<String>,
    pub note_index: Option<usize>,
    pub children: Option<Element>,
    #[props(default = false)]
    pub is_tree: bool,
}
#[component]
pub fn Note(props: NoteProps) -> Element {
    // let sub_name = use_signal(|| props.sub_name.clone());
    // let all_sub = use_context::<Signal<Vec<CustomSub>>>();
    // let subs = all_sub.read();
    // let current_sub = subs.iter().find(|s| s.name == sub_name()).unwrap();

    let mut show_detail = use_signal(|| false);
    let mut detail = use_signal(|| String::new());
    let mut event = use_signal(|| props.event.clone());
    let mut notetext = use_signal(|| props.event.content.clone());
    let mut pk = use_signal(|| props.event.author().clone());
    let mut eid = use_signal(|| props.event.id().clone());
    let mut e_id = use_signal(|| eid().to_hex());

    use_effect(use_reactive(&props.event, move |newest_event| {
        event.set(newest_event.clone());
        notetext.set(newest_event.content.clone());
        pk.set(newest_event.author().clone());
        eid.set(newest_event.id().clone());
        e_id.set(newest_event.id().clone().to_hex());
    }));

    let mut element = use_signal(|| {
        rsx! {
            div {
                class: "pl-52",
               "Loading..."
            }
        }
    });
    let repost_text = use_signal(|| {
        if props.event.kind() == Kind::Repost {
            match Event::from_json(&props.event.content) {
                Ok(event) => event.content.to_string(),
                Err(e) => {
                    tracing::error!("parse event error: {:?}", e);
                    // props.data.content.clone()
                    String::new()
                }
            }
        } else {
            String::new()
        }
    });
    let reply = use_signal(|| match TextNote::try_from(props.event.clone()) {
        Ok(text_note) => (text_note.is_reply(), Some(text_note)),
        Err(e) => {
            tracing::error!("parse event error: {:?}", e);
            (false, None)
        }
    });

    let optional_str_ref: String = match props.relay_name.clone() {
        Some(s) => s,
        None => String::from("default"),
    };
    let relay_name = use_signal(|| optional_str_ref.clone());
    let is_repost = props.event.kind() == Kind::Repost;
    let is_highlight = use_signal(|| {
        props.is_tree
            && props
                .clsname
                .clone()
                .unwrap_or("".to_string())
                .contains("com-post--active")
    });

    use_effect(use_reactive(&props.event, move |_| {
        let data = if is_repost {
            repost_text().clone()
        } else {
            notetext().clone()
        };
        element.set(format_note_content(&data, &relay_name()));
        spawn(async move {
            if is_highlight() {
                note_srcoll_into_view(&e_id()).await;
            };
        });
    }));

    let nav = navigator();
    let handle_nav = move |route: Route| {
        nav.push(route);
    };
    rsx! {
        div {
            class: format!("com-post p-6 {}", props.clsname.clone().unwrap_or("".to_string())),
            id: format!("note-{}", event.read().id().to_string()),
            div {
                class: "note-header flex items-start justify-between",
                Avatar {
                    pubkey: pk.read().clone(),
                    timestamp: props.event.created_at.as_u64(),
                    relay_name: &relay_name(),
                    repost_event: match props.event.kind() {
                        Kind::Repost => {
                            let repost_event = Event::from_json(&props.event.content).unwrap();
                            Some(repost_event)
                        }
                        _=> None
                    },
                }
                MoreInfo {
                    on_detail: move |_| {
                        let json_value: serde_json::Value = serde_json::from_str(&props.event.as_json()).unwrap();
                        let formatted_json = serde_json::to_string_pretty(&json_value).unwrap();
                        detail.set(formatted_json);
                        show_detail.set(!show_detail());
                    },
                }
            }
            div {
                class: "note-content font-size-16 word-wrap lh-26",
                onclick: move |_| {
                    let _reply = reply();
                    if _reply.0 {
                        let text_note = _reply.1.as_ref().unwrap();
                        handle_nav(Route::NoteDetail {
                            sub: urlencoding::encode(&props.sub_name.clone()).to_string(),
                            root_id: text_note.get_root().unwrap().to_hex(),
                            note_id: event.read().id().to_hex(), // 使用克隆的 event
                        });
                    } else {
                        handle_nav(Route::NoteDetail {
                            sub: urlencoding::encode(&props.sub_name.clone()).to_string(),
                            root_id: event.read().id().to_hex(), // 使用克隆的 event
                            note_id: event.read().id().to_hex(), // 使用克隆的 event
                        });
                    }
                },
                if reply().0 && !props.is_tree {
                    Reply {
                        event: event.read().clone(),
                        sub_name: props.sub_name.clone(),
                        relay_name: props.relay_name.clone().unwrap_or("default".to_string()),
                    }
                }
                {element}
            }

            div {
                class: "note-action-wrapper flex items-center justify-between pl-52 pr-12",
                div {
                    class: "note-action flex items-center",
                    div {
                        class: "note-action-item cursor-pointer flex items-center",
                        span {
                            class: "note-action-icon",
                            dangerous_inner_html: "{TURN_LEFT}"
                        }
                    }
                    div {
                        class: "note-action-item cursor-pointer flex items-center",
                        span {
                            class: "note-action-icon",

                            // dangerous_inner_html: match _state.action {
                            //     NoteAction::Replay => TURN_LEFT.to_string(),
                            //     NoteAction::Share => TURN_RIGHT.to_string(),
                            //     NoteAction::Qoute => QUTE.to_string(),
                            //     NoteAction::Zap => ZAP.to_string(),
                            // }
                            dangerous_inner_html: "{TURN_RIGHT}"
                        }
                        // span {
                        //     class: "note-action-count font-size-12 txt-1",
                        //     {format!("{}", _state.count)}
                        // }
                    }
                    div {
                        class: "note-action-item cursor-pointer flex items-center",
                        span {
                            class: "note-action-icon",
                            dangerous_inner_html: "{QUTE}"
                        }
                    }
                    div {
                        class: "note-action-item cursor-pointer flex items-center",
                        span {
                            class: "note-action-icon",
                            dangerous_inner_html: "{ZAP}"
                        }
                    }

                    // span{
                    //     class: "note-action-wrapper-span ml-10",
                    // }
                }

                if props.is_expand {
                    div {
                        // "data-expand": props.event.id().to_string(),
                        class: "note-action-expand cursor-pointer",
                        onclick: move |_| {
                            props.on_expand.call(());
                        },
                        span {
                            dangerous_inner_html: "{DOWN}",
                        }
                    }
                }
                div{
                    style: "display: none",
                    {event().clone().as_json()}
                }

            }
        }
    }
}

#[component]
pub fn MoreInfo(on_detail: EventHandler<()>) -> Element {
    let mut edit = use_signal(|| false);
    let mut modal_manager = use_context::<Signal<ModalManager>>();
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
    let popover = {
        rsx! {
            div {
                class: "show-true note-more-box",
                div {
                    class:"note-more-content-box",
                    div {
                        class: "note-more-button",
                        onclick: move |_| {
                            edit.set(false);
                        },
                        div {
                            dangerous_inner_html: "{SHARE}"
                        }
                        "Share"
                    }
                    div {
                        class: "note-more-button",
                        onclick: move |_| {
                            edit.set(false);
                        },
                        div {
                            dangerous_inner_html: "{STAR}"
                        }
                        "Book Mark"
                    }
                    div {
                        class: "note-more-button",
                        onclick: move |_| {
                            edit.set(false);
                        },
                        div {
                            dangerous_inner_html: "{STATION}"
                        }
                        "Broadcast"
                    }
                    div {
                        class: "note-more-button",
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
    };
    rsx! {
        div {
            onclick: move |event| {
                // Save the coordinates of the event relative to the screen
                pos.set(event.screen_coordinates().to_tuple());
            },
            class: "relative",
            div {
                class: "more-trigger",
                div {
                    onclick: move |e| {
                        let (x, y) = e.page_coordinates().to_tuple();
                        edit.set(!edit());
                        let popover_modal_id = modal_manager.write().add_popover(popover.clone(), (x, y));
                        modal_manager.write().open_modal(&popover_modal_id);
                    },
                    dangerous_inner_html: "{MORE}"
                }
            }

        }
    }
}
