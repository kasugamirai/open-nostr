use dioxus::prelude::*;
use nostr_indexeddb::database::Order;
use nostr_sdk::{Event, Filter, JsonUtil, Kind, Metadata, PublicKey};

use crate::{
    nostr::{
        multiclient::{EventCache, MultiClient},
        utils::get_newest_event,
    },
    utils::format::format_create_at,
};

#[derive(PartialEq, Clone, Props)]
pub struct AvatarProps {
    pubkey: PublicKey,
    timestamp: u64,
    relay_name: String,
    repost_event: Option<Event>,
}

#[component]
pub fn Avatar(props: AvatarProps) -> Element {
    let multiclient = use_context::<Signal<MultiClient>>();

    // Using signals for reactive state management
    let nickname = use_signal(|| "Nostr Account".to_string());
    let avatar = use_signal(|| "https://avatars.githubusercontent.com/u/1024025?v=4".to_string());
    let root_pic = use_signal(|| "https://avatars.githubusercontent.com/u/1024025?v=4".to_string());
    let root_nickname = use_signal(|| "Nostr Account".to_string());
    let event_cache = use_context::<Signal<EventCache>>();

    use_effect(use_reactive(
        (&props.pubkey, &props.relay_name),
        move |(pubkey, relay_name)| {
            spawn({
                let multiclient = multiclient;
                let event_cache = event_cache;
                let mut nickname = nickname;
                let mut avatar = avatar;
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
                                    nickname.set(metadata.display_name.unwrap_or_else(|| {
                                        metadata.name.unwrap_or("Nostr Account".to_string())
                                    }));
                                    avatar.set(metadata.picture.unwrap_or_else(|| {
                                        "https://avatars.githubusercontent.com/u/1024025?v=4"
                                            .to_string()
                                    }));
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

    // Rendering based on whether there's a repost event
    if let Some(repost_event) = &props.repost_event {
        rsx! {
            div {
                class: "post-avatar flex items-center min-width-120",
                img {
                    class: "square-40 radius-20 mr-12 relative z-1",
                    style: "margin-right: -12px;",
                    src: "{avatar}",
                    alt: "avatar",
                }
                img {
                    class: "square-40 radius-20 mr-12",
                    src: "{root_pic}",
                    alt: "avatar",
                }
                div {
                    class: "profile flex flex-col max-width-80",
                    span {
                        class: "nickname font-size-16 txt-1 text-overflow",
                        {root_nickname}
                    }
                    span {
                        class: "created txt-3 font-size-12 text-overflow",
                        "{format_create_at(repost_event.created_at().as_u64())}"
                    }
                }
            }
        }
    } else {
        rsx! {
            div {
                class: "post-avatar flex items-center min-width-120",
                img {
                    class: "square-40 radius-20 mr-12",
                    src: "{avatar}",
                    alt: "avatar",
                }
                div {
                    class: "profile flex flex-col max-width-80",
                    span {
                        class: "nickname font-size-16 txt-1 text-overflow",
                        "{nickname}"
                    }
                    span {
                        class: "created txt-3 font-size-12 text-overflow",
                        "{format_create_at(props.timestamp)}"
                    }
                }
            }
        }
    }
}
