use dioxus::prelude::*;
use nostr_sdk::EventId;

use crate::{
    nostr::{
        fetch::{get_event_by_id, get_replies},
        multiclient::MultiClient,
        note::ReplyTrees,
    },
    views::note_list::note::{Note, NoteData},
};

#[component]
pub fn NoteDetail(sub: String, id: String) -> Element {
    let multiclient = use_context::<Signal<MultiClient>>();

    let sub_name = use_signal(|| sub.clone());
    let event_id = use_signal(|| id.clone());

    let mut replytree = use_signal(|| ReplyTrees::default());
    let mut element = use_signal(|| rsx! { div { "Loading..." } });
    let _ = use_resource(move || async move {
        let clients = multiclient();
        let client = clients.get(&sub_name.read()).unwrap();

        match get_event_by_id(&client, &EventId::from_hex(&event_id()).unwrap(), None).await {
            Ok(Some(event)) => {
                let mut rt: Write<_, UnsyncStorage> = replytree.write();
                rt.accept(vec![event]);
                match get_replies(&client, &EventId::from_hex(&event_id()).unwrap(), None).await {
                    Ok(replies) => {
                        rt.accept(replies.clone());
                    }
                    Err(e) => {
                        tracing::error!("error: {:?}", e);
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
        element.set(rsx! {
            div {
                style: "width: 100%; height: 100%; display: flex; gap: 20px;",
                div {
                    style: "flex: 1; height: 100%; overflow-y: scroll;",
                    Layer {
                        replytree: replytree,
                        sub_name: sub_name.read(),
                        event_id: event_id.read(),
                    }
                }
                div {
                    style: "width: 500px;",
                }
            }
        })
    });

    rsx! {
        {element}
    }
}

#[component]
fn Layer(replytree: Signal<ReplyTrees>, sub_name: String, event_id: String) -> Element {
    let e_id = EventId::from_hex(&event_id.clone()).unwrap();
    let rt = replytree.read();
    tracing::info!("replytree: {:?}", rt);
    let event = rt.get_note_by_id(&e_id).unwrap();
    let replies = rt.get_replies(&e_id, None);
    tracing::info!("event: {:?}", event);
    tracing::info!("replies: {:?}", replies);
    let mut show = use_signal(|| false);
    let repliesLen = replies.len();
    rsx! {
        Note {
            on_expand: move |_| show.set(!show()),
            sub_name: sub_name.clone(),
            data: NoteData::from(&event.inner.clone(), 0),
            is_expand: repliesLen > 0,
        }
        if show() {
            div {
                style: "margin-left: 20px;",
                for reply in replies {
                    Note {
                        sub_name: sub_name.clone(),
                        data: NoteData::from(&reply.inner.clone(), 0),
                        is_expand: false,
                    }
                }
            }
        }
    }
}
