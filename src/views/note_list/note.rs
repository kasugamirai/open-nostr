use std::collections::HashMap;

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
    let subs_map = use_context::<Signal<HashMap<String, CustomSub>>>();

    let mut event = use_signal(|| props.event.clone());
    // let mut notetext = use_signal(|| props.event.content.clone());
    // let mut show_detail = use_signal(|| false);
    // let mut detail = use_signal(|| String::new());
    // let mut pk = use_signal(|| props.event.author().clone());
    // let mut eid = use_signal(|| props.event.id().clone());
    // let mut e_id = use_signal(|| eid().to_hex());
    let mut relay_name = use_signal(|| match props.relay_name.clone() {
        Some(s) => s,
        None => String::from("default"),
    });
    let mut element = use_signal(|| {
        rsx! {
            div {
                class: "pl-52",
               "Loading..."
            }
        }
    });
    let mut render_content = use_signal(|| String::from("Loading..."));
    let mut reply = use_signal(|| match TextNote::try_from(props.event.clone()) {
        Ok(text_note) => (text_note.is_reply(), Some(text_note)),
        Err(e) => {
            tracing::error!("parse event error: {:?}", e);
            (false, None)
        }
    });
    // update the event when the props.event changes
    use_effect(use_reactive(
        (
            &props.event,
            &props.sub_name,
            &props.is_tree,
            &props.clsname,
        ),
        move |(newest_event, subname, is_tree, clsname)| {
            {
                event.set(newest_event.clone());
                let subs_map = subs_map.read();
                if subs_map.contains_key(&subname) {
                    let sub = subs_map.get(&subname).unwrap();
                    relay_name.set(sub.relay_set.clone());
                }
            }
            {
                match TextNote::try_from(newest_event.clone()) {
                    Ok(text_note) => {
                        reply.set((text_note.is_reply(), Some(text_note)));
                    }
                    Err(e) => {
                        tracing::error!("parse event error: {:?}", e);
                        reply.set((false, None));
                    }
                }
            }
            {
                let is_repost = newest_event.clone().kind() == Kind::Repost;
                let data = {
                    if is_repost {
                        if newest_event.kind() == Kind::Repost {
                            match Event::from_json(&newest_event.content) {
                                Ok(event) => event.content.to_string(),
                                Err(e) => {
                                    tracing::error!("parse event error: {:?}", e);
                                    String::new()
                                }
                            }
                        } else {
                            String::new()
                        }
                    } else {
                        newest_event.content.to_string()
                    }
                };
                let is_highlight = {
                    is_tree
                        && clsname
                            .clone()
                            .unwrap_or("".to_string())
                            .contains("com-post--active")
                };
                render_content.set(data);
                // element.set(format_note_content(&data, &relay_name()));
                if is_highlight {
                    spawn(async move {
                        note_srcoll_into_view(&newest_event.id.to_hex()).await;
                    });
                }
            }
        },
    ));

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
                    pubkey: event().pubkey,
                    timestamp: event().created_at.as_u64(),
                    relay_name: relay_name.read().clone(),
                    repost_event: match event().kind() {
                        Kind::Repost => {
                            let repost_event = Event::from_json(&event().content).unwrap();
                            Some(repost_event)
                        }
                        _=> None
                    },
                }
                MoreInfo {
                    on_detail: move |_| {
                        // let json_value: serde_json::Value = serde_json::from_str(&props.event.as_json()).unwrap();
                        // let formatted_json = serde_json::to_string_pretty(&json_value).unwrap();
                        // detail.set(formatted_json);
                        // show_detail.set(!show_detail());
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
                }{
                    format_note_content(&render_content(), &relay_name())
                }
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
