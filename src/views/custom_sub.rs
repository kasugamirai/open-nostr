use dioxus::prelude::*;
use nostr_sdk::{Filter, Kind};

use crate::{
    components::{icons::*, Dropdown, InputCard},
    state::subscription::{CustomSub, FilterTemp},
    utils::format::{format_public_key, format_timestamp},
};

fn kind_to_str(index: u64) -> String {
    let kind = Kind::from(index);
    match kind {
        _ => format!("{:?}", kind),
    }
}

#[component]
pub fn CustomSub() -> Element {
    let mut custom_sub = use_signal(|| CustomSub::default());
    let mut new_relay = use_signal(|| String::new());
    let mut edit = use_signal(|| false);
    let handle_export = move || {
        let eval = eval(
            r#"
                let c = navigator.clipboard;
                if (!c) {
                    console.error('Clipboard not supported');
                    return false;
                }
                let msg = await dioxus.recv();
                console.log(msg);
                await c.writeText(msg);
                alert("Copied to clipboard");
                return true;
            "#,
        );
        eval.send(custom_sub.read().json().into()).unwrap();
    };

    rsx! {
        div {
            class: "custom-sub",
            div {
                class: "custom-sub-header",
                h2 { "Custom Sub" }
                div {
                    class: "custom-sub-header-more",
                    Dropdown {
                        pos: "right".to_string(),
                        trigger: rsx! {
                            button {
                                class: "trigger",
                                dangerous_inner_html: "{MORE}"
                            }
                        },
                        children: rsx! {
                            div {
                                class: "content",
                                button {
                                    class: "content-btn",
                                    "Import"
                                }
                                button {
                                    class: "content-btn",
                                    onclick: move |_| handle_export(),
                                    "Export"
                                }
                                if edit() {
                                    button {
                                        class: "content-btn",
                                        onclick: move |_| edit.set(false),
                                        "Save"
                                    }
                                    button {
                                        class: "content-btn",
                                        onclick: move |_| edit.set(false),
                                        "Reset"
                                    }
                                } else {
                                    button {
                                        class: "content-btn",
                                        onclick: move |_| edit.set(true),
                                        "Edit"
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div {
                class: "custom-sub-content",
                div {
                    class: "custom-sub-name",
                    "Name:"
                    InputCard {
                        edit: false,
                        on_change: move |v| {
                            let mut sub = custom_sub.write();
                            sub.name = v;
                        },
                        placeholder: None,
                        value: "{custom_sub.read().name}",
                    }
                }
                div {
                    class: "custom-sub-relays",
                    "Relays:"
                    div {
                        style: "display: inline-block;",
                        Dropdown {
                            pos: "left".to_string(),
                            trigger: rsx! {
                                div {
                                    class: "card disabled",
                                    "{custom_sub.read().relay_set.name}"
                                }
                            },
                            div {
                                style: r#"
                                    display: flex;
                                    flex-direction: column;
                                    gap: 10px;
                                    padding: 10px;
                                    border-radius: var(--radius-1);
                                    border: 1px solid var(--boc-1);
                                    background-color: var(--bgc-0);
                                "#,
                                div {
                                    style: "display: flex; gap: 10px;",
                                    input {
                                        style: r#"
                                            border: none;
                                            border-bottom: 1px solid var(--boc-1);
                                            font-size: 16px;
                                        "#,
                                        r#type: "text",
                                        value: "{custom_sub.read().relay_set.name}",
                                        oninput: move |event| {
                                            let mut sub = custom_sub.write();
                                            sub.relay_set.name = event.value();
                                        }
                                    }
                                    button {
                                        class: "btn-icon right",
                                        onclick: move |_| {},
                                        div {
                                            dangerous_inner_html: "{TRUE}"
                                        }
                                    }
                                }
                                for (i, relay) in custom_sub.read().relay_set.iter().enumerate() {
                                    div {
                                        style: "display: flex; gap: 10px;",
                                        input {
                                            style: r#"
                                                border: none;
                                                border-bottom: 1px solid var(--boc-1);
                                                font-size: 16px;
                                            "#,
                                            r#type: "text",
                                            value: "{relay}",
                                            oninput: move |event| {
                                                let mut sub = custom_sub.write();
                                                sub.relay_set.relays[i] = event.value();
                                            }
                                        }
                                        button {
                                            class: "btn-icon remove",
                                            onclick: move |_| {
                                                let mut sub = custom_sub.write();
                                                sub.relay_set.remove(i);
                                            },
                                            div {
                                                dangerous_inner_html: "{FALSE}"
                                            }
                                        }
                                    }
                                }
                                div {
                                    style: "display: flex; gap: 10px;",
                                    input {
                                        style: r#"
                                            border: none;
                                            border-bottom: 1px solid var(--boc-1);
                                            font-size: 16px;
                                        "#,
                                        r#type: "text",
                                        value: "{new_relay}",
                                        oninput: move |event| {
                                            new_relay.set(event.value());
                                        }
                                    }
                                    button {
                                        class: "btn-icon add",
                                        onclick: move |_| {
                                            let mut sub = custom_sub.write();
                                            sub.relay_set.push(new_relay());
                                        },
                                        div {
                                            dangerous_inner_html: "{ADD}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div {
                class: "custom-sub-filters",
                "Filters:"
            }
            for (i, filter) in custom_sub.read().filters.iter().enumerate() {
                div {
                    class: "custom-sub-item",
                    match filter {
                        FilterTemp::HashTag(tags) => {
                            rsx! {
                                div {
                                    class: "custom-sub-filter-item",
                                    span {
                                        class: "title",
                                        "Tags:"
                                    }
                                    for (j, tag) in tags.iter().enumerate() {
                                        div {
                                            class: "custom-sub-tag",
                                            InputCard {
                                                edit: tag.is_empty(),
                                                on_change: move |v: String| {
                                                    let mut sub = custom_sub.write();
                                                    if let FilterTemp::HashTag(ref mut tags_ref) = sub.filters[i] {
                                                        if v.is_empty() {
                                                            tags_ref.remove(j);
                                                        } else {
                                                            tags_ref[j] = v;
                                                        }
                                                    }
                                                },
                                                placeholder: Some("Input".to_string()),
                                                value: tag,
                                            }
                                        }
                                    }
                                    button {
                                        class: "btn-add",
                                        dangerous_inner_html: "{ADD}",
                                        onclick: move |_| {
                                            let mut sub = custom_sub.write();
                                            if let FilterTemp::HashTag(ref mut tags_ref) = sub.filters[i] {
                                                tags_ref.push("".to_string());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        FilterTemp::Aaccounts(kinds, authors) => {
                            rsx! {
                                div {
                                    class: "custom-sub-filter-item",
                                    span {
                                        class: "title",
                                        "Kinds:"
                                    }
                                    for (j, kind) in kinds.iter().enumerate() {
                                        div {
                                            class: "custom-sub-kind",
                                            InputCard {
                                                on_change: move |v: String| {
                                                    let mut sub = custom_sub.write();
                                                    if let FilterTemp::Aaccounts(ref mut kinds_ref, _) = sub.filters[i] {
                                                        if v.is_empty() {
                                                            kinds_ref.remove(j);
                                                        } else {
                                                            let v = v.parse::<u64>().unwrap_or(0);
                                                            kinds_ref[j] = v;
                                                        }
                                                    }
                                                },
                                                placeholder: Some("Input".to_string()),
                                                value: kind_to_str(*kind),
                                            }
                                        }
                                    }
                                    Dropdown {
                                        trigger: rsx! {
                                            button {
                                                class: "btn-add",
                                                dangerous_inner_html: "{ADD}",
                                            }
                                        },
                                        children: rsx! {
                                            div {
                                                class: "btn-add-content",
                                                input {
                                                    r#type: "checkbox",
                                                    oninput: move |event| {
                                                        let is_enabled = event.value() == "true";
                                                        let index = nostr_sdk::Kind::TextNote.as_u64();
                                                        if is_enabled {
                                                            let mut sub = custom_sub.write();
                                                            if let FilterTemp::Aaccounts(ref mut kinds_ref, _) = sub.filters[i] {
                                                                if !kinds_ref.contains(&index) {
                                                                    kinds_ref.push(index);
                                                                }
                                                            }
                                                        } else {
                                                            let mut sub = custom_sub.write();
                                                            if let FilterTemp::Aaccounts(ref mut kinds_ref, _) = sub.filters[i] {
                                                                kinds_ref.retain(|&x| x != index);
                                                            }
                                                        }
                                                    }
                                                }
                                                "Note"
                                                input {
                                                    r#type: "checkbox",
                                                    oninput: move |event| {
                                                        let is_enabled = event.value() == "true";
                                                        let index = nostr_sdk::Kind::Repost.as_u64();
                                                        if is_enabled {
                                                            let mut sub = custom_sub.write();
                                                            if let FilterTemp::Aaccounts(ref mut kinds_ref, _) = sub.filters[i] {
                                                                if !kinds_ref.contains(&index) {
                                                                    kinds_ref.push(index);
                                                                }
                                                            }
                                                        } else {
                                                            let mut sub = custom_sub.write();
                                                            if let FilterTemp::Aaccounts(ref mut kinds_ref, _) = sub.filters[i] {
                                                                kinds_ref.retain(|&x| x != index);
                                                            }
                                                        }
                                                    }
                                                }
                                                "Repost"
                                            }
                                        }
                                    }
                                }
                                div {
                                    class: "custom-sub-filter-item",
                                    span {
                                        class: "title",
                                        "Accounts:"
                                    }
                                }
                            }
                        }
                        _ => {
                            rsx!{}
                        }
                    }
                }
            }
            div {
                class: "custom-sub-add",
                span {
                    Dropdown {
                        trigger: rsx! {
                            button {
                                class: "btn-add",
                                dangerous_inner_html: "{ADD}"
                            }
                        },
                        children: rsx! {
                            div {
                                class: "btn-add-content",
                                button {
                                    class: "btn-add-item",
                                    onclick: move |_| {
                                        let mut sub = custom_sub.write();
                                        sub.filters.push(FilterTemp::HashTag(vec![]));
                                    },
                                    "Only Tags"
                                }
                                button {
                                    class: "btn-add-item",
                                    onclick: move |_| {
                                        let mut sub = custom_sub.write();
                                        sub.filters.push(FilterTemp::Aaccounts(vec![], vec![]));
                                    },
                                    "Follow People"
                                }
                                button {
                                    class: "btn-add-item",
                                    onclick: move |_| {
                                        let mut sub = custom_sub.write();
                                        sub.filters.push(FilterTemp::Events(vec![]));
                                    },
                                    "Follow Notes"
                                }
                                button {
                                    class: "btn-add-item",
                                    onclick: move |_| {
                                        let mut sub = custom_sub.write();
                                        sub.filters.push(FilterTemp::Customize(Filter::new()));
                                    },
                                    "Customize"
                                }
                            }
                        }
                    }
                }
            }
            div {
                "{custom_sub.read().json()}"
            }
        }
    }
}
