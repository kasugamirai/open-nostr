use std::time::Duration;

use dioxus::prelude::*;
use nostr_sdk::{Client, EventId, Filter};

use crate::{
    components::icons::*,
    utils::format::{format_content, format_create_at, format_public_key},
    Route,
};

#[derive(PartialEq, Clone)]
pub struct NoteData {
    pub id: String,
    pub author: String,
    pub created_at: u64,
    pub kind: String,
    pub tags: Vec<String>,
    pub content: String,
    pub index: usize,
    pub event: nostr_sdk::Event,
}

#[derive(PartialEq, Clone, Props)]
pub struct NoteProps {
    pub on_detail: EventHandler<()>,
    pub data: NoteData,
    //pub metadata: nostr_sdk::Metadata,
}

#[component]
pub fn Note(props: NoteProps) -> Element {
    let client = use_context::<Signal<Client>>();

    let author = props.data.author.clone();
    let e = EventId::from_hex(author).unwrap();

    let future = use_resource(move || async move {
        // TODO: get metadata
        let filter = Filter::new().event(e);
        match client
            .read()
            .get_events_of(vec![filter], Some(Duration::from_secs(30)))
            .await
        {
            Ok(events) => {
                if events.is_empty() {
                    Some("https://is.gd/hidYxs")
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    });

    rsx! {
        div {
            class: "com-post",
            div {
                class: "com-post-author",
                div {
                    class: "com-post-author-avatar",
                    img {
                        src: match &*future.read_unchecked() {
                            Some(Some(s)) => s,
                            Some(None) => "https://is.gd/hidYxs",
                            None => "https://is.gd/hidYxs",
                        }
                    }
                }
                div {
                    class: "com-post-author-profile",
                    span {
                        class: "com-post-author-profile-name",
                        {format_public_key(&props.data.author, None)}
                    }
                    span {
                        class: "com-post-author-profile-created",
                        {format_create_at(props.data.created_at)}
                    }
                }
                div {
                    style: "flex: 1;",
                }
                div {
                    class: "com-post-author-more",
                    MoreInfo {
                        on_detail: move |_| {
                            props.on_detail.call(());
                        },
                    }
                }
            }
            div {
                class: "com-post-content",
                dangerous_inner_html: "{format_content(&props.data.content)}",
            }
            div {
                class: "com-post-info",
                div {
                    class: "com-post-info-item com-post-info-reply",
                    span {
                        dangerous_inner_html: "{TURN_LEFT}",
                    }
                    span {
                        class: "com-post-info-item-data",
                        "5"
                    }
                }
                div {
                    class: "com-post-info-item com-post-info-share",
                    span {
                        dangerous_inner_html: "{TURN_RIGHT}",
                    }
                    span {
                        class: "com-post-info-item-data",
                        "2"
                    }
                }
                div {
                    class: "com-post-info-item com-post-info-comment",
                    span {
                        dangerous_inner_html: "{MARKS}",
                    }
                    span {
                        class: "com-post-info-item-data",
                        "2"
                    }
                }
                div {
                    class: "com-post-info-item com-post-info-reward",
                    span {
                        dangerous_inner_html: "{FLASH}",
                    }
                    span {
                        class: "com-post-info-item-data",
                        "40k"
                    }
                }
                Link {
                    class: "com-post-info-item com-post-info-reply",
                    to: Route::Topic { id: props.data.id.clone() },
                    span {
                        dangerous_inner_html: "{ADD}",
                    }
                }
            }
        }
    }
}

#[component]
pub fn MoreInfo(on_detail: EventHandler<()>) -> Element {
    let mut edit = use_signal(|| false);
    rsx! {
        div {
            style: "position: relative;",
            div {
                class: "more-trigger",
                div {
                    onclick: move |_| {
                        edit.set(!edit());
                    },
                    dangerous_inner_html: "{MORE}"
                }
            }
            div {
                class: "show-{edit}",
                style: "position: absolute; right: 0; background-color: var(--bgc-0); border-radius: var(--radius-1); display: flex; flex-direction: column; gap: 10px; padding: 10px; 20px; border: 1px solid var(--boc-1); z-index: 100;",
                div {
                    style: "display: flex; flex-direction: column; gap: 10px; padding: 10px; 20px; width: 140px;",
                    div {
                        style: "display: flex; align-items: center; gap: 5px; cursor: pointer;",
                        onclick: move |_| {
                            edit.set(false);
                        },
                        div {
                            dangerous_inner_html: "{SHARE}"
                        }
                        "Share"
                    }
                    div {
                        style: "display: flex; align-items: center; gap: 5px; cursor: pointer;",
                        onclick: move |_| {
                            edit.set(false);
                        },
                        div {
                            dangerous_inner_html: "{STAR}"
                        }
                        "Book Mark"
                    }
                    div {
                        style: "display: flex; align-items: center; gap: 5px; cursor: pointer;",
                        onclick: move |_| {
                            edit.set(false);
                        },
                        div {
                            dangerous_inner_html: "{STATION}"
                        }
                        "Broadcast"
                    }
                    div {
                        style: "display: flex; align-items: center; gap: 5px; cursor: pointer;",
                        onclick: move |_| {
                            on_detail.call(());
                            edit.set(false);
                        },
                        div {
                            dangerous_inner_html: "{INFO}"
                        }
                        "Details"
                    }
                }
            }
        }
    }
}
