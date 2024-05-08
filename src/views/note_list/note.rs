use std::time::Duration;

use dioxus::prelude::*;
use nostr_sdk::{Client, EventId, Filter, JsonUtil};

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

impl NoteData {
    pub fn from(event: &nostr_sdk::Event, index: usize) -> Self {
        NoteData {
            id: event.id().to_hex(),
            author: event.author().to_hex(),
            created_at: event.created_at().as_u64(),
            kind: "".to_string(),
            tags: vec![],
            content: event.content.to_string(),
            index,
            event: event.clone(),
        }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct NoteProps {
    pub data: NoteData,
    //pub metadata: nostr_sdk::Metadata,
}

#[component]
pub fn Note(props: NoteProps) -> Element {
    let author = props.data.author.clone();
    let e = EventId::from_hex(author).unwrap();

    let future = use_resource(move || async move {
        let client = Client::default();
        // TODO: get metadata
        let filter = Filter::new().event(e);
        match client
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

    let mut show_detail = use_signal(|| false);
    let mut detail = use_signal(|| String::new());

    let highlight = move || {
        let eval: UseEval = eval(
            r#"
                let _ = await dioxus.recv();
                hljs.highlightAll();
            "#,
        );
        eval.send("".into()).unwrap();
    };

    rsx! {
        div {
            class: "com-post",
            div {
                style: format!("position: fixed; top: 0; left: 0; right: 0; bottom: 0; background-color: rgba(0, 0, 0, 0.5); z-index: 99999999; display: {};", if *show_detail.read() { "block" } else { "none" }),
                div {
                    style: "width: 50%; height: 60%; max-width: 900px; background-color: #fff; position: fixed; top: 50%; left: 50%; transform: translate(-50%, -50%); padding: 20px; border-radius: 10px;",
                    button {
                        class: "btn-icon remove",
                        style: "position: absolute; top: -12px; left: -12px;",
                        onclick: move |_| {
                            show_detail.set(false);
                        },
                        dangerous_inner_html: "{FALSE}"
                    }
                    pre {
                        style: "height: 100%; overflow-y: auto; font-size: 16px;",
                        code {
                            class: "language-json",
                            "{detail}"
                        }
                    }
                }
            }
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
                            let json_value: serde_json::Value = serde_json::from_str(&props.data.event.as_json()).unwrap();
                            let formatted_json = serde_json::to_string_pretty(&json_value).unwrap();
                            detail.set(formatted_json);
                            highlight();
                            show_detail.set(!show_detail());
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
                    to: Route::NoteDetail { id: props.data.id.clone() },
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

    // close when click outside
    let root_click_pos = use_context::<Signal<(f64, f64)>>();
    let mut pos: Signal<(f64, f64)> = use_signal(|| root_click_pos());
    use_effect(use_reactive((&pos,), move |(pos,)| {
        // The coordinates of root element
        let root_pos = root_click_pos();

        // The coordinates of current element
        let current_pos = pos();

        // Determine if two coordinates are the same
        if current_pos.0 != root_pos.0 || current_pos.1 != root_pos.1 {
            edit.set(false);
        }
    }));

    rsx! {
        div {
            onclick: move |event| {
                // Save the coordinates of the event relative to the screen
                pos.set(event.screen_coordinates().to_tuple());
            },
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
