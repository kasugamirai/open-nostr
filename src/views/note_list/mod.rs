pub(crate) mod custom_sub;
pub mod detail_modal;
pub mod note;
pub mod reply;

use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::sync::atomic::Ordering;
use std::collections::HashMap;
use std::fmt::format;
use std::rc::Rc;
use std::sync::atomic::AtomicUsize;

use crate::components::icons::LOADING;
use crate::components::ModalManager;
use crate::nostr::multiclient::MultiClient;
use crate::nostr::register::{NotificationHandler, Register, RegisterError};
use crate::nostr::EventPaginator;
use crate::store::subscription::CustomSub;
use crate::utils::js::{get_scroll_info, throttle};
use dioxus::prelude::*;
use dioxus_elements::sub;
use futures::future::BoxFuture;
use nostr_indexeddb::database::Order;
use nostr_sdk::{Event, RelayMessage, RelayPoolNotification, SubscriptionId, Timestamp};
use note::Note;
use std::sync::{mpsc, Arc, Mutex, RwLock};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};

#[derive(Debug, Clone, Props, PartialEq)]
pub struct NoteListProps {
    pub name: String,
    pub reload_time: Timestamp,
    #[props(default = true)]
    pub is_cache: bool,
}
#[component]
pub fn NoteList(props: NoteListProps) -> Element {
    let NoteListProps {
        name,
        reload_time,
        is_cache,
    } = props;
    let mut reload_flag = use_signal(|| reload_time.clone());
    let mut sub_current = use_signal(CustomSub::empty);
    // let mut sub_index = use_signal(|| 0);
    let mut notes: Signal<Vec<Event>> = use_signal(|| Vec::new());
    let mut paginator = use_signal(|| None);
    let mut is_loading = use_signal(|| false);
    let mut sub_register = use_context::<Signal<Register>>();
    let mut modal_manager = use_context::<Signal<ModalManager>>();
    let subs_map = use_context::<Signal<HashMap<String, CustomSub>>>();
    let multiclient = use_context::<Signal<MultiClient>>();
    let new_event_counts = use_signal(|| 0);

    let handle_sub_list = move || -> NotificationHandler {
        Arc::new(move |notification| {
        // let new_event_counts = new_event_counts.clone();
            Box::pin(async move {
                match notification {
                    RelayPoolNotification::Message {
                        message: RelayMessage::Event { event, .. },
                        ..
                    } => {
                        tracing::info!(
                            "eventid: {:?}, author: {:?}, eventkind: {:?}, eventcontent: {:?}",
                            event.id.to_string(),
                            event.author().to_string(),
                            event.kind,
                            event.content
                        );
                        Ok(false) 
                    }
                    _ => {
                        tracing::info!("notification: {:?}", notification);
                        Ok(false)
                    }
                }
            })
        })
    };


    use_effect(use_reactive((&new_event_counts,), move |(val,)| {
        tracing::info!("new_event_counts: {:?}", val);
        // 可以在这里添加其他逻辑，比如重新渲染或其他操作
    }));
    let handle_fetch = move || {
        tracing::info!("handle_fetch");
        spawn(async move {
            if !is_loading() {
                is_loading.set(true);
                // if let Some(paginator_lock) = paginator() {
                let mut paginator_write: Write<Option<EventPaginator>, UnsyncStorage> =
                    paginator.write();
                let result = paginator_write.as_mut();
                if let Some(paginator) = result {
                    let events = paginator.next_page().await;
                    tracing::info!("handle_fetch 111");
                    match events {
                        Ok(events) => {
                            tracing::info!("handle_fetch 2222");
                            notes.extend(events.iter().cloned());
                            is_loading.set(false);
                        }
                        Err(e) => {
                            tracing::error!("Error: {:?}", e);
                            is_loading.set(false);
                        }
                    }
                } else {
                    is_loading.set(false);
                }
            }
        });
    };
    use_effect(use_reactive(
        (&name, &reload_time, &is_cache),
        move |(s, time, iscache)| {
            tracing::info!("name: {:?}, time: {:?}, iscache: {:?}", s, time, iscache);
            let subs_map_lock = subs_map();
            if subs_map_lock.contains_key(&s) {
                let current = subs_map_lock.get(&s).unwrap();
                sub_current.set(current.clone());
            }

            let handler = handle_sub_list.clone();
            tracing::info!("sub_current: {:?}", 1111);
            spawn(async move {
                tracing::info!("sub_current: {:?}", 2222);
                let sub_current = sub_current.read().clone();
                let filters = sub_current.get_filters();
                let mut clients = multiclient();
                tracing::info!("sub_current: {:?}", sub_current);
                let client_result = clients.get_or_create(&sub_current.relay_set).await;

                match client_result {
                    Ok(hc) => {
                        let client = hc.client();
                        {
                            let sub_id =
                                SubscriptionId::new(format!("note-list-{}", sub_current.name));
                            // let handle_sub_list: NotificationHandler = handle_sub_list.clone();
                            sub_register
                                .write()
                                .add_subscription(
                                    &client,
                                    sub_id.clone(),
                                    filters.clone(),
                                    handler(),
                                    None,
                                )
                                .await
                                .unwrap();
                            if sub_current.live {
                                spawn({
                                    let client = client.clone();
                                    async move {
                                        sub_register().handle_notifications(&client).await.unwrap();
                                    }
                                });
                            }
                            //     sub_register().set_stop_flag(&sub_id, true).await;
                            // }
                            tracing::info!("sub_id: {:?}", sub_id);
                        }
                        {
                            let paginator_result = EventPaginator::new(
                                client.clone(),
                                filters.clone(),
                                None,
                                40,
                                sub_current.live,
                            );
                            paginator.set(Some(paginator_result));
                            // notes.clear();
                            // reload_flag.set(time);
                        }
                        {
                            tracing::info!("is_cache: {:?}", iscache);
                            if !iscache && !sub_current.live {
                                notes.set(vec![]);
                                handle_fetch();
                                return;
                            }
                            let stored_events =
                                client.database().query(filters.clone(), Order::Desc).await;
                            match stored_events {
                                Ok(events) => {
                                    notes.set(events);
                                }
                                Err(e) => {
                                    // Rename the binding from Err to e
                                    notes.set(vec![]);
                                }
                            }
                            handle_fetch();
                        }
                    }
                    Err(e) => {
                        tracing::error!("Error: {:?}", e);
                    }
                };
            });
        },
    ));

    let on_mounted = move |_| {
        if name.clone().is_empty() {
            navigator().replace("/404");
        }
        // handle_fetch();
    };
    // spawn(async move {
    //     let subs_map_lock = subs_map();
    //     if subs_map_lock.contains_key(&name.clone()) {
    //         let sub_current = subs_map_lock.get(&namne.clone()).unwrap();
    //         let clients = multiclient();
    //         let hc = clients.get_or_create(&sub_current.relay_set).await;
    //         if sub_current.live {
    //             match hc {
    //                 Ok(client)=>{
    //                     let client = client.client();
    //                     sub_register.write().add_subscription(
    //                         &client,
    //                         SubscriptionId::new(format!("note-list-{}", sub_current.name)),
    //                         sub_current.get_filters(),
    //                         handle_sub_list(),
    //                         None,
    //                     ).await.unwrap();
    //                     sub_register().handle_notifications(&client).await.unwrap();
    //                 },
    //                 Err(e)=>{
    //                     tracing::error!("Error: {:?}", e);
    //                 }
    //             }

    //         }
    //     }
    // });
    // use_effect(use_reactive(&reload_flag(), move |next_reload_flag| {
    //     tracing::info!("reload_flag: {:?}", next_reload_flag);
    //     handle_fetch();
    // }));
    rsx! {
            div {
                onmounted: on_mounted,
                class:"flex-box-left h-full",
                id: "note-list",
                onscroll: move |_| {
                    let callback = Closure::wrap(Box::new({
                        move || {
                            match get_scroll_info("note-list") {
                                Ok(scroll_info) => {
                                    let scroll_top = scroll_info.scroll_top;
                                    let scroll_height = scroll_info.scroll_height;
                                    let client_height = scroll_info.client_height;
                                    if scroll_height - scroll_top - client_height <= 100 {
                                        handle_fetch();
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

                    modal_manager.write().destory_all_modals_by_level(4);
                },
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
                        if is_loading() {
                            div {
                                class: "laoding-box",
                                dangerous_inner_html: "{LOADING}"
                            }
                        }
                    }
                }
            }
    }
}
