use core::str;
use std::sync::Arc;

use dioxus::prelude::*;
use nostr_sdk::{Client, EventId};

use crate::{
    nostr::{
        fetch::{get_event_by_id, get_replies},
        multiclient::MultiClient,
        note::{ReplyTrees, TextNote},
    }, utils::js::node_add_class, views::note_list::note::Note, CustomSub, Route
};

pub async fn get_event(client: Arc<Client>, e_id: EventId, replytree: &mut Signal<ReplyTrees>) {
    match get_event_by_id(&client, &e_id, None).await {
        Ok(Some(event)) => {
            replytree.write().accept(vec![event.clone()]);

            match get_replies(&client, &e_id, None).await {
                Ok(replies) => {
                    replytree.write().accept(replies.clone());

                    for reply in replies {
                        let client_clone = Arc::clone(&client);
                        let mut replytree_clone = replytree.clone(); // Declare replytree_clone as mutable
                        let reply_id = reply.id.clone();
                        spawn(async move {
                            get_event(client_clone, reply_id, &mut replytree_clone).await;
                        });
                    }
                }
                Err(e) => {
                    tracing::error!("error: {:?}", e);
                }
            }

            let note = TextNote::try_from(event.clone()).unwrap();
            if note.is_reply() {
                if let Some(root_id) = note.root {
                    let client_clone = Arc::clone(&client);
                    let mut replytree_clone = replytree.clone(); // Declare replytree_clone as mutable
                    spawn(async move {
                        get_event(client_clone, root_id, &mut replytree_clone).await;
                    });
                }
            }
        }
        Ok(None) => {
            tracing::error!("event not found");
        }
        Err(e) => {
            tracing::error!("error: {:?}", e);
        }
    }
}

#[component]
pub fn NoteDetail(sub: String, id: String) -> Element {
    let multiclient = use_context::<Signal<MultiClient>>();

    let all_sub = use_context::<Signal<Vec<CustomSub>>>();
    let sub_name = use_signal(|| sub.clone());
    let event_id = use_signal(|| id.clone());
    let mut replytree = use_context::<Signal<ReplyTrees>>();
    let mut element = use_signal(|| rsx! { div { "Loading..." } });
    let _ = use_resource(move || async move {
        let clients = multiclient();
        let sub_current = all_sub.read();
        let sub = sub_current.iter().find(|s| s.name == sub_name()).unwrap();
        let client = clients.get(&sub.relay_set).clone().map(Arc::new);
        let eventId = EventId::from_hex(&event_id()).unwrap();
        replytree.write().clear();
        if let Some(client) = client {
            // get_event(client, event_id, &mut replytree).await;

            match get_event_by_id(&client, &eventId, None).await {
                Ok(Some(event)) => {
                    let mut rt: Write<_, UnsyncStorage> = replytree.write();
                    rt.accept(vec![event]);
                    // match get_replies(&client, &eventId, None).await {
                    //     Ok(replies) => {
                    //         rt.accept(replies.clone());
                    //     }
                    //     Err(e) => {
                    //         tracing::error!("error: {:?}", e);
                    //     }
                    // }
                }
                Ok(None) => {
                    tracing::error!("event not found");
                }
                Err(e) => {
                    tracing::error!("error: {:?}", e);
                }
            }
        } else {
            tracing::error!("client not found");
        }
    });

    use_effect(use_reactive(&replytree, move |rt| {
        element.set(rsx! {
            div {
                class: "note-detail-mode-box",
                div {
                    class: "note-detail-mode-content",
                    div {
                        class: "relative z-1",
                        Layer {
                            sub_name: sub_name.read(),
                            event_id: event_id.read(),
                        }
                    }
                }
                div {
                    class:"width-500",
                }
            }
        })
    }));
    rsx! {
        {element}
    }
}

#[derive(PartialEq, Clone, Props)]
struct LayerProps {
    sub_name: String,
    event_id: String,
    #[props(default = false)]
    show_pt_size: bool,
}

#[component]
fn Layer(props: LayerProps) -> Element {
    let e_id = EventId::from_hex(&props.event_id.clone()).unwrap();
    let route_path = use_route::<Route>();
    let replytree = use_context::<Signal<ReplyTrees>>();
    let replytree_lock = replytree.read();
    if replytree_lock.is_empty() {
        return rsx! { div { "Loading..." } };
    }
    let event = replytree_lock.get_note_by_id(&e_id).unwrap();
    let replies = replytree_lock.get_replies(&e_id, None);
    let mut show = use_signal(|| true);
    let repliesLen = replies.len();

    let path_note_id = match route_path {
        Route::NoteDetail { sub, id } => {
            // spawn(async move {
            //     // note-750eccfc04cf63ebe002d63c538a315eac179258f9c5bde417d61be11ca9b261
            //     node_add_class(&format!("note-{}", id), "note-active").await; 
            // });
            id
        }
        _ => "".to_string(),
        
    };
    rsx! {
        Note {
            on_expand: move |_| {
                show.set(!show());
            },
            sub_name: props.sub_name.clone(),
            event: event.inner.clone(),
            is_expand: repliesLen > 0,
            is_tree: true,
            clsname: format!("relative {} z-{} {} mb-12", if path_note_id.eq(&props.event_id) {
                "com-post--active"
            } else {
                ""
            }, if repliesLen > 0 {repliesLen} else {0}, if props.show_pt_size {"pt-16"} else {""})
        }
        if show() && repliesLen > 0 {
            div {
                class: format!("relative z-{}", if repliesLen > 0 {repliesLen-1} else {0}),
                style: format!("margin-top: -28px;"),
                // style: format!(""),
                for (i, reply) in replies.iter().enumerate() {
                    // Note {
                    //     sub_name: sub_name.clone(),
                    //     event: reply.inner.clone(),
                    //     is_expand: false,
                    //     is_tree: true,
                    // }
                    Layer {
                        sub_name: props.sub_name.clone(),
                        event_id: reply.inner.id.clone(),
                        show_pt_size: i == 0,
                    }
                }
            }
        }
    }
}
