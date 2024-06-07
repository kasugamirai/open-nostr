use dioxus::prelude::*;
use nostr_indexeddb::database::Order;
use nostr_sdk::{Filter, JsonUtil, Kind, Metadata, PublicKey};

use crate::nostr::{
    fetch::get_metadata,
    multiclient::{EventCache, MultiClient},
    utils::get_newest_event,
};

#[derive(PartialEq, Clone, Props)]
pub struct MentionProps {
    pubkey: PublicKey,
    relay_name: String,
}

#[component]
pub fn Mention(props: MentionProps) -> Element {
    let multiclient = use_context::<Signal<MultiClient>>();
    let event_cache = use_context::<Signal<EventCache>>();
    let mut ele = use_signal(|| {
        rsx! {
            a {
                href: "javascript:void(0);",
                "@user loading..."
            }
        }
    });
    // Fetching metadata for the main avatar
    use_effect(use_reactive(
        (&props.pubkey, &props.relay_name),
        move |(pubkey, relay_name)| {
            spawn({
                let multiclient = multiclient.clone();
                let event_cache = event_cache.clone();
                async move {
                    let hc_client = {
                        let multiclient = multiclient.read();
                        if let Some(client) = multiclient.get_client(&relay_name).await {
                            client
                        } else {
                            tracing::error!("client not found");
                            return;
                        }
                    };

                    let events = event_cache
                        .read()
                        .cached_get_events_of(
                            &hc_client,
                            vec![Filter::new().author(pubkey).kind(Kind::Metadata)],
                            None,
                        )
                        .await;
                    match events {
                        Ok(events) => {
                            if let Some(event) = get_newest_event(&events) {
                                if let Ok(metadata) = Metadata::from_json(&event.content) {
                                    ele.set(rsx! {
                                        a {
                                            href: "javascript:void(0);",
                                            {format!("@{}", metadata.display_name.unwrap_or_else(|| {
                                                metadata.name.unwrap_or("Nostr Account".to_string())
                                            }))}
                                        }
                                    });
                                }
                            }
                        }
                        Err(e) => {
                            tracing::error!("get_metadata error: {:?}", e);
                        }
                    }
                }
            });
        },
    ));

    rsx! {
        {ele}
    }
}
