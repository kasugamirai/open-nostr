pub(crate) mod custom_sub;
pub mod note;
pub mod reply;

use std::time::Duration;

use dioxus::prelude::*;
use nostr_sdk::Event;

use crate::{
    nostr::multiclient::MultiClient,
    store::{subscription::CustomSub, CBWebDatabase},
};

use custom_sub::CustomSubscription;
use note::Note;

#[component]
pub fn NoteList(name: String) -> Element {
    tracing::info!("NoteList: {:?}", name);
    // all custom subscriptions
    let mut all_sub = use_context::<Signal<Vec<CustomSub>>>();

    let mut sub_current = use_signal(CustomSub::empty);
    let mut sub_index = use_signal(|| 0);
    let mut cb_database_db = use_context::<Signal<CBWebDatabase>>();

    use_effect(use_reactive((&name,), move |(s,)| {
      for (i, sub) in all_sub.read().iter().enumerate() {
      for (i, sub) in all_sub.read().iter().enumerate() {

          tracing::info!("name/name/name/subClone: {:?}", all_sub.len());
          tracing::info!("name/name/name/i: {:?}", i);
        for (i, sub) in all_sub.read().iter().enumerate() {

          tracing::info!("name/name/name/subClone: {:?}", all_sub.len());
          tracing::info!("name/name/name/i: {:?}", i);
            if sub.name == s {
                sub_current.set(sub.clone());
                sub_index.set(i);
          }
          }

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
            
            let hc = clients.get_or_create(&sub.relay_set).await.unwrap();
            let client = hc.client();
            // TODO: use global client by this subscription
            tracing::info!("Filters: {:#?}", filters);
            // TODO: use the 'subscribe' function if this sub requires subscription
            match client.get_events_of(filters, Some(Duration::from_secs(5))).await {
                Ok(events) => {
                    // TODO: add or append to database
                    // notes.clear();
                    notes.extend(events);
                },
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
