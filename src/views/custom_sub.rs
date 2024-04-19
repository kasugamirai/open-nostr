mod account;
mod cus_tags;
mod limit;

use dioxus::prelude::*;
use nostr_sdk::Kind;

use crate::{
    components::{icons::*, DateTimePicker, Dropdown, InputCard, InputKv},
    state::subscription::{Account, CustomFilter, CustomSub, CustomTag, FilterTemp},
    utils::format::{format_public_key, format_timestamp},
};
use account::InputAccount;
use cus_tags::InputCusTag;
use limit::InputLimit;

fn kind_to_str(index: u64) -> String {
    let kind = Kind::from(index);
    match kind {
        _ => format!("{:?}", kind),
    }
}

static KINDS: [(Kind, &str, u64); 64] = [
    (Kind::Metadata, "Metadata", 0),
    (Kind::TextNote, "TextNote", 1),
    (Kind::RecommendRelay, "RecommendRelay", 2),
    (Kind::ContactList, "ContactList", 3),
    (Kind::OpenTimestamps, "OpenTimestamps", 1040),
    (Kind::EncryptedDirectMessage, "EncryptedDirectMessage", 4),
    (Kind::EventDeletion, "EventDeletion", 5),
    (Kind::Repost, "Repost", 6),
    (Kind::GenericRepost, "GenericRepost", 16),
    (Kind::Reaction, "Reaction", 7),
    (Kind::BadgeAward, "BadgeAward", 8),
    (Kind::ChannelCreation, "ChannelCreation", 40),
    (Kind::ChannelMetadata, "ChannelMetadata", 41),
    (Kind::ChannelMessage, "ChannelMessage", 42),
    (Kind::ChannelHideMessage, "ChannelHideMessage", 43),
    (Kind::ChannelMuteUser, "ChannelMuteUser", 44),
    (Kind::PublicChatReserved45, "PublicChatReserved45", 45),
    (Kind::PublicChatReserved46, "PublicChatReserved46", 46),
    (Kind::PublicChatReserved47, "PublicChatReserved47", 47),
    (Kind::PublicChatReserved48, "PublicChatReserved48", 48),
    (Kind::PublicChatReserved49, "PublicChatReserved49", 49),
    (Kind::WalletConnectInfo, "WalletConnectInfo", 13194),
    (Kind::Reporting, "Reporting", 1984),
    (Kind::Label, "Label", 1985),
    (Kind::ZapPrivateMessage, "ZapPrivateMessage", 9733),
    (Kind::ZapRequest, "ZapRequest", 9734),
    (Kind::ZapReceipt, "ZapReceipt", 9735),
    (Kind::MuteList, "MuteList", 10000),
    (Kind::PinList, "PinList", 10001),
    (Kind::Bookmarks, "Bookmarks", 10003),
    (Kind::Communities, "Communities", 10004),
    (Kind::PublicChats, "PublicChats", 10005),
    (Kind::BlockedRelays, "BlockedRelays", 10006),
    (Kind::SearchRelays, "SearchRelays", 10007),
    (Kind::SimpleGroups, "SimpleGroups", 10009),
    (Kind::Interests, "Interests", 10015),
    (Kind::Emojis, "Emojis", 10030),
    (Kind::RelayList, "RelayList", 10002),
    (Kind::Authentication, "Authentication", 22242),
    (Kind::WalletConnectRequest, "WalletConnectRequest", 23194),
    (Kind::WalletConnectResponse, "WalletConnectResponse", 23195),
    (Kind::NostrConnect, "NostrConnect", 24133),
    (Kind::LiveEvent, "LiveEvent", 30311),
    (Kind::LiveEventMessage, "LiveEventMessage", 1311),
    (Kind::ProfileBadges, "ProfileBadges", 30008),
    (Kind::BadgeDefinition, "BadgeDefinition", 30009),
    (Kind::Seal, "Seal", 13),
    (Kind::GiftWrap, "GiftWrap", 1059),
    (Kind::SealedDirect, "SealedDirect", 14),
    (Kind::SetStall, "SetStall", 30017),
    (Kind::SetProduct, "SetProduct", 30018),
    (Kind::JobFeedback, "JobFeedback", 7000),
    (Kind::FollowSets, "FollowSets", 30000),
    (Kind::RelaySets, "RelaySets", 30002),
    (Kind::BookmarkSets, "BookmarkSets", 30003),
    (Kind::ArticlesCurationSets, "ArticlesCurationSets", 30004),
    (Kind::VideosCurationSets, "VideosCurationSets", 30005),
    (Kind::InterestSets, "InterestSets", 30015),
    (Kind::EmojiSets, "EmojiSets", 30030),
    (Kind::ReleaseArtifactSets, "ReleaseArtifactSets", 30063),
    (Kind::LongFormTextNote, "LongFormTextNote", 30023),
    (Kind::FileMetadata, "FileMetadata", 1063),
    (Kind::HttpAuth, "HttpAuth", 27235),
    (
        Kind::ApplicationSpecificData,
        "ApplicationSpecificData",
        30078,
    ),
];

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
                        FilterTemp::Aaccounts(kinds, accounts) => {
                            rsx! {
                                div {
                                    class: "custom-sub-filter-item",
                                    span {
                                        class: "title",
                                        "Kinds:"
                                    }
                                    for kind in kinds.iter() {
                                        div {
                                            class: "card custom-sub-kind",
                                            "{kind_to_str(*kind)}"
                                        }
                                    }
                                    Dropdown {
                                        pos: "right",
                                        trigger: rsx! {
                                            button {
                                                class: "btn-add",
                                                dangerous_inner_html: "{ADD}",
                                            }
                                        },
                                        children: rsx! {
                                            div {
                                                class: "btn-add-content",
                                                style: r#"
                                                    display: grid;
                                                    grid-template-columns: repeat(3, 1fr);
                                                    gap: 8px;
                                                "#,
                                                for kind in KINDS.iter() {
                                                    div {
                                                        style: "display: flex; gap: 8px;",
                                                        "{kind.1}"
                                                        input {
                                                            r#type: "checkbox",
                                                            oninput: move |event| {
                                                                let is_enabled = event.value() == "true";
                                                                let index = kind.2;
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
                                                    }
                                                }
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
                                    for (j, account) in accounts.iter().enumerate() {
                                        div {
                                            class: "custom-sub-account",
                                            InputAccount {
                                                edit: account.npub.is_empty(),
                                                on_change: move |a: Account| {
                                                    let mut sub = custom_sub.write();
                                                    if let FilterTemp::Aaccounts(_, ref mut accounts_ref) = sub.filters[i] {
                                                        if a.npub.is_empty() {
                                                            accounts_ref.remove(j);
                                                        } else {
                                                            accounts_ref[j] = a;
                                                        }
                                                    }
                                                },
                                                placeholder: Some(("pubkey/npub".to_string(), "alt name".to_string())),
                                                value: account.clone(),
                                            }
                                        }
                                    }
                                    button {
                                        class: "btn-add",
                                        dangerous_inner_html: "{ADD}",
                                        onclick: move |_| {
                                            let mut sub = custom_sub.write();
                                            if let FilterTemp::Aaccounts(_, ref mut accounts_ref) = sub.filters[i] {
                                                accounts_ref.push(Account::empty());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        FilterTemp::Events(events) => {
                            rsx! {
                                div {
                                    class: "custom-sub-filter-item",
                                    span {
                                        class: "title",
                                        "Notes:"
                                    }
                                    for (j, event) in events.iter().enumerate() {
                                        div {
                                            class: "custom-sub-event",
                                            InputKv {
                                                edit: event[1].is_empty(),
                                                on_change: move |(k, v): (String, String)| {
                                                    let mut sub = custom_sub.write();
                                                    if let FilterTemp::Events(ref mut events_ref) = sub.filters[i] {
                                                        if v.is_empty() {
                                                            events_ref.remove(j);
                                                        } else {
                                                            events_ref[j] = vec!["id/nevent".to_string(), v];
                                                        }
                                                    }
                                                },
                                                placeholder: Some(("id/nevent".to_string(), "alt name".to_string())),
                                                value: (event[0].clone(), event[1].clone()),
                                            }
                                        }
                                    }
                                    button {
                                        class: "btn-add",
                                        dangerous_inner_html: "{ADD}",
                                        onclick: move |_| {
                                            let mut sub = custom_sub.write();
                                            if let FilterTemp::Events(ref mut events_ref) = sub.filters[i] {
                                                events_ref.push(vec![String::from(""), String::from("")]);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        FilterTemp::Customize(filter) => {
                            rsx! {
                                div {
                                    class: "custom-sub-filter-item",
                                    span {
                                        class: "title",
                                        "Kinds:"
                                    }
                                    for kind in filter.kinds.iter() {
                                        div {
                                            class: "card custom-sub-kind",
                                            "{kind_to_str(*kind)}"
                                        }
                                    }
                                    Dropdown {
                                        pos: "right",
                                        trigger: rsx! {
                                            button {
                                                class: "btn-add",
                                                dangerous_inner_html: "{ADD}",
                                            }
                                        },
                                        children: rsx! {
                                            div {
                                                class: "btn-add-content",
                                                style: r#"
                                                    display: grid;
                                                    grid-template-columns: repeat(3, 1fr);
                                                    gap: 8px;
                                                "#,
                                                for kind in KINDS.iter() {
                                                    div {
                                                        style: "display: flex; gap: 8px;",
                                                        "{kind.1}"
                                                        input {
                                                            r#type: "checkbox",
                                                            oninput: move |event| {
                                                                let is_enabled = event.value() == "true";
                                                                let index = kind.2;
                                                                if is_enabled {
                                                                    let mut sub = custom_sub.write();
                                                                    if let FilterTemp::Customize(ref mut filter_ref) = sub.filters[i] {
                                                                        if !filter_ref.kinds.contains(&index) {
                                                                            filter_ref.kinds.push(index);
                                                                        }
                                                                    }
                                                                } else {
                                                                    let mut sub = custom_sub.write();
                                                                    if let FilterTemp::Customize(ref mut filter_ref) = sub.filters[i] {
                                                                        filter_ref.kinds.retain(|&x| x != index);
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
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
                                    for (j, account) in filter.accounts.iter().enumerate() {
                                        div {
                                            class: "custom-sub-account",
                                            InputAccount {
                                                edit: account.npub.is_empty(),
                                                on_change: move |a: Account| {
                                                    let mut sub = custom_sub.write();
                                                    if let FilterTemp::Customize(ref mut filter_ref) = sub.filters[i] {
                                                        if a.npub.is_empty() {
                                                            filter_ref.accounts.remove(j);
                                                        } else {
                                                            filter_ref.accounts[j] = a;
                                                        }
                                                    }
                                                },
                                                placeholder: Some(("pubkey/npub".to_string(), "alt name".to_string())),
                                                value: account.clone(),
                                            }
                                        }
                                    }
                                    button {
                                        class: "btn-add",
                                        dangerous_inner_html: "{ADD}",
                                        onclick: move |_| {
                                            let mut sub = custom_sub.write();
                                            if let FilterTemp::Customize(ref mut filter_ref) = sub.filters[i] {
                                                filter_ref.accounts.push(Account::empty());
                                            }
                                        }
                                    }
                                }
                                div {
                                    class: "custom-sub-filter-item",
                                    span {
                                        class: "title",
                                        "Time:"
                                    }
                                    DateTimePicker {
                                        value: filter.time.0,
                                        end: filter.time.1,
                                        on_change: move |_| {},
                                    }
                                }
                                div {
                                    class: "custom-sub-filter-item",
                                    span {
                                        class: "title",
                                        "Limit:"
                                    }
                                    InputLimit {
                                        edit: false,
                                        on_change: move |v: u64| {
                                            let mut sub = custom_sub.write();
                                            if let FilterTemp::Customize(ref mut filter_ref) = sub.filters[i] {
                                                filter_ref.limit = v;
                                            }
                                        },
                                        placeholder: "limit".to_string(),
                                        value: filter.limit,
                                    }
                                }
                                div {
                                    class: "custom-sub-filter-item",
                                    span {
                                        class: "title",
                                        "Tags:"
                                    }
                                    for (j, tag) in filter.tags.iter().enumerate() {
                                        div {
                                            class: "custom-sub-tag",
                                            InputCusTag {
                                                edit: tag.value.is_empty(),
                                                on_change: move |c: CustomTag| {
                                                    let mut sub = custom_sub.write();
                                                    if let FilterTemp::Customize(ref mut filter_ref) = sub.filters[i] {
                                                        if c.value.is_empty() {
                                                            filter_ref.tags.remove(j);
                                                        } else {
                                                            filter_ref.tags[j] = c;
                                                        }
                                                    }
                                                },
                                                placeholder: Some(("tag".to_string(), "value".to_string())),
                                                value: tag.clone(),
                                            }
                                        }
                                    }
                                    button {
                                        class: "btn-add",
                                        dangerous_inner_html: "{ADD}",
                                        onclick: move |_| {
                                            let mut sub = custom_sub.write();
                                            if let FilterTemp::Customize(ref mut filter_ref) = sub.filters[i] {
                                                filter_ref.tags.push(CustomTag::empty());
                                            }
                                        }
                                    }
                                }
                            }
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
                                        let cf = CustomFilter {
                                            kinds: vec![],
                                            accounts: vec![],
                                            time: (0, 0),
                                            limit: 500,
                                            tags: vec![]
                                        };
                                        sub.filters.push(FilterTemp::Customize(cf));
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
