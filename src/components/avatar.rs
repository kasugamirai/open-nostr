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
    let mut nickname = use_signal(|| "Nostr Account".to_string());
    let mut avatar =
        use_signal(|| "https://avatars.githubusercontent.com/u/1024025?v=4".to_string());
    let mut root_pic =
        use_signal(|| "https://avatars.githubusercontent.com/u/1024025?v=4".to_string());
    let mut root_nickname = use_signal(|| "Nostr Account".to_string());

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
                        nickname.set(metadata.display_name.unwrap_or_else(|| {
                            metadata.name.unwrap_or("Nostr Account".to_string())
                        }));
                        avatar.set(metadata.picture.unwrap_or_else(|| {
                            "https://avatars.githubusercontent.com/u/1024025?v=4".to_string()
                        }));
                    }
                    match get_metadata(&client, &pubkey, None).await {
                        Ok(metadata) => {
                            nickname.set(metadata.display_name.unwrap_or_else(|| {
                                metadata.name.unwrap_or("Nostr Account".to_string())
                            }));
                            avatar.set(metadata.picture.unwrap_or_else(|| {
                                "https://avatars.githubusercontent.com/u/1024025?v=4".to_string()
                            }));
                        }
                        Err(e) => {
                            tracing::error!("get_metadata error: {:?}", e);
                        }
                    }
                });
            }
        },
    ));

    // Fetching metadata for the repost event, if any
    use_effect(use_reactive(
        (&props.repost_event, &props.relay_name),
        move |(repost_event, relay_name)| {
            let multiclient = multiclient();
            spawn(async move {
                if let Some(event) = repost_event {
                    if let Some(client) = multiclient.get_client(&relay_name) {
                        let filter = Filter::new().author(event.pubkey).kind(Kind::Metadata);
                        let event_result = client.client()
                            .database()
                            .query(vec![filter], Order::Desc)
                            .await
                            .unwrap();
                        if let Some(event) = get_newest_event(&event_result) {
                            let metadata = Metadata::from_json(&event.content).unwrap();
                            root_pic.set(metadata.picture.unwrap_or_else(|| {
                                "https://avatars.githubusercontent.com/u/1024025?v=4"
                                    .to_string()
                            }));
                            root_nickname.set(metadata.display_name.or(metadata.name).unwrap());
                        }
                        match get_metadata(&client.client(), &event.pubkey, None).await {
                            Ok(metadata) => {
                                root_pic.set(metadata.picture.unwrap_or_else(|| {
                                    "https://avatars.githubusercontent.com/u/1024025?v=4"
                                        .to_string()
                                }));
                                root_nickname.set(metadata.display_name.or(metadata.name).unwrap());
                            }
                            Err(e) => {
                                tracing::error!("get_metadata error: {:?}", e);
                            }
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
                class: "post-avatar flex items-center",
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
                    class: "profile flex flex-col",
                    span {
                        class: "nickname font-size-16 txt-1",
                        {root_nickname}
                    }
                    span {
                        class: "created txt-3 font-size-12",
                        "{format_create_at(repost_event.created_at().as_u64())}"
                    }
                }
            }
        }
    } else {
        rsx! {
            div {
                class: "post-avatar flex items-center",
                img {
                    class: "square-40 radius-20 mr-12",
                    src: "{avatar}",
                    alt: "avatar",
                }
                div {
                    class: "profile flex flex-col",
                    span {
                        class: "nickname font-size-16 txt-1",
                        "{nickname}"
                    }
                    span {
                        class: "created txt-3 font-size-12",
                        "{format_create_at(props.timestamp)}"
                    }
                }
            }
        }
    }
}
