pub(crate) mod custom_sub;
pub mod note;
pub mod reply;

use std::{sync::Arc, time::Duration};

use dioxus::prelude::*;
use nostr_sdk::{client, Event};
use wasm_bindgen::{closure::Closure, JsCast, JsValue};

use crate::{
    nostr::{fetch::EventPaginator, multiclient::MultiClient},
    store::{subscription::CustomSub, CBWebDatabase},
    utils::js::{get_scroll_info, throttle},
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
    let mut notes: Signal<Vec<Event>> = use_signal(|| Vec::new());
    let mut paginator: Signal<Option<EventPaginator>> = use_signal(|| None);

    let multiclient = use_context::<Signal<MultiClient>>();
    let cb_database_db = use_context::<Signal<CBWebDatabase>>();
    // use_r
    use_effect(use_reactive((&name,), move |(s,)| {
        for (i, sub) in all_sub.read().iter().enumerate() {
            if sub.name == s {
                sub_current.set(sub.clone());
                sub_index.set(i);
            }
        }
    }));
    spawn(async move {
        let sub = sub_current.read().clone();
        let filters = sub.get_filters();
        tracing::info!("Subscription: {:#?}", filters);
        let mut clients = multiclient();
        let client_result = clients.get_or_create(&sub.relay_set).await;

        match client_result {
            Ok(hc) => {
                let client = hc.client();
                let paginator_result = EventPaginator::new(&client, filters, None, 40);
                // paginator.set(Some(paginator_result));
            },
            Err(e) => {
                tracing::error!("Error: {:?}", e);
            }
        };
    });

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

                    {
                        sub_current.set(value.clone());
                    }
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
        });
    };
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

    let on_mounted = move |_| {
        if name.is_empty() {
            return;
        }
        handle_fetch();
    };
    let mut index = use_signal(|| 0);

    let handle_reload = move |_: CustomSub| {
        //todo
        index += 1;
        // sub_current.set(value);
    };
    rsx! {
        div {
            onmounted: on_mounted,
            class:"flex-box",
            div {
                class:"flex-box-left",
                id: "note-list",
                onscroll: move |_| {
                    let callback = Closure::wrap(Box::new({
                        // let sub_current = sub_current.clone();
                        move || {
                            match get_scroll_info("note-list") {
                                Ok(scroll_info) => {
                                    let scroll_top = scroll_info.scroll_top;
                                    let scroll_height = scroll_info.scroll_height;
                                    let client_height = scroll_info.client_height;
                                    if scroll_height - scroll_top - client_height <= 100 {
                                        tracing::info!("scroll to bottom");
                                    }
                                }
                                Err(e) => {
                                    tracing::error!("Error: {:?}", e);
                                }
                            }
                        }
                    })  as Box<dyn Fn()>);
                    let callback_js = callback.into_js_value();
                    let throttled_callback = throttle(callback_js, 1000); // 300ms throttling delay
                    let func: &js_sys::Function = throttled_callback.as_ref().unchecked_ref();
                    func.call0(&JsValue::NULL).unwrap();
                    // throttled_callback();
                    // throttle(move || {
                    //     get_scroll_info("note-list");
                    // }, 100);
                    // get_scroll_info("note-list");
                    // e.data().get;
                    // tracing::info!("scroll: {:?}", e.get_scroll_offset());
                },
                // List {
                //     index: index(),
                //     subscription: sub_current.read().clone(),
                //     events: vec![],
                // }
                div {
                    class: "note-more-mod-box",
                    div {
                        class: "note-more-mod-box",
                        for (i, note) in notes().clone().iter().enumerate() {
                            Note {
                                sub_name: sub_current().name.clone(),
                                event: note.clone(),
                                relay_name: sub_current().relay_set.clone(),
                                note_index: i,
                            }
                        }
                    }
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
    events: Vec<Event>,
}

#[component]
pub fn List(props: ListProps) -> Element {
    // let mut sub_current = use_signal(|| props.subscription.clone());

    // let mut notes: Signal<Vec<Event>> = use_signal(std::vec::Vec::new);
    // let mut index = use_signal(|| 1);

    // let multiclient = use_context::<Signal<MultiClient>>();
    let events = use_signal(|| props.events.clone());
    // // get events from relay && set data to database and notes
    // let handle_fetch = move || {
    //     spawn(async move {
    //         let sub = sub_current.read().clone();
    //         let filters = sub.get_filters();
    //         tracing::info!("Subscription: {:#?}", filters);
    //         let mut clients = multiclient();
    //         let client_result = clients.get_or_create(&sub.relay_set).await;

    //         let hc = match client_result {
    //             Ok(hc) => hc,
    //             Err(e) => {
    //                 tracing::error!("Error: {:?}", e);
    //                 return;
    //             }
    //         };

    //         let client = hc.client();
    //         // TODO: use global client by this subscription
    //         tracing::info!("Filters: {:#?}", filters);
    //         // TODO: use the 'subscribe' function if this sub requires subscription
    //         match client
    //             .get_events_of(filters, Some(Duration::from_secs(5)))
    //             .await
    //         {
    //             Ok(events) => {
    //                 // TODO: add or append to database
    //                 // notes.clear();
    //                 notes.extend(events);
    //             }
    //             Err(e) => {
    //                 tracing::error!("Error: {:?}", e);
    //             }
    //         }
    //     })
    // };

    // let handle_load = move || {
    //     let count = notes.read().len();
    //     if count == 0 {
    //         // let database = WebDatabase::open("events_database").await.unwrap();
    //         // TODO: load from database
    //         // if load_from_database.len() == 0 {
    //         //     handle_fetch();
    //         // } else {
    //         //     set data to notes
    //         // }
    //         handle_fetch();
    //     }
    // };

    // use_effect(use_reactive(
    //     (&props.index, &props.subscription),
    //     move |(i, sub)| {
    //         if i != index() {
    //             tracing::info!("Subscription changed: {:?}", index());
    //             sub_current.set(sub);
    //             index.set(i);
    //             notes.clear();
    //             handle_load();
    //         }
    //     },
    // ));
    let eles = use_memo(move || {
        let mut eles = vec![];
        for (i, note) in events.read().iter().enumerate() {
            eles.push(rsx! {
                Note {
                    sub_name: props.subscription.name.clone(),
                    event: note.clone(),
                    relay_name: props.subscription.relay_set.clone(),
                    note_index: i,
                }
            });
        }
        eles
    });
    rsx! {
        // div {
        //     class: "note-more-mod-box",
        //     for (i, note) in events.read().clone().iter().enumerate() {
        //         Note {
        //             sub_name: props.subscription.name.clone(),
        //             event: note.clone(),
        //             relay_name: props.subscription.relay_set.clone(),
        //             note_index: i,
        //         }
        //     }
        // }
    }
}
