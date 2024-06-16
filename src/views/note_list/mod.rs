pub(crate) mod custom_sub;
pub mod detail_modal;
pub mod new_note_msg;
pub mod note;
pub mod reply;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::components::icons::LOADING;
use crate::init::MODAL_MANAGER;
use crate::init::SUB_COUNTERS;
use crate::nostr::EventPaginator;
use crate::nostr::MultiClient;
use crate::nostr::{NotificationHandler, Register};
use crate::store::subscription::CustomSub;
use crate::utils::js::{get_scroll_info, throttle};
use dioxus::prelude::*;
use new_note_msg::NewNoteMsg;
use nostr_indexeddb::database::Order;
use nostr_sdk::{Event, RelayMessage, RelayPoolNotification, SubscriptionId, Timestamp};
use note::Note;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};

#[derive(Debug, Clone, Props, PartialEq)]
pub struct NoteListProps {
    pub name: String,
    pub reload_time: Timestamp,
    #[props(default = true)]
    pub is_cache: bool,
}

pub fn handle_sub_list(sub_id: Arc<RwLock<SubscriptionId>>) -> NotificationHandler {
    Arc::new(move |notification| {
        let sub_id = sub_id.read().unwrap().clone();
        Box::pin(async move {
            let sub_id = sub_id.clone(); 
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
                    {
                        SUB_COUNTERS.write().inc(&sub_id, *event.clone());
                        let id = "sub-new-msg".to_string();
                        MODAL_MANAGER.write().add_message(
                            rsx! {
                                NewNoteMsg {
                                    sub_id: sub_id.clone()
                                }
                            },
                            id.clone(),
                        );
                        MODAL_MANAGER.write().open_modal(&id);
                    }
                    Ok(false)
                }
                _ => {
                    tracing::info!("notification: {:?}", notification);
                    Ok(false)
                }
            }
        })
    })
}
#[component]
pub fn NoteList(props: NoteListProps) -> Element {
    let NoteListProps {
        name,
        reload_time,
        is_cache,
    } = props;
    let mut sub_current = use_signal(CustomSub::empty);
    let mut sub_name = use_signal(|| name.clone());
    let mut iscache = use_signal(|| is_cache.clone());
    // let mut sub_index = use_signal(|| 0);
    let mut notes: Signal<Vec<Event>> = use_signal(|| Vec::new());
    let mut paginator = use_signal(|| None);
    let mut is_loading = use_signal(|| false);
    let mut sub_register = use_context::<Signal<Register>>();
    let subs_map = use_context::<Signal<HashMap<String, CustomSub>>>();
    let multiclient = use_context::<Signal<MultiClient>>();
    let handle_fetch = move |is_clear: Option<bool>| {
        spawn(async move {
            if !is_loading() {
                is_loading.set(true);
                // if let Some(paginator_lock) = paginator() {
                let mut paginator_write: Write<Option<EventPaginator>, UnsyncStorage> =
                    paginator.write();
                let result = paginator_write.as_mut();
                if let Some(paginator) = result {
                    if let Some(events) = paginator.next_page().await {
                        tracing::info!("hello handle fetch {:?}", notes.len());
                        if let Some(clear) = is_clear {
                            if clear {
                                notes.clear();
                            }
                        }
                        notes.extend(events.iter().cloned());
                        is_loading.set(false);
                    } else {
                        is_loading.set(false);
                    }
                }
            }
        });
    };
    let handle_init = move || {
        let handler = handle_sub_list.clone();
        spawn(async move {
            let sub_current = sub_current.read().clone();
            let filters = sub_current.get_filters();
            let clients = multiclient();
            let client_result = clients.get_or_create(&sub_current.relay_set).await;

            match client_result {
                Ok(hc) => {
                    let client = hc.client();
                    {
                        is_loading.set(false);
                        tracing::info!("hello handle init");
                        let sub_id = SubscriptionId::new(format!("note-list-{}", sub_current.name));

                        if sub_current.live {
                            tracing::info!("sub_id: {:?}", sub_id.clone());
                            sub_register
                                .write()
                                .add_subscription(
                                    &client,
                                    sub_id.clone(),
                                    filters.clone(),
                                    handler(Arc::new(RwLock::new(sub_id.clone()))),
                                    None,
                                )
                                .await
                                .unwrap();
                            spawn({
                                let client = client.clone();
                                async move {
                                    if !sub_register().get_sub_flag(&sub_id).await {
                                        sub_register().handle_notifications(&client).await.unwrap();
                                    }
                                }
                            });
                        } else {
                            sub_register().remove_subscription(&sub_id).await;
                        }
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
                    }
                    {
                        if !iscache() && !sub_current.live {
                            handle_fetch(Some(true));
                            return;
                        }

                        if !sub_current.live {
                            let stored_events =
                                client.database().query(filters.clone(), Order::Desc).await;
                            match stored_events {
                                Ok(events) => {
                                    notes.set(events);
                                }
                                Err(_) => {
                                    notes.set(vec![]);
                                }
                            }
                        }
                        handle_fetch(Some(true));
                    }
                }
                Err(e) => {
                    tracing::error!("Error: {:?}", e);
                }
            };
        })
    };
    use_effect(use_reactive(
        (&name, &reload_time, &is_cache),
        move |(s, _, new_is_cache)| {
            notes.clear();
            is_loading.set(true);
            iscache.set(new_is_cache);
            sub_name.set(s.clone());
            let subs_map_lock = subs_map();
            if subs_map_lock.contains_key(&s) {
                let current = subs_map_lock.get(&s).unwrap();
                sub_current.set(current.clone());
            }
            handle_init();
        },
    ));

    let on_mounted = move |_| {
        if name.clone().is_empty() {
            navigator().replace("/404");
        }
        notes.clear();
    };
    use_effect(use_reactive((&SUB_COUNTERS.signal(),), move |(mut counter,)| {
        let sub_id = SubscriptionId::new(format!("note-list-{}", &sub_name()));
        let count = counter
            .read()
            .get_size(&sub_id);
        let events = counter.read().get_event(&sub_id).unwrap_or(vec![]);
        let sub_current = sub_current.read().clone();
        match count {
            Some(c) => {
                let modal_id = MODAL_MANAGER.read().has_modal(&"sub-new-msg".to_string());
                if sub_current.live && c <= 0 && modal_id && !events.is_empty() {

                    for event in events.iter() {
                        notes.insert(0, event.clone());
                    }
                    counter.write().clear(&sub_id);
                }
            }
            None => {}
        };
    }));
    rsx! {
            div {
                key: "note-list-{sub_name()}",
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
                                        handle_fetch(Some(false));
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

                    MODAL_MANAGER.write().destory_all_modals_by_level(4);
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
                                // key: note.id.to_string(),
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
