mod account;
mod add_filter;
mod event;
mod hashtag;
mod kind;
mod limit;
mod relays;
mod tag;

use dioxus::prelude::*;

use crate::{
    components::{icons::*, DateTimePicker, Dropdown, InputCard},
    state::subscription::{Account, CustomSub, Event, FilterTemp, RelaySet, Tag},
};
use account::AccountInput;
use add_filter::AddFilter;
use event::EventInput;
use hashtag::HashTagInput;
use kind::KindInput;
use limit::LimitInput;
use relays::RelaysInput;
use tag::TagInput;

#[component]
pub fn CustomSub() -> Element {
    let mut custom_sub_global = use_context::<Signal<CustomSub>>();
    let mut custom_sub = use_signal(|| CustomSub::default());
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
                        show: "active".to_string(),
                        trigger: rsx! {
                            div {
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
                                        onclick: move |_| {
                                            custom_sub_global.set(custom_sub.read().clone());
                                            edit.set(false);
                                        },
                                        "Save"
                                    }
                                    button {
                                        class: "content-btn",
                                        onclick: move |_| {
                                            custom_sub.set(custom_sub_global.read().clone());
                                            edit.set(false);
                                        },
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
                        RelaysInput {
                            on_change: move |v: RelaySet| {
                                let mut sub = custom_sub.write();
                                sub.relay_set = v;
                            },
                            relay_set: custom_sub.read().relay_set.clone(),
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
                    button {
                        class: "custom-sub-item-remove {edit}",
                        dangerous_inner_html: "{FALSE}",
                        onclick: move |_| {
                            let mut sub = custom_sub.write();
                            sub.filters.remove(i);
                        }
                    }
                    match filter {
                        FilterTemp::HashTag(hashtag) => {
                            rsx! {
                                div {
                                    class: "custom-sub-filter-item",
                                    span {
                                        class: "title",
                                        "Tags:"
                                    }
                                    for (j, tag) in hashtag.tags.iter().enumerate() {
                                        div {
                                            class: "custom-sub-tag",
                                            HashTagInput {
                                                edit: tag.is_empty(),
                                                on_change: move |v: String| {
                                                    let mut sub = custom_sub.write();
                                                    if let FilterTemp::HashTag(ref mut hashtag_ref) = sub.filters[i] {
                                                        if v.is_empty() {
                                                            hashtag_ref.tags.remove(j);
                                                        } else {
                                                            hashtag_ref.tags[j] = v;
                                                        }
                                                    }
                                                },
                                                tag: tag,
                                                index: i * 10 + j,
                                            }
                                        }
                                    }
                                    button {
                                        class: "btn-add {edit}",
                                        dangerous_inner_html: "{ADD}",
                                        onclick: move |_| {
                                            let mut sub = custom_sub.write();
                                            if let FilterTemp::HashTag(ref mut hashtag_ref) = sub.filters[i] {
                                                hashtag_ref.tags.push("".to_string());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        FilterTemp::Accounts(accounts) => {
                            rsx! {
                                div {
                                    class: "custom-sub-filter-item",
                                    span {
                                        class: "title",
                                        "Kinds:"
                                    }
                                    KindInput {
                                        value: accounts.kinds.clone(),
                                        on_change: move |kinds| {
                                            let mut sub = custom_sub.write();
                                            if let FilterTemp::Accounts(ref mut accounts_ref) = sub.filters[i] {
                                                accounts_ref.kinds = kinds;
                                            }
                                        },
                                        index: i,
                                    }
                                }
                                div {
                                    class: "custom-sub-filter-item",
                                    span {
                                        class: "title",
                                        "Accounts:"
                                    }
                                    for (j, account) in accounts.accounts.iter().enumerate() {
                                        div {
                                            class: "custom-sub-account",
                                            AccountInput {
                                                edit: account.npub.is_empty(),
                                                on_change: move |a: Account| {
                                                    let mut sub = custom_sub.write();
                                                    if let FilterTemp::Accounts(ref mut accounts_ref) = sub.filters[i] {
                                                        if a.npub.is_empty() {
                                                            accounts_ref.accounts.remove(j);
                                                        } else {
                                                            accounts_ref.accounts[j] = a;
                                                        }
                                                    }
                                                },
                                                account: account.clone(),
                                                index: i * 10 + j,
                                            }
                                        }
                                    }
                                    button {
                                        class: "btn-add {edit}",
                                        dangerous_inner_html: "{ADD}",
                                        onclick: move |_| {
                                            let mut sub = custom_sub.write();
                                            if let FilterTemp::Accounts(ref mut accounts_ref) = sub.filters[i] {
                                                accounts_ref.accounts.push(Account::empty());
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
                                    for (j, event) in events.events.iter().enumerate() {
                                        div {
                                            class: "custom-sub-event",
                                            EventInput {
                                                edit: event.nevent.is_empty(),
                                                on_change: move |a: Event| {
                                                    let mut sub = custom_sub.write();
                                                    if let FilterTemp::Events(ref mut events_ref) = sub.filters[i] {
                                                        if a.nevent.is_empty() {
                                                            events_ref.events.remove(j);
                                                        } else {
                                                            events_ref.events[j] = a;
                                                        }
                                                    }
                                                },
                                                event: event.clone(),
                                                index: i * 10 + j,
                                            }
                                        }
                                    }
                                    button {
                                        class: "btn-add {edit}",
                                        dangerous_inner_html: "{ADD}",
                                        onclick: move |_| {
                                            let mut sub = custom_sub.write();
                                            if let FilterTemp::Events(ref mut events_ref) = sub.filters[i] {
                                                events_ref.events.push(Event::empty());
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
                                    KindInput {
                                        value: filter.kinds.clone(),
                                        on_change: move |kinds| {
                                            let mut sub = custom_sub.write();
                                            if let FilterTemp::Customize(ref mut filter_ref) = sub.filters[i] {
                                                filter_ref.kinds = kinds;
                                            }
                                        },
                                        index: i,
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
                                            AccountInput {
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
                                                account: account.clone(),
                                                index: i * 10 + j,
                                            }
                                        }
                                    }
                                    button {
                                        class: "btn-add {edit}",
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
                                        value: filter.since,
                                        end: filter.until,
                                        on_change: move |(start, end): (u64, u64)| {
                                            let mut sub = custom_sub.write();
                                            if let FilterTemp::Customize(ref mut filter_ref) = sub.filters[i] {
                                                filter_ref.since = start;
                                                filter_ref.until = end;
                                            }
                                        },
                                    }
                                }
                                div {
                                    class: "custom-sub-filter-item",
                                    span {
                                        class: "title",
                                        "Limit:"
                                    }
                                    LimitInput {
                                        edit: false,
                                        on_change: move |v: usize| {
                                            let mut sub = custom_sub.write();
                                            if let FilterTemp::Customize(ref mut filter_ref) = sub.filters[i] {
                                                filter_ref.limit = v;
                                            }
                                        },
                                        limit: filter.limit,
                                        index: i,
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
                                            TagInput {
                                                edit: tag.value.is_empty(),
                                                on_change: move |c: Tag| {
                                                    let mut sub = custom_sub.write();
                                                    if let FilterTemp::Customize(ref mut filter_ref) = sub.filters[i] {
                                                        if c.value.is_empty() {
                                                            filter_ref.tags.remove(j);
                                                        } else {
                                                            filter_ref.tags[j] = c;
                                                        }
                                                    }
                                                },
                                                tag: tag.clone(),
                                                index: i * 10 + j,
                                            }
                                        }
                                    }
                                    button {
                                        class: "btn-add {edit}",
                                        dangerous_inner_html: "{ADD}",
                                        onclick: move |_| {
                                            let mut sub = custom_sub.write();
                                            if let FilterTemp::Customize(ref mut filter_ref) = sub.filters[i] {
                                                filter_ref.tags.push(Tag::empty());
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
                    AddFilter {
                        on_click: move |filter| {
                            let mut sub = custom_sub.write();
                            sub.filters.push(filter);
                        }
                    }
                }
            }
            // div {
            //     "{custom_sub.read().json()}"
            // }
        }
    }
}
