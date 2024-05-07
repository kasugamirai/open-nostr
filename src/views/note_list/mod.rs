mod note;

use std::time::Duration;

use dioxus::prelude::*;
use nostr_sdk::{Client, Event};

use crate::state::subscription::CustomSub;
use note::{Note, NoteData};

#[derive(PartialEq, Clone, Props)]
pub struct NoteListProps {
    subscription: CustomSub,
}

#[component]
pub fn NoteList(props: NoteListProps) -> Element {
    let mut sub_current = use_signal(|| props.subscription.clone());

    let mut notes: Signal<Vec<Event>> = use_signal(|| vec![]);

    // get events from relay && set data to database and notes
    let handle_fetch = move || {
        spawn(async move {
            tracing::info!("handle_fetch");
            // TODO: use global client by this subscription
            let sub = sub_current.read().clone();
            let client = Client::default();
            client
                .add_relays(sub.relay_set.relays.clone())
                .await
                .unwrap();

            client.connect().await;

            let filters = sub.get_filters();

            // TODO: use the 'subscribe' function if this sub requires subscription
            let events = client
                .get_events_of(filters, Some(Duration::from_secs(180)))
                .await
                .unwrap();

            // TODO: add or append to database

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
        (&props.subscription,),
        move |(subscription,)| {
            let current_name = sub_current().name;
            if subscription.name != current_name {
                tracing::info!("use_effect: {subscription:?}");
                sub_current.set(subscription.clone());
                notes.clear();
                handle_load();
            }
        },
    ));

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 10px; width: 100%;",
            for (i, note) in notes.read().clone().iter().enumerate() {
                Note {
                    data: NoteData::from(note, i),
                }
            }
        }
    }
}
