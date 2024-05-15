use dioxus::prelude::*;
use nostr_sdk::EventId;

use crate::nostr::{
    fetch::{get_event_by_id, get_replies},
    multiclient::MultiClient,
    note::ReplyTrees,
};

#[component]
pub fn NoteDetail(sub: String, id: String) -> Element {
    let multiclient = use_context::<Signal<MultiClient>>();

    let sub_name = use_signal(|| sub.clone());
    let event_id = use_signal(|| id.clone());

    let mut replytree = use_signal(|| ReplyTrees::default());
    let _ = use_resource(move || async move {
        spawn(async move {
            let clients = multiclient();
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
    });

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
                style: "width: 500px;",
            }
        }
    }
}

#[component]
fn Layer(sub_name: String, event_id: String) -> Element {
    let multiclient = use_context::<Signal<MultiClient>>();
    let mut element = use_signal(|| rsx! { div { "Loading..." } });
    let sub_name = use_signal(|| sub_name.clone());
    let event_id = use_signal(|| event_id.clone());
    let _ = use_resource(move || async move {
        let clients = multiclient();
        let client = clients.get(&sub_name()).unwrap();
        match get_event_by_id(&client, &EventId::from_hex(&event_id()).unwrap(), None).await {
            Ok(Some(event)) => {
                let replies =
                    match get_replies(&client, EventId::from_hex(&event_id()).unwrap(), None).await
                    {
                        Ok(replies) => replies,
                        Err(e) => {
                            tracing::error!("error: {:?}", e);
                            vec![]
                        }
                    };
                element.set(rsx! {
                    div {
                        "{event.content}"
                    }
                    div {
                        for e in replies {
                            Layer {
                                sub_name: sub_name.read().clone(),
                                event_id: e.id.to_string(),
                            }
                        }
                    }
                });
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
