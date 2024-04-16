use std::collections::HashSet;

use dioxus::prelude::*;
use nostr_sdk::{Filter, GenericTagValue, Kind, PublicKey, Timestamp};

use crate::{
    components::icons::*,
    utils::format::{format_public_key, format_timestamp},
};

fn kind_to_string(kind: Kind) -> String {
    match kind {
        Kind::TextNote => "Text Note",
        Kind::Repost => "Repost",
        _ => "Unknown",
    }
    .to_string()
}

fn tags_to_string(tags: HashSet<GenericTagValue>) -> String {
    let mut s = vec![];
    for tag in tags {
        match tag {
            GenericTagValue::PublicKey(pk) => {
                s.push(format_public_key(&pk.to_string(), None));
            }
            GenericTagValue::EventId(eid) => {
                s.push(format_public_key(&eid.to_hex(), None));
            }
            GenericTagValue::String(v) => {
                s.push(v);
            }
        }
    }
    s.join(", ")
}

#[component]
pub fn CustomSub() -> Element {
    let mut subs = use_context::<Signal<Vec<Filter>>>();
    let handle_click = move |_| {
        let mut f = Filter::new();
        f = f.kinds(vec![Kind::TextNote, Kind::Repost]);
        f = f.authors(PublicKey::parse(
            "nsec1dmvtj7uldpeethalp2ttwscy32jx36hr9jslskwdqreh2yk70anqhasx64",
        ));
        f = f.since(Timestamp::now());
        f = f.until(Timestamp::now());
        f = f.limit(500);
        f = f.hashtag("steak".to_string());
        subs.push(f);
    };
    let mut handle_add_kind = move |index: usize| {
        // TODO: Remove item before insert, auto update page
        let mut f = subs.remove(index).clone();
        f = f.kind(Kind::Metadata);
        subs.insert(index, f);
    };
    rsx! {
        div {
            class: "custom-sub",
            div {
                class: "custom-sub-header",
                h2 { "Custom Sub" }
                button {
                    class: "btn",
                    "Import"
                }
                button {
                    class: "btn",
                    "Export"
                }
                button {
                    class: "btn",
                    "Save"
                }
            }
            div {
                class: "custom-sub-content",
                div {
                    class: "custom-sub-name",
                    "Name:"
                    div {
                        class: "card disabled",
                        "#steakstr"
                    }
                }
                div {
                    class: "custom-sub-relays",
                    "Relays:"
                    div {
                        class: "card disabled",
                        "Default"
                    }
                }
                div {
                    class: "custom-sub-filters",
                    "Filters:"
                }
                for (i, sub) in subs.iter().enumerate() {
                    div {
                        class: "custom-sub-item",
                        if let Some(authors) = sub.authors.clone() {
                            div {
                                class: "custom-sub-filter-item",
                                span {
                                    class: "title",
                                    "Authors:"
                                }
                                for author in authors {
                                    div {
                                        class: "card custom-sub-author",
                                        "{format_public_key(&author.to_hex(), Some(6))}"
                                    }
                                }
                                button {
                                    class: "btn-add",
                                    onclick: move |_| handle_add_kind(i) ,
                                    dangerous_inner_html: "{ADD}"
                                }
                            }
                        }
                        if let Some(kinds) = sub.kinds.clone() {
                            div {
                                class: "custom-sub-filter-item",
                                span {
                                    class: "title",
                                    "Kinds:"
                                }
                                for kind in kinds {
                                    div {
                                        class: "card custom-sub-kind",
                                        "{kind_to_string(kind)}"
                                    }
                                }
                                button {
                                    class: "btn-add",
                                    onclick: move |_| handle_add_kind(i) ,
                                    dangerous_inner_html: "{ADD}"
                                }
                            }
                        }
                        div {
                            class: "custom-sub-filter-item",
                            span {
                                class: "title",
                                "Time:"
                            }
                            if let Some(since) = sub.since.clone() {
                                div {
                                    class: "card custom-sub-time",
                                    "{format_timestamp(since.as_u64(), None)}"
                                    span {
                                        dangerous_inner_html: "{RIGHT}"
                                    }
                                }
                            }
                            if let Some(until) = sub.until.clone() {
                                div {
                                    class: "card custom-sub-time",
                                    span {
                                        dangerous_inner_html: "{LEFT}"
                                    }
                                    "{format_timestamp(until.as_u64(), None)}"
                                }
                            }
                        }
                        div {
                            class: "custom-sub-filter-item",
                            span {
                                class: "title",
                                "Limits:"
                            }
                            if let Some(limit) = sub.limit.clone() {
                                div {
                                    class: "card custom-sub-limits",
                                    "{limit}"
                                }
                            }
                        }
                        div {
                            class: "custom-sub-filter-item",
                            span {
                                class: "title",
                                "Tags:"
                            }
                            if !sub.generic_tags.is_empty() {
                                for tag in sub.generic_tags.clone() {
                                    div {
                                        class: "card custom-sub-tag",
                                        "#{tag.0.as_char()} | {tags_to_string(tag.1)}"
                                    }
                                }
                            }
                        }
                    }
                }
                div {
                    class: "custom-sub-add",
                    button {
                        class: "btn-add",
                        onclick: handle_click,
                        dangerous_inner_html: "{ADD}"
                    }
                }
            }
        }
    }
}
