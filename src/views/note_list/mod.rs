pub(crate) mod custom_sub;
pub mod note;
pub mod reply;

use std::time::Duration;

use dioxus::prelude::*;
use nostr_sdk::Event;

use crate::{
    nostr::multiclient::MultiClient,
    store::{subscription::CustomSub, CBWebDatabase},
    Route,
};

use custom_sub::CustomSubscription;
use note::Note;

#[component]
pub fn NoteList(name: String) -> Element {
    tracing::info!("NoteList: {:?}", name);
    // all custom subscriptions
    let mut all_sub = use_context::<Signal<Vec<CustomSub>>>();

    let mut sub_current = use_signal(|| CustomSub::empty());
    let mut sub_index = use_signal(|| 0);
    let cb_database_db = use_context::<Signal<CBWebDatabase>>();

    use_effect(use_reactive((&name,), move |(s,)| {
        for (i, sub) in all_sub.read().iter().enumerate() {
            tracing::info!("name/name/name/subClone: {:?}", all_sub.len());
            tracing::info!("name/name/name/i: {:?}", i);
            if sub.name == s {
                sub_current.set(sub.clone());
                sub_index.set(i);
            }
        }
    }));

    let handle_save = move |value: CustomSub| {
        spawn(async move {
            let old_name = {
                let sub_current_lock = sub_current();
                sub_current_lock.name.clone()
            };
            let edit_value = value.clone();
            tracing::info!("Update: {:?}", edit_value);

            match cb_database_db()
                .update_custom_sub(old_name.clone(), edit_value.clone())
                .await
            {
                Ok(_) => {
                    let edit_name = edit_value.name.clone();

                    // 成功更新后再次获取 sub_current 并更新其值
                    {
                        sub_current.set(value.clone());
                    }

                    // 更新 all_sub 的值
                    let index: usize = *sub_index.read();
                    {
                        let mut subs: Write<_, UnsyncStorage> = all_sub.write();
                        subs[index] = sub_current().clone();
                    }

                    if old_name != edit_name {
                        navigator().replace(Route::NoteList { name: edit_name });
                    }
                    tracing::info!("Update success: wait for reload");
                }
                Err(e) => {
                    tracing::error!("Update error: {:?}", e);
                }
            }
            // {
            //     tracing::info!("Update success: wait for reload");
            //     sub_current.set(value.clone());
            //     let index: usize = *sub_index.read();
            //     let mut subs: Write<_, UnsyncStorage> = all_sub.write();
            //     subs[index] = sub_current.read().clone();
            // }
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

    let mut notes: Signal<Vec<Event>> = use_signal(std::vec::Vec::new);
    let mut index = use_signal(|| 1);

    let multiclient = use_context::<Signal<MultiClient>>();

    // get events from relay && set data to database and notes
    let handle_fetch = move || {
        spawn(async move {
            let sub = sub_current.read().clone();
            let filters = sub.get_filters();
            tracing::info!("Subscription: {:#?}", filters);
            let mut clients = multiclient();
            let client_result = clients.get_or_create(&sub.relay_set).await;

            let hc = match client_result {
                Ok(hc) => hc,
                Err(e) => {
                    tracing::error!("Error: {:?}", e);
                    return;
                }
            };

            let client = hc.client();
            // TODO: use global client by this subscription
            tracing::info!("Filters: {:#?}", filters);
            // TODO: use the 'subscribe' function if this sub requires subscription
            match client
                .get_events_of(filters, Some(Duration::from_secs(5)))
                .await
            {
                Ok(events) => {
                    // TODO: add or append to database
                    // notes.clear();
                    notes.extend(events);
                }
                Err(e) => {
                    tracing::error!("Error: {:?}", e);
                }
            }
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
            }
        }
    }
}
