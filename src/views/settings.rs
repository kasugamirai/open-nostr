use std::{str::FromStr, time::Duration, vec};

use dioxus::prelude::*;
use nostr_sdk::prelude::*;

use crate::{
    utils::format::{format_create_at, format_public_key, splite_by_replys},
};
// {
//     "id": "eb8142a456387a0f593273b808290b29765a1958700f94bcc6c1ff0cef7fa4b0",
//     "pubkey": "fcab5a7bee61b9d16f36ef9c5801227cdc500c746b9ab501e808685e0eddb9f7",
//     "content": "引用一下：\nnostr:note1fvyyese28wfwyznjcaacp3e62wnn4ufng4vk8ftr3k0hxyafq3rshmm6fk\n\nyes",
//     "kind": 1,
//     "created_at": 1715628472,
//     "tags": [],
//     "sig": "73997bd7d63555721a05f56ff979900b3b3f2760e81e983ea7d32298242e5ebd053b8417594c61fdc1ec05b53eb3a2b50b36e5ae9754ec6bba764f94019a82c2",
//     "relays": []
//   }
#[component]
pub fn Settings() -> Element {
    let mut data = use_signal(vec::Vec::new);
    let get_events = move || {
        let n: &str = r#"
        qwetgsss http://1/2.jpg #Dog
        123 nostr:adsg4ea34hasedrf #Car
        geoamkhhh
        "#;
        let res = splite_by_replys(n);
        tracing::info!("res======>: {:?}", res);
        spawn(async move {
            let client = Client::default();
            // wss://relayable.org,wss://btc.klendazu.com
            // client.add_relay("wss://freerelay.xyz/").await.unwrap();
            // client.add_relay("wss://nos.lol/").await.unwrap();
            // client.add_relay("wss://nostr.wine/").await.unwrap();
            // client.add_relay("wss://soloco.nl/").await.unwrap();
            client.add_relay("wss://relay.damus.io/").await.unwrap();
            // client.add_relay("wss://relay.snort.social/").await.unwrap();

            client.connect().await;

            let mut filter: Filter = Filter::new();
            let public_key = PublicKey::from_str(
                "fcab5a7bee61b9d16f36ef9c5801227cdc500c746b9ab501e808685e0eddb9f7",
            )
            .unwrap();
            filter = filter.kind(Kind::TextNote).limit(100).author(public_key);

            let events = client
                .get_events_of(vec![filter], Some(Duration::from_secs(60)))
                .await
                .unwrap();
            tracing::info!("events: {:?}", events);
            data.set(events);

            let _ = client.disconnect().await;
        });
    };

    rsx! {
        div {
            onmounted: move |_| {
                get_events()
            },
            style: "height: 100%; display: flex; flex-direction: column; gap: 10px; overflow-y: auto;",
            for event in data.iter() {
                EventItem { event: event.clone() }
            }
        }
    }
}

#[component]
pub fn EventItem(event: nostr_sdk::Event) -> Element {
    rsx! {
        // Note {
        //     sub_name: "".to_string(),
        //     data: NoteData::from(&event.clone(), 1),
        // }
        div {
            class: "event-note",
            div {
                class: "header",
                div {
                    class: "user",
                    if  event.kind == Kind::Repost {
                      div {
                        class: "avatar avatarLeft",
                          img {
                              class: "image",
                              src: "https://avatars.githubusercontent.com/u/1024025?v=4"
                          }
                      }
                    } else {
                      div {
                        class: "avatar avatarLeft",
                          img {
                              class: "image",
                              src: "https://avatars.githubusercontent.com/u/1024025?v=4"
                          }
                      }
                      div{
                        class:"placeholder",
                      }
                      div {
                        class: "avatar avatarRight",
                          img {
                              class: "image",
                              src: "https://avatars.githubusercontent.com/u/1024025?v=4"
                          }
                      }
                    }
                    div {
                        class: "profile",
                        span {
                            class: "nickname",
                            {format_public_key(&event.author().to_hex(), None)}
                        }
                        span {
                            class: "created",
                            {format_create_at(event.created_at().as_u64())}
                        }
                    }
                }
                div {
                    class: "action,"
                }
            }
            div {
                class: "reply",
                div {
                    class: "title",
                    "Re:"
                }
                div {
                    class: "note",
                    EventLess { event: event.clone(), content: "".to_string() }
                }
            }
            for i in splite_by_replys(&event.content.to_string()) {
                if i.starts_with("nostr:") {
                    div {
                        class: "quote",
                        div {
                            class: "title",
                            "Qt:"
                        }
                        div {
                            class: "note",
                            EventLess { event: event.clone(), content: i }
                        }
                    }
                } else {
                    div {
                        class: "content",
                        dangerous_inner_html: "{i}"
                    }
                }
            }
        }
    }
}

#[component]
fn EventLess(event: nostr_sdk::Event, content: String) -> Element {
    rsx! {
        div {
            class: "event-less",
            // Avatar { pubkey: event.author(),  timestamp: event.created_at().as_u64() }
            div {
                class: "text",
                dangerous_inner_html: "{event.content.to_string()}",
            }
        }
    }
}
