use core::str;
use std::sync::Arc;

use dioxus::prelude::*;
use nostr::event;
use nostr_sdk::{Client, Event, EventId};

use crate::{
    nostr::{
        fetch::{get_event_by_id, get_replies, Error},
        multiclient::MultiClient,
        note::{ReplyTreeManager, ReplyTrees, TextNote},
    },
    views::note_list::note::Note,
    CustomSub, Route,
};
#[component]
pub fn NoteDetail(sub: String, id: String) -> Element {
    let multiclient = use_context::<Signal<MultiClient>>();
    let all_sub = use_context::<Signal<Vec<CustomSub>>>();
    let sub_name = use_signal(|| sub.clone());
    let event_id = use_signal(|| id.clone());
    let mut refresh = use_signal(|| false);
    let mut replytree_manager = use_context::<Signal<ReplyTreeManager>>();
    let mut element = use_signal(|| rsx! { div { "Loading..." } });

    let _ = use_resource(move || async move {
        let mut manager = replytree_manager.write();
        if manager
            .get_tree(&EventId::from_hex(&event_id()).unwrap())
            .is_none()
        {
            let sub_name = sub_name.clone();
            let event_id = event_id.clone();
            let refresh = refresh.clone();
            spawn({
                let multiclient = multiclient.clone();
                async move {
                    let clients = multiclient.read();
                    let sub_current = all_sub.read();
                    let sub = sub_current.iter().find(|s| s.name == sub_name()).unwrap();
                    
                    let eventid = EventId::from_hex(&event_id()).unwrap();

                    if let Some(client) = clients.get_client(&sub.relay_set) {
                        let client = client.client();
                        match fetch_event_and_replies(&client, &eventid).await {
                            Ok(event) => {
                                element.set(rsx! {
                                    div {
                                        class: "note-detail-mode-box",
                                        div {
                                            class: "note-detail-mode-content",
                                            div {
                                                class: "relative z-1",
                                                Layer {
                                                    sub_name: sub_name.read().to_string(),
                                                    root_id: event.id(),
                                                    event_id:event.id(),
                                                    reply: None,
                                                }
                                            }
                                        }
                                        div {
                                            class:"width-500",
                                        }
                                    }
                                });
                            }
                            Err(e) => {
                                tracing::error!("error: {:?}", e);
                            }
                        }
                    } else {
                        tracing::error!("client not found");
                    }
                }
            });
        }
    });

    rsx! {
        div {
            class: "note-detail-mode-box",
            div {
                class: "note-detail-mode-content",
                div {
                    class: "relative z-1",
                    Layer {
                        sub_name: sub_name.read().to_string(),
                        root_id: event_id.read().to_string(),
                        event_id: event_id.read().to_string(),
                        reply: None,
                    }
                }
            }
            div {
                class:"width-500",
            }
        }
    }
}

async fn fetch_event_and_replies(client: &Client, event_id: &EventId) -> Result<Event, Error> {
    let mut replytree_manager = use_context::<Signal<ReplyTreeManager>>();
    match get_event_by_id(client, event_id, None).await {
        Ok(Some(event)) => {
            let event_clone = event.clone();
            replytree_manager
                .write()
                .accept_event(event_id.clone(), vec![event]);
            match get_replies(client, event_id, None).await {
                Ok(replies) => {
                    replytree_manager
                        .write()
                        .accept_event(event_id.clone(), replies.clone());
                }
                Err(e) => {
                    tracing::error!("error: {:?}", e);
                }
            }
            Ok(event_clone)
        }
        Ok(None) => Err(Error::NotFound),
        Err(e) => Err(e),
    }
}

#[derive(Clone, Props, PartialEq)]
struct LayerProps {
    sub_name: String,
    root_id: String,
    event_id: String,
    #[props(default = false)]
    show_pt_size: bool,
    reply: Option<TextNote>,
}

#[component]
fn Layer(props: LayerProps) -> Element {
    let root_id = EventId::from_hex(&props.root_id).unwrap();
    let event_id = EventId::from_hex(&props.event_id).unwrap();
    let route_path = use_route::<Route>();
    let replytree_manager = use_context::<Signal<ReplyTreeManager>>();
    let manager = replytree_manager.read();
    let mut show = use_signal(|| true);
    if let Some(replytree) = manager.get_tree(&root_id) {
        let root_event = if let Some(event) = &props.reply {
            event
        } else {
            replytree.get_note_by_id(&event_id).unwrap()
        };

        let replies = replytree.get_replies(&event_id, None);
        let repliesLen = replies.len();
        let path_note_id = match route_path {
            Route::NoteDetail { sub, id } => id,
            _ => "".to_string(),
        };
        rsx! {
            div {
                Note {
                    on_expand: move |_| {
                        show.set(!show());
                    },
                    sub_name: props.sub_name.clone(),
                    event: root_event.inner.clone(),
                    is_expand: replies.len() > 0,
                    is_tree: true,
                    clsname: format!("relative {} z-{} {} mb-12", if path_note_id.eq(&props.event_id) {
                        "com-post--active"
                    } else {
                        ""
                    }, if repliesLen > 0 {repliesLen} else {0}, if props.show_pt_size {"pt-16"} else {""})
                }
                if show() && !replies.is_empty() {
                    div {
                        class: format!("relative z-{}", if repliesLen > 0 {repliesLen-1} else {0}),
                        style: format!("margin-top: -28px;"),
                        for (i, reply) in replies.iter().enumerate() {
                                Layer {
                                    sub_name: props.sub_name.clone(),
                                    root_id: props.root_id.clone(),
                                    event_id: reply.inner.id.clone(),
                                    show_pt_size: i == 0,
                                    reply: Some((*reply).clone()), // Clone the reply variable
                                }
                            }
                    }
                }
            }
        }
    } else {
        rsx! { div { "Loading..." } }
    }
}
