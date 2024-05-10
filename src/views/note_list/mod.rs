mod custom_sub;
pub mod note;

use std::{collections::HashMap, time::Duration};

use dioxus::prelude::*;
use nostr_sdk::{Client, Event, RelayPoolNotification, SubscriptionId, Timestamp};

use crate::{state::subscription::CustomSub, storage::CapybastrDb};

use custom_sub::CustomSubscription;
use note::{Note, NoteData};

#[component]
pub fn NoteList(name: String) -> Element {
    // all custom subscriptions
    let mut sub_all = use_context::<Signal<Vec<CustomSub>>>();

    let mut sub_current = use_signal(|| CustomSub::empty());
    let mut sub_index = use_signal(|| 0);

    use_effect(use_reactive((&name,), move |(s,)| {
        for (i, sub) in sub_all.read().iter().enumerate() {
            if sub.name == s {
                sub_current.set(sub.clone());
                sub_index.set(i);
            }
        }
    }));

    let handle_save = move |value: CustomSub| {
        sub_current.set(value);
        let index = *sub_index.read();
        let mut subs = sub_all.write();
        subs[index] = sub_current.read().clone();

        let s = sub_current();

        spawn(async move {
            let db = CapybastrDb::new("subscription".to_string()).await.unwrap();
            db.delete_data(&s.name).await.unwrap();
            db.add_data(&s.name, &s).await.unwrap();

            // let db = CapybastrDb::new("subscription list".to_string()).await.unwrap();
            // db.delete_data("SUBSCRIPTION_LIST").await.unwrap();
            // db.add_data("SUBSCRIPTION_LIST", &String::from("[\"Dog\", \"Car\"]")).await.unwrap();
        });
    };

    let handle_reload = move |value: CustomSub| {
        let mut v = value.clone();
        v.tampstamp = Timestamp::now().as_i64();
        sub_current.set(v);
    };

    rsx! {
        div {
            style: "display: flex; width: 100%; height: 100%; gap: 20px;",
            div {
                style: "flex: 1; overflow-y: scroll; width: 100%;",
                List {
                    subscription: sub_current.read().clone(),
                }
            }
            div {
                style: "width: 600px; height: 100%; position: relative; display: flex; flex-direction: column; gap: 10px;",
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
    subscription: CustomSub,
}

#[component]
pub fn List(props: ListProps) -> Element {
    let mut sub_current = use_signal(|| props.subscription.clone());

    let mut notes: Signal<Vec<Event>> = use_signal(|| vec![]);

    let clients = use_context::<Signal<HashMap<String, Client>>>();

    // get events from relay && set data to database and notes
    let handle_fetch = move || {
        spawn(async move {
            // TODO: use global client by this subscription
            let sub = sub_current.read().clone();
            // let client = Client::default();
            // client
            //     .add_relays(sub.relay_set.relays.clone())
            //     .await
            //     .unwrap();

            // client.connect().await;

            let cs = clients();

            let c = cs.get(&sub.name).unwrap();

            let filters = sub.get_filters();

            // TODO: use the 'subscribe' function if this sub requires subscription
            let events = c
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
            let sub = sub_current();
            if subscription.tampstamp != sub.tampstamp || subscription.name != sub.name {
                sub_current.set(subscription.clone());
                notes.clear();
                handle_load();
            }
        },
    ));

    let handle_mounted = move || {
        spawn(async move {
            let sub = sub_current.read().clone();
            if sub.live {
                let cs = clients();
                let c = cs.get(&sub.name).unwrap();
                c.handle_notifications(|notification| async {
                    if let RelayPoolNotification::Event {
                        relay_url,
                        subscription_id,
                        event,
                    } = notification
                    {
                        if subscription_id == SubscriptionId::new(sub.name.clone()) {
                            tracing::info!("{relay_url}: {event:?}");
                        }
                    }
                    Ok(false) // Set to true to exit from the loop
                })
                .await
                .unwrap();
            }
        });
    };

    rsx! {
        div {
            onmounted: move |_| {
                handle_mounted();
            },
            style: "display: flex; flex-direction: column; gap: 10px; width: 100%;",
            for (i, note) in notes.read().clone().iter().enumerate() {
                Note {
                    data: NoteData::from(note, i),
                }
            }
        }
    }
}
