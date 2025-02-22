use std::collections::HashMap;
use dioxus::prelude::*;
use nostr_sdk::{Event, JsonUtil, Kind};
use crate::components::{
    icons::*,
    Avatar
};
use crate::init::MODAL_MANAGER;
use crate::nostr::get_reactions;
use crate::nostr::MultiClient;
use crate::nostr::{ReplyTreeManager, TextNote};
use crate::utils::format::format_note_content;
use crate::utils::js::note_srcoll_into_view;
use crate::views::note_list::detail_modal::DetailModal;
use crate::views::note_list::reply::Reply;
use crate::{CustomSub, Route};

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
    let subs_map: Signal<HashMap<String, CustomSub>> = use_context::<Signal<HashMap<String, CustomSub>>>();
    let multiclient = use_context::<Signal<MultiClient>>();
    let reply_tree_manager = use_context::<Signal<ReplyTreeManager>>();

    let mut event = use_signal(|| props.event.clone());
    
    let mut relay_name = use_signal(|| match props.relay_name.clone() {
        Some(s) => s,
        None => String::from("default"),
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

    //loading replay count
    let mut replay_count = use_signal(|| 0usize);
    use_effect(move || {
        let _root_id = event.read().id();
        let manager_lock = reply_tree_manager.read();
        let replies = manager_lock.get_replies(&_root_id);
        if replies.len() > 0 {
            replay_count.set(replies.len());
        }
    });

    //loading reactions
    let mut reactions_maps: Signal<HashMap<String, i32>> = use_signal(|| HashMap::new());
    use_effect(use_reactive(
        (&props.is_tree, &props.sub_name, &props.event.id),
        move |(is_tree, sub_name, eid)| {
            spawn(async move {
                let _subs_map: HashMap<String, CustomSub> = subs_map();
                if !_subs_map.contains_key(&sub_name) {
                    return;
                }
                let sub = _subs_map.get(&sub_name).unwrap();
                let clients = multiclient();
                let client_result = clients.get_or_create(&sub.relay_set).await;
                match client_result {
                    Ok(hc) => {
                        let client: std::sync::Arc<nostr_sdk::Client> = hc.client();
                        match get_reactions(&client, &eid, None, is_tree).await {
                            Ok(reactions) => {
                                if reactions.len() > 0 {
                                    reactions_maps.set(reactions);
                                }
                            }
                            Err(e) => {
                                tracing::error!("get reactions error: {:?}", e);
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("reactions client Error: {:?}", e);
                    }
                }
            });
        },
    ));

    let nav = navigator();
    let handle_nav = move |route: Route| {
        nav.push(route);
    };
    rsx! {
        div {
            key: "{event().id().to_hex()}",
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
                        let json_value: serde_json::Value = serde_json::from_str(&props.event.as_json()).unwrap();
                        let formatted_json = serde_json::to_string_pretty(&json_value).unwrap();
                        let _id = event().id().to_hex();
                            let modal_id = MODAL_MANAGER.write().add_modal(rsx! {
                                DetailModal {
                                    detail: formatted_json,
                                    id: _id.clone(),
                                }
                            }, _id.clone());
                            MODAL_MANAGER.write().open_modal(&modal_id);
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
                            note_id: event.read().id().to_hex(), // Use clone event
                        });
                    } else {
                        handle_nav(Route::NoteDetail {
                            sub: urlencoding::encode(&props.sub_name.clone()).to_string(),
                            root_id: event.read().id().to_hex(), // Use clone event
                            note_id: event.read().id().to_hex(), // Use clone event
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
                        if *replay_count.read()>0 {
                           span {
                                class: "note-action-count",
                                {replay_count.to_string()}
                            }
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

                    //split div
                    div {
                        class: "note-action-item cursor-pointer flex items-center split-bar"
                    }

                    //reactions div
                    for (reaction, count) in reactions_maps.read().iter() {
                        div {
                            class: "note-action-item cursor-pointer flex items-center",
                            span {
                                class: "note-action-icon",
                                dangerous_inner_html: "{reaction}"
                            }
                            span {
                                class: "note-action-icon",
                                dangerous_inner_html: "{count}"
                            }
                        }
                    }
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
pub fn MoreInfo(on_detail: EventHandler<dioxus::prelude::Event<MouseData>>) -> Element {
    let mut edit = use_signal(|| false);
    // close when click outside
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
                        onclick: move |e| {
                            on_detail.call(e);
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
    let mut popover_id = use_signal(|| String::new());
    rsx! {
        div {
            class: "more-trigger note-more cusror-pointer",
            div {
                "data-popover-id": popover_id(),
                onpointerenter: move |e| {
                    if !popover_id().is_empty() {
                        MODAL_MANAGER.write()
                            .update_popover_position(&popover_id(), e.page_coordinates().to_tuple())
                    }
                },
                onclick: move |e| {
                    e.stop_propagation();
                    let (x, y) = e.page_coordinates().to_tuple();
                    if MODAL_MANAGER.read().has_modal(&popover_id()) {
                        MODAL_MANAGER.write().open_modal(&popover_id());
                    } else {
                        let popover_modal_id = MODAL_MANAGER.write().add_popover(popover.clone(), (x, y));
                        MODAL_MANAGER.write().open_modal(&popover_modal_id);
                        popover_id.set(popover_modal_id);
                    }
                },
                dangerous_inner_html: "{MORE}"
            }
        }
    }
}
