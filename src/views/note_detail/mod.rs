use std::time::Duration;

use dioxus::{events, prelude::*};
use nostr_sdk::{Alphabet, Event, EventId, Filter, JsonUtil, Kind, SingleLetterTag};
use serde::Serialize;

use crate::{
    nostr::{
        fetch::{get_event_by_id, get_replies},
        multiclient::MultiClient,
        note::{DisplayOrder, ReplyTrees, TextNote},
    },
    views::note_list::note::{Note, NoteData},
};

#[derive(Debug, Clone, Serialize)]
struct NoteTree {
    content: String,
    children: Vec<NoteTree>,
    event: Event,
}

fn get_notetree(event: &Event) -> Vec<NoteTree> {
    vec![NoteTree {
        content: event.content.to_string(),
        children: vec![],
        event: event.clone(),
    }]
}

impl PartialEq for NoteTree {
    fn eq(&self, other: &Self) -> bool {
        self.content == other.content
    }
}

#[component]
pub fn NoteDetail(sub: String, id: String) -> Element {
    let multiclient = use_context::<Signal<MultiClient>>();

    let sub_name = use_signal(|| sub.clone());
    let event_id = use_signal(|| id.clone());

    let mut count = use_signal(|| 0);

    let mut replytree = use_signal(|| ReplyTrees::default());
    let on_mounted = move |_| {
        spawn(async move {
            let clients = multiclient();
            tracing::info!("clients: {:?}", clients);
            let client = clients.get(&sub_name.read()).unwrap();

            match get_event_by_id(&client, &EventId::from_hex(&event_id()).unwrap(), None).await {
                Ok(Some(event)) => {
                    replytree.write().accept(vec![event]);
                    match get_replies(&client, EventId::from_hex(&event_id()).unwrap(), None).await
                    {
                        Ok(replies) => {
                            replytree.write().accept(replies);
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
        });
    };

    rsx! {
        div {
            style: "width: 100%; height: 100%; display: flex; gap: 20px;",
            div {
                style: "flex: 1; height: 100%; overflow-y: scroll;",
                Layer {
                    sub_name: sub,
                    event_id: id,
                }
            }
            div {
                style: "width: 300px;",
                button {
                    onclick: on_mounted,
                    "Get Event"
                }
            }
        }
    }
}

#[component]
fn Layer(sub_name: String, event_id: String) -> Element {
    let multiclient = use_context::<Signal<MultiClient>>();
    let mut element = use_signal(|| rsx! { div {} });
    let sub_name = use_signal(|| sub_name.clone());
    let event_id = use_signal(|| event_id.clone());
    let _ = use_resource(move || async move {
        let clients = multiclient();
        let client = clients.get(&sub_name()).unwrap();
        match get_event_by_id(&client, &EventId::from_hex(&event_id()).unwrap(), None).await {
            Ok(Some(event)) => {
                match get_replies(&client, EventId::from_hex(&event_id()).unwrap(), None).await {
                    Ok(replies) => {
                        element.set(rsx! {
                            div {
                                "123"
                            }
                        });
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
    });

    rsx! {
        {element}
    }
}
