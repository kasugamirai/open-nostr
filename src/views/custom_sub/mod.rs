mod account;
mod add_filter;
mod event;
mod hashtag;
mod input;
mod kind;
mod limit;
mod relays;
mod tag;

use dioxus::prelude::*;

use crate::{
    components::{icons::*, DateTimePicker, Dropdown},
    state::subscription::{Account, CustomSub, Event, FilterTemp, RelaySet, Tag},
    utils::js::{export_to_clipboard, import_from_clipboard},
};
use account::AccountInput;
use add_filter::AddFilter;
use event::EventInput;
use hashtag::{HashTagAdd, HashTagInput};
use input::Input;
use kind::KindInput;
use limit::LimitInput;
use relays::RelaysInput;
use tag::TagInput;

#[derive(PartialEq, Clone, Props)]
pub struct CustomSubscriptionProps {
    on_save: EventHandler<CustomSub>,
    on_reload: EventHandler<CustomSub>,
    subscription: CustomSub,
}

#[component]
pub fn CustomSubscription(props: CustomSubscriptionProps) -> Element {
    let mut sub_current = use_signal(|| props.subscription.clone());

    use_effect(use_reactive(
        (&props.subscription,),
        move |(subscription,)| {
            tracing::info!("use_effect subscription: {subscription:?}");
            sub_current.set(subscription.clone());
        },
    ));

    let mut edit = use_context_provider(|| Signal::new(false));

    let handle_reset = move |_| {
        sub_current.set(props.subscription.clone());
        edit.set(false);
    };

    let handle_save = move |_| {
        props.on_save.call(sub_current.read().clone());
        edit.set(false);
    };

    let handle_reload = move |_| {
        tracing::info!("handle_reload11111111111");
        props.on_reload.call(sub_current.read().clone());
    };

    let handle_import = move || {
        spawn(async move {
            let value = import_from_clipboard().await;
            sub_current.set(CustomSub::from(&value));
        });
    };

    let handle_export = move || {
        export_to_clipboard(sub_current.read().json());
    };

    rsx! {
        div {
            class: "custom-sub",
            div {
                class: "custom-sub-header",
                div {
                    style: "display: flex; align-items: center; gap: 10px;",
                    h2 { "Custom Sub" }
                    button {
                        class: "btn-icon purple small",
                        onclick: handle_reload,
                        dangerous_inner_html: "{RELOAD}",
                    }
                }
                div {
                    class: "custom-sub-header-more",
                    Dropdown {
                        pos: "right".to_string(),
                        mode: "active".to_string(),
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
                                    onclick: move |_| handle_import(),
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
                                        onclick: handle_save,
                                        "Save"
                                    }
                                    button {
                                        class: "content-btn",
                                        onclick: handle_reset,
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
                    Input {
                        edit: false,
                        on_change: move |v| {
                            let mut sub = sub_current.write();
                            sub.name = v;
                        },
                        value: "{sub_current().name}",
                    }
                }
                div {
                    class: "custom-sub-relays",
                    "Relays:"
                    div {
                        style: "display: inline-block;",
                        RelaysInput {
                            on_change: move |v: RelaySet| {
                                let mut sub = sub_current.write();
                                sub.relay_set = v;
                            },
                            relay_set: sub_current.read().relay_set.clone(),
                        }
                    }
                }
            }
            div {
                class: "custom-sub-filters",
                "Filters:"
            }
            for (i, filter) in sub_current.read().filters.iter().enumerate() {
                div {
                    class: "custom-sub-item",
                    button {
                        class: "custom-sub-item-remove {edit}",
                        dangerous_inner_html: "{FALSE}",
                        onclick: move |_| {
                            let mut sub = sub_current.write();
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
                                                    let mut sub = sub_current.write();
                                                    if let FilterTemp::HashTag(ref mut hashtag_ref) = sub.filters[i] {
                                                        if v.is_empty() {
                                                            hashtag_ref.tags.remove(j);
                                                        } else {
                                                            hashtag_ref.tags[j] = v;
                                                        }
                                                    }
                                                },
                                                tag: tag,
                                            }
                                        }
                                    }
                                    div {
                                        class: "btn-add {edit}",
                                        HashTagAdd {
                                            on_change: move |v| {
                                                let mut sub = sub_current.write();
                                                if let FilterTemp::HashTag(ref mut hashtag_ref) = sub.filters[i] {
                                                    hashtag_ref.tags.push(v);
                                                }
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
                                            let mut sub = sub_current.write();
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
                                                    let mut sub = sub_current.write();
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
                                            let mut sub = sub_current.write();
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
                                                    let mut sub = sub_current.write();
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
                                            let mut sub = sub_current.write();
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
                                            let mut sub = sub_current.write();
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
                                                    let mut sub = sub_current.write();
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
                                            let mut sub = sub_current.write();
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
                                            let mut sub = sub_current.write();
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
                                            let mut sub = sub_current.write();
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
                                                    let mut sub = sub_current.write();
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
                                            let mut sub = sub_current.write();
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
                            let mut sub = sub_current.write();
                            sub.filters.push(filter);
                        }
                    }
                }
            }
            // div {
            //     "{sub_current.read().json()}"
            // }
        }
    }
}
