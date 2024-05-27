use dioxus::prelude::*;
use nostr_indexeddb::database::Order;
use nostr_sdk::{Event, Filter, JsonUtil, Kind, Metadata, PublicKey};

use crate::{
    nostr::{
        fetch::{get_metadata, get_newest_event},
        multiclient::MultiClient,
    },
    utils::format::format_create_at,
};

#[derive(PartialEq, Clone, Props)]
pub struct MentionProps {
    pubkey: PublicKey,
    relay_name: String,
}

#[component]
pub fn Mention(props: MentionProps) -> Element {
    let multiclient = use_context::<Signal<MultiClient>>();
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
            let multiclient = multiclient();
            if let Some(client) = multiclient.get_client(&relay_name) {
                let client = client.client();
                // client.send_event_builder(builder)
                spawn(async move {
                    let filter = Filter::new().author(pubkey).kind(Kind::Metadata);
                    let event_result = client
                        .database()
                        .query(vec![filter], Order::Desc)
                        .await
                        .unwrap();
                    if let Some(event) = get_newest_event(&event_result) {
                        let metadata = Metadata::from_json(&event.content).unwrap();
                        ele.set(rsx! {
                            a {
                                href: "javascript:void(0);",
                                {format!("@{}", metadata.display_name.unwrap_or_else(|| {
                                    metadata.name.unwrap_or("Nostr Account".to_string())
                                }))}
                            }
                        });
                        
                    }
                    match get_metadata(&client, &pubkey, None).await {
                        Ok(metadata) => {
                            ele.set(rsx! {
                                a {
                                    href: "javascript:void(0);",
                                    {format!("@{}", metadata.display_name.unwrap_or_else(|| {
                                        metadata.name.unwrap_or("Nostr Account".to_string())
                                    }))}
                                }
                            });
                        }
                        Err(e) => {
                            tracing::error!("get_metadata error: {:?}", e);
                        }
                    }
                });
            }
        },
    ));

    rsx! {
        {ele}
    }
}
