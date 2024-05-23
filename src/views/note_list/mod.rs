mod custom_sub;
pub mod note;
pub mod quote;
pub mod reply;
pub mod note_wrapper;

use std::time::Duration;

use dioxus::prelude::*;
use nostr_sdk::{Event, JsonUtil, Kind};

use crate::{
    nostr::{multiclient::MultiClient, note::TextNote},
    store::{subscription::CustomSub, CBWebDatabase},
};

use custom_sub::CustomSubscription;
use note::Note;
use note_wrapper::Note_wrapper;


#[component]
pub fn NoteList(name: String) -> Element {
    tracing::info!("NoteList: {:?}", name);
    // {
    //     let url = match web_sys::window().unwrap().location().href() {
    //         Ok(url) => url,
    //         Err(_) => "".to_string(),
    //     };

    //     // 创建一个新的 URL 对象
    //     let url_object = match web_sys::Url::new(&url) {
    //         Ok(url) => url,
    //         Err(_) => panic!("Invalid URL"),
    //     };

    //     // 获取查询字符串（即 URL 中的 ?key=value 部分）
    //     let search_params = url_object.search_params();
    //     // 使用 UrlSearchParams API 来获取特定的搜索参数
    //     let my_param = search_params.get("my_param").unwrap();

    //     // 将参数值转换为字符串（如果需要的话）
    //     let my_param_str: String = my_param.into();

    //     tracing::info!("URL: {:?}", my_param_str);
    // }

    // all custom subscriptions
    let mut all_sub = use_context::<Signal<Vec<CustomSub>>>();

    let mut sub_current = use_signal(|| CustomSub::empty());
    let mut sub_index = use_signal(|| 0);
    let mut cb_database_db = use_context::<Signal<CBWebDatabase>>();

    use_effect(use_reactive((&name,), move |(s,)| {
        for (i, sub) in all_sub.read().iter().enumerate() {
            if sub.name == s {
                sub_current.set(sub.clone());
                sub_index.set(i);
            }
        }
    }));

    let handle_save = move |value: CustomSub| {
        spawn(async move {
            let old_name = sub_current.read().name.clone();

            sub_current.set(value.clone());
            tracing::info!("Save: {:?}", value);
            let index: usize = *sub_index.read();
            let mut subs = all_sub.write();
            subs[index] = sub_current.read().clone();

            // Capture necessary variables for the async block
            let sub_current_clone = sub_current.clone();
            let old_name_clone = old_name.clone();

            // Move the database write operation here
            let cb_database_db_write = cb_database_db.write(); // Ensure .await is used if necessary
            tracing::info!("Update: {:?}", sub_current_clone.read()); // Ensure you read from the Arc
            cb_database_db_write
                .update_custom_sub(old_name_clone, sub_current_clone.read().clone())
                .await
                .unwrap();
        });
    };

    let mut index = use_signal(|| 0);

    let handle_reload = move |_: CustomSub| {
        //todo
        index += 1;
        // sub_current.set(value);
    };

    rsx! {
        div {
            class:"flex-box",
            div {
                class:"flex-box-left",
                List {
                    index: index(),
                    subscription: sub_current.read().clone(),
                }
            }
            div {
                class: "sub-style",
                CustomSubscription {
                    on_save: handle_save,
                    on_reload: handle_reload,
                    subscription: sub_current.read().clone(),
                }
            }
        }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct ListProps {
    index: i32,
    subscription: CustomSub,
}

#[component]
pub fn List(props: ListProps) -> Element {
    let mut sub_current = use_signal(|| props.subscription.clone());

    let mut notes: Signal<Vec<Event>> = use_signal(|| vec![]);
    let mut index = use_signal(|| 1);

    let multiclient = use_context::<Signal<MultiClient>>();

    // get events from relay && set data to database and notes
    let handle_fetch = move || {
        spawn(async move {
            let sub = sub_current.read().clone();
            let filters = sub.get_filters();
            tracing::info!("Subscription: {:#?}", filters);
            let clients = multiclient();
            let client: &nostr_sdk::Client = &clients.get_or_create(&sub.relay_set).await;
            // TODO: use global client by this subscription
            tracing::info!("Filters: {:#?}", filters);
            // TODO: use the 'subscribe' function if this sub requires subscription
            let events = client
                .get_events_of(filters, Some(Duration::from_secs(180)))
                .await
                .unwrap();
            // TODO: add or append to database

            notes.clear();
            notes.extend(events);
        })
    };

    let handle_load = move || {
        let count = notes.read().len();
        if count == 0 {
            // let database = WebDatabase::open("events_database").await.unwrap();
            // TODO: load from database
            // if load_from_database.len() == 0 {
            //     handle_fetch();
            // } else {
            //     set data to notes
            // }
            handle_fetch();
        }
    };

    use_effect(use_reactive(
        (&props.index, &props.subscription),
        move |(i, sub)| {
            if i != index() {
                tracing::info!("Subscription changed: {:?}", index());
                sub_current.set(sub);
                index.set(i);
                notes.clear();
                handle_load();
            }
        },
    ));

    rsx! {
        div {
            onmounted: move |_| {
                handle_load();
            },
            class: "note-more-mod-box",
            for (i, note) in notes.read().clone().iter().enumerate() {
                Note {
                    sub_name: props.subscription.name.clone(),
                    event: note.clone(),
                    relay_name: props.subscription.relay_set.clone(),
                    note_index: i,
                }
                // Note_wrapper {
                //     sub_name: props.subscription.name.clone(),
                //     event: note.clone(),
                //     relay_name: props.subscription.relay_set.clone(),
                
                // }
            }
        }
    }
}

