mod account;
mod add_filter;
mod event;
mod hashtag;
mod input;
mod kind;
mod limit;
pub mod relays;
mod tag;

use chrono::format;
use dioxus::prelude::*;
use regex::Regex;

use crate::{
    components::{icons::*, DateTimePicker, Dropdown, Switch},
    store::subscription::{Account, CustomSub, Event, FilterTemp, RelaySet, Tag},
    utils::{contants::NUM_AND_LETTER_REG, js::alert},
    Route, // utils::js::{export_to_clipboard, import_from_clipboard},
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
    // subscription: CustomSub,
    sub_name: String,
}

#[component]
pub fn CustomSubscription(props: CustomSubscriptionProps) -> Element {
    let mut sub_name = use_signal(|| props.sub_name.clone());
    let mut sub_current = use_signal(|| props.subscription.clone());
    let all_subs = use_context::<Vec<CustomSub>>();
    use_effect(use_reactive(
        (&props.subscription,),
        move |(subscription,)| {
            sub_current.set(subscription.clone());
        },
    ));
    let mut edit = use_context_provider(|| Signal::new(false));

    let handle_reset = move |_| {
        sub_current.set(props.subscription.clone());
        edit.set(false);
    };

    let handle_save = move |_| {
        props.on_save.call(sub_current().clone());
        edit.set(false);
    };

    let handle_reload = move |_| {
        tracing::info!("emit reload");
        props.on_reload.call(sub_current.read().clone());
    };

    let handle_import = move || {
        // spawn(async move {
        //     let value = import_from_clipboard().await;
        //     sub_current.set(CustomSub::from(&value));
        // });
    };
    let handle_export = move || {
        // spawn(async move {
        //     let value = import_from_clipboard().await;
        //     sub_current.set(CustomSub::from(&value));
        // });
    };
    let handle_change_subname = move |v: String| {
        // check sub name
        if v.is_empty() {
            return;
        }
        {
            let mut sub = sub_current.write();
            sub.name = v;
        };
        {
            let sub = sub_current();
            tracing::info!("save sub: {:#?}", sub);
            props.on_save.call(sub.clone());
        }
    };
    let handle_change_replyset = move |v: RelaySet| {
        {
            let mut sub: Write<CustomSub, UnsyncStorage> = sub_current.write();
            sub.relay_set.clone_from(&v.name);
        }
        {
            let sub = sub_current();
            tracing::info!("save sub: {:#?}", sub);
            props.on_save.call(sub.clone());
        }
    };

    rsx! {
        div {
            class: "sub-style custom-sub",
            div {
                class: "custom-sub-header",
                div {
                    class: "sub-header",
                    h2 {
                      class:"custom-sub-family",
                      "Custom Sub"
                    }
                    button {
                        class: "btn-icon purple small",
                        onclick: handle_reload,
                        dangerous_inner_html: "{RELOAD}",
                    }
                }
                div {
                    class: "custom-sub-header-more btnSvg",
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
                                    span{
                                      dangerous_inner_html: "{IMPORTICON}",
                                    }
                                    "Import"
                                }
                                button {
                                    class: "content-btn",
                                    onclick: move |_| handle_export(),
                                    span{
                                      dangerous_inner_html: "{EXPORTICON}",
                                    }
                                    "Export"
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
                    div {
                        class: "width-80-fontSize-16",
                        "Name:"
                    }
                    Input {
                        edit: true,
                        on_change: handle_change_subname,
                        value: "{sub_current().name}",
                    }
                }
                div {
                    class: "custom-sub-relays",
                    div {
                        class: "width-80-fontSize-16",
                        "Relays:"
                    }
                    div {
                        class:"display-inline-block",
                        RelaysInput {
                            on_change: handle_change_replyset,
                            relay_name: &sub_current.read().relay_set,
                            is_popup: true,
                        }
                    }
                }
                div {
                    class: "custom-sub-live",
                    div {
                        class: "width-80-fontSize-16",
                        "Live:"
                    }
                    div {
                        class:"display-inline-block",
                        div {
                            class:"flex-box-center",
                            Switch {
                                value: sub_current().live,
                                on_change: move |value: bool| {
                                    let mut sub = sub_current.write();
                                    sub.live = value;
                                },
                            }
                            button {
                                class: format!("btn-icon purple small {}", if sub_current().live { "display-none-important" } else { "display-inline-block" }),
                                onclick: handle_reload,
                                dangerous_inner_html: "{RELOAD}",
                            }
                        }
                    }
                }
                div {
                  class: "custom-sub-name",
                  div {
                      class: "width-80-fontSize-16",
                      "Filters:"
                  }
                  div {
                    class:"display-inline-block",
                    div {
                        class:"sub-edit-button",
                        if edit() {
                            button {
                              class: "btn-circle btn-circle-true",
                              onclick: handle_save,
                              dangerous_inner_html: "{TRUE}"
                            }
                            button {
                                class: "btn-circle btn-circle-false ml-5",
                                onclick: handle_reset,
                                dangerous_inner_html: "{FALSE}"
                            }
                        } else {
                          button {
                            class: "btn-icon purple small",
                            onclick: move |_| edit.set(true),
                            dangerous_inner_html: "{SUBEDIT}",
                          }
                        }
                    }
                  }
                }
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
                                        class: "{edit}",
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
        }
    }
}
