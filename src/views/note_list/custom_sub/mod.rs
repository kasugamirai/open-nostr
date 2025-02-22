mod account;
mod add_filter;
mod event;
mod hashtag;
mod input;
mod kind;
mod limit;
pub mod relays;
mod tag;

use std::collections::HashMap;

use account::AccountInput;
use add_filter::AddFilter;
use chrono::format;
use dioxus::prelude::*;
use dioxus_elements::tr;
use event::EventInput;
use hashtag::{HashTagAdd, HashTagInput};
use input::Input;
use kind::KindInput;
use limit::LimitInput;
use regex::Regex;
use relays::RelaysInput;
use tag::TagInput;
use std::sync::Arc;
use crate::init::NEW_CUSTOM_SUB_KEY;

use crate::utils::js::{export_to_clipboard, import_from_clipboard};
use crate::{
    components::{icons::*, DateTimePicker, Dropdown, Switch},
    store::{
        subscription::{Account, CustomSub, Event, FilterTemp, RelaySet, Tag},
        CBWebDatabase,
    },
    utils::{contants::NUM_AND_LETTER_REG, js::{alert,verify_filters}},
    Route, // utils::js::{export_to_clipboard, import_from_clipboard},
};

#[derive(PartialEq, Clone, Props)]
pub struct CustomSubscriptionProps {
    on_save: EventHandler<CustomSub>,
    on_reload: EventHandler<CustomSub>,
    sub_name: String,
    is_add: bool,
}

