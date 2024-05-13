use std::time::Duration;

use dioxus::prelude::*;
use nostr_sdk::prelude::*;

use crate::{
    components::icons::*,
    utils::format::{format_content, format_create_at, format_public_key}, Route,
};

enum NoteAction {
    Replay,
    Share,
    Qoute,
    Zap,
}

struct NoteActionState {
    action: NoteAction,
    count: u64,
}

#[component]
pub fn Settings() -> Element {
    let mut data = use_signal(|| vec![]);
    let get_events = move || {
        spawn(async move {
            let client = Client::default();

            client.add_relay("wss://btc.klendazu.com").await.unwrap();

            client.connect().await;

            let mut filter: Filter = Filter::new();
            filter = filter.kind(Kind::TextNote).limit(5).hashtag("Dog");

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
    let note_action_state = vec![
        NoteActionState {
            action: NoteAction::Replay,
            count: 100,
        },
        NoteActionState {
            action: NoteAction::Share,
            count: 10,
        },
        NoteActionState {
            action: NoteAction::Qoute,
            count: 10,
        },
        NoteActionState {
            action: NoteAction::Zap,
            count: 20,
        },
    ];

    rsx! {
        div {
            class: "event-note",
            div {
                class: "header",
                div {
                    class: "user",
                    div {
                        class: "avatar",
                        img {
                            class: "image",
                            src: "https://is.gd/hidYxs"
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
                    EventLess { event: event.clone() }
                }
            }
            div {
                class: "content",
                dangerous_inner_html: "{format_content(&event.content.to_string())}",
            }
            div {
                class: "quote",
                div {
                    class: "title",
                    "Qt:"
                }
                div {
                    class: "note",
                    EventLess { event: event.clone() }
                }
            }
            div {
                class: "footer",
                {note_action_state.iter().map(|_state| {
                    rsx! {
                        div {
                            class: "info",
                            span {
                                class: "note-action-icon",
                                dangerous_inner_html: match _state.action {
                                    NoteAction::Replay => TURN_LEFT.to_string(),
                                    NoteAction::Share => TURN_RIGHT.to_string(),
                                    NoteAction::Qoute => QUTE.to_string(),
                                    NoteAction::Zap => ZAP.to_string(),
                                }
                            }
                            span {
                                class: "note-action-count",
                                {format!("{}", _state.count)}
                            }
                        }
                    }
                })}
                span{
                    style: "height: 24px; width: 3px; background-color: var(--txt-3); margin-left: 10px;",
                }
                Link {
                    class: "info",
                    to: Route::NoteDetail { id: event.id().to_hex() },
                    span {
                        dangerous_inner_html: "{ADD}",
                    }
                }
            }
        }
    }
}

#[component]
fn EventLess(event: nostr_sdk::Event) -> Element {
    rsx! {
        div {
            class: "event-less",
            div {
                class: "header",
                div {
                    class: "avatar",
                    img {
                        class: "image",
                        src: "https://is.gd/hidYxs"
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
                class: "text",
                dangerous_inner_html: "{event.content.to_string()}",
            }
        }
    }
}