#[component]
pub fn CustomSubscription(props: CustomSubscriptionProps) -> Element {
    let mut sub_name = use_signal(|| props.sub_name.clone());
    let mut sub_current = use_signal(|| CustomSub::empty());
    let mut old_sub: Signal<CustomSub> = use_signal(|| CustomSub::empty());
    let is_add: bool = props.is_add.clone();
    // let all_subs = use_context::<Vec<CustomSub>>();
    let mut subs_map = use_context::<Signal<HashMap<String, CustomSub>>>();
    let cb_database_db: Signal<CBWebDatabase> = use_context::<Signal<CBWebDatabase>>();
    // let mut is_new_name = use_signal(|| false);
    let mut edit = use_context_provider(|| Signal::new(false));
    
    use_effect(use_reactive((&props.sub_name,), move |(sub_name_new,)| {
        sub_name.set(sub_name_new.clone());
        sub_current.set(CustomSub::empty());
        {
            let subs_map_lock = subs_map();
            if subs_map_lock.contains_key(&sub_name_new) {
                let current = subs_map_lock.get(&sub_name_new).unwrap();
                sub_current.set(current.clone());
                old_sub.set(current.clone());
                edit.set(false);
            } else {
                if sub_name_new.eq(NEW_CUSTOM_SUB_KEY) {
                    let mut _sub_current = CustomSub::empty();
                    let mut init_sub_name = String::from(sub_name_new);
                    init_sub_name.push_str(&subs_map_lock.len().to_string());
                    _sub_current.name = init_sub_name;
                    sub_current.set(_sub_current.clone());
                    old_sub.set(_sub_current.clone());
                    // is_new_name.set(true);
                    edit.set(true);
                    tracing::info!("init sub_current: {:#?}", 111);
                }
            }
        }
    }));
    

    let handle_reset = move |_| {
        sub_current.set(old_sub.read().clone());
        edit.set(false);
    };
    
    

    let handle_save = move || {
        // TODO: save sub
        spawn(async move {
            
            let _filters  = sub_current().filters;
            let is_verify = verify_filters(&_filters).await;
            if  let Err(msg) = is_verify {
                alert(msg).await;
                return;
            }

            let old_name = sub_name();
            let edit_value = sub_current();
            tracing::info!("old name: {:#?}", old_name);
            tracing::info!("Update: {:#?}", edit_value);
            match cb_database_db()
                .update_custom_sub(old_name.clone(), edit_value.clone())
                .await
            {
                Ok(_) => {
                    let edit_name = edit_value.name.clone();
                    // update the current subscription
                    // {
                    //     sub_current.set(value.clone());
                    // }
                    {
                        let mut subs_map: Write<HashMap<String, CustomSub>, UnsyncStorage> =
                            subs_map.write();
                        tracing::info!("Update success: update subs_map {:?}", edit_value);
                        subs_map.insert(edit_name.clone(), edit_value.clone());
                        if old_name != edit_name {
                            subs_map.remove(&old_name);
                        };
                    }

                    // if old_name != edit_name {
                        navigator().replace(Route::Subscription { name: edit_name });
                    // } else {
                        // props.on_save.call(sub_current().clone());
                        // edit.set(false);
                    // }
                    tracing::info!("Update success: wait for reload");
                }
                Err(e) => {
                    tracing::error!("Update error: {:?}", e);
                    // alert("Update error");
                    alert(format!("Update error: {:?}", e)).await;
                    edit.set(false);
                }
            }
        });
    };

    let handle_reload = move |_| {
        tracing::info!("emit reload");
        props.on_reload.call(sub_current.read().clone());
    };

    let handle_import = move || {
        // TODO import from clipboard
        // spawn(async move {
        //     let value = import_from_clipboard().await;
        //     if value == "" {
        //         return;
        //     } else {
        //         sub_current.set(CustomSub::from(&value));
        //         handle_save();
        //     }
        // });
    };
    let handle_export = move || {
        spawn(async move {
            let _ = export_to_clipboard(sub_current().json()).await;
        });
    };
    let handle_change_subname = move |v: String| {
        let subs_map_lock = subs_map();
        // check sub name
        if v.is_empty() && v.eq(&sub_name()) {
            return;
        }

        {
            let mut sub = sub_current.write();
            sub.name = v;
        };
        {
            if !is_add {
                handle_save();
            }
            
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
            // props.on_save.call(sub.clone());
            if !is_add {
                handle_save();
            }
        }
    };
    let handle_change_live = move |v: bool| {
        {
            let mut sub: Write<CustomSub, UnsyncStorage> = sub_current.write();
            sub.live = v;
        }
        {
            let sub = sub_current();
            tracing::info!("save sub: {:#?}", sub);
            // props.on_save.call(sub.clone());
            if !is_add {
                handle_save();
            }
        }
    };

    let handle_new_save = move || {
        spawn(async move { 
            let old_name = sub_name();
            let edit_value = sub_current().clone();
            let _sub_name = edit_value.name.clone();
            let subs_map_lock = subs_map();
            if subs_map_lock.contains_key(&_sub_name){
                spawn(async move {
                    alert("The name already exists. Do not add it again".to_string()).await;
                });
                return;
            }else if _sub_name.eq(NEW_CUSTOM_SUB_KEY){
                spawn(async move {
                    alert("The default value of name: new is not allowed".to_string()).await;
                });
                let mut sub = sub_current.write();
                sub.name = sub_name();
                return;
            }

            let _filters  = sub_current().filters;
            let is_verify = verify_filters(&_filters).await;
            if  let Err(msg) = is_verify {
                alert(msg).await;
                return;
            }
            
            match cb_database_db()
                .save_custom_sub(edit_value.clone())
                .await
            {
                Ok(_) => {
                    {
                        let mut subs_map =
                            subs_map.write();
                        subs_map.insert(_sub_name.clone(), edit_value.clone());
                        navigator().replace(Route::Subscription { name: _sub_name });
                    }
                }
                Err(e) => {
                    tracing::error!("Save error: {:?}", e);
                    alert(format!("Save error: {:?}", e)).await;
                    edit.set(false);
                }
            }

        });
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

                    if sub_name()!=NEW_CUSTOM_SUB_KEY {
                        button {
                            class: "btn-icon purple small",
                            onclick: handle_reload,
                            dangerous_inner_html: "{RELOAD}",
                        }
                      }else{
                        button {
                            class: "btn-style-unify wh-70",
                            onclick: move |_| {
                                handle_new_save();
                            },
                            "save"
                        }
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
                                on_change: handle_change_live,
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
                              onclick: move |_| {
                                if !is_add {
                                    handle_save();
                                }
                              },
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
