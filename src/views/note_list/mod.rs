pub(crate) mod custom_sub;
pub mod note;
pub mod reply;

use std::collections::HashMap;

use dioxus::prelude::*;
use nostr_indexeddb::database::Order;
use nostr_sdk::{Event, Timestamp};
use wasm_bindgen::{closure::Closure, JsCast, JsValue};

use crate::{
    components::icons::LOADING,
    nostr::{
        fetch::EventPaginator,
        multiclient::MultiClient,
    },
    store::subscription::CustomSub,
    utils::js::{get_scroll_info, throttle},
};

use note::Note;

#[derive(Debug, Clone, Props, PartialEq)]
pub struct NoteListProps {
    pub name: String,
    pub reload_time: Timestamp,
    #[props(default=true)]
    pub is_cache: bool,
}


#[component]
pub fn NoteList(props: NoteListProps) -> Element {
    let NoteListProps {name, reload_time, is_cache} = props;
    let mut reload_flag = use_signal(|| reload_time.clone());
    let subs_map = use_context::<Signal<HashMap<String, CustomSub>>>();
    let mut sub_current = use_signal(|| CustomSub::empty());
    // let mut sub_index = use_signal(|| 0);
    let mut notes: Signal<Vec<Event>> = use_signal(|| Vec::new());
    let mut paginator = use_signal(|| None);
    let multiclient = use_context::<Signal<MultiClient>>();
    let mut is_loading = use_signal(|| false);


    let handle_fetch = move || {
        spawn(async move {
            if !is_loading() {
                is_loading.set(true);
                // if let Some(paginator_lock) = paginator() {
                let mut paginator_write: Write<Option<EventPaginator>, UnsyncStorage> =
                    paginator.write();
                let result = paginator_write.as_mut();
                if let Some(paginator) = result {
                    let events = paginator.next_page().await;
                    match events {
                        Ok(events) => {
                            notes.extend(events.iter().cloned());
                            is_loading.set(false);
                        }
                        Err(e) => {
                            tracing::error!("Error: {:?}", e);
                            is_loading.set(false);
                        }
                    }
                }
            }
        });
    };

    use_effect(use_reactive((&name, &reload_time, &is_cache), move |(s, time, iscache)| {
        let subs_map_lock = subs_map();
        if subs_map_lock.contains_key(&s) {
            let current = subs_map_lock.get(&s).unwrap();
            sub_current.set(current.clone());
        }

        spawn(async move {
            let sub_current = sub_current.read().clone();
            let filters = sub_current.get_filters();
            let mut clients = multiclient();
            let client_result = clients.get_or_create(&sub_current.relay_set).await;
            match client_result {
                Ok(hc) => {
                    let client = hc.client();
                    {
                        let paginator_result = EventPaginator::new(client.clone(), filters.clone(), None, 40);
                        paginator.set(Some(paginator_result));
                        // notes.clear();
                        reload_flag.set(time.clone());
                    }
                    {
                        tracing::info!("is_cache: {:?}", iscache);
                        if !iscache {
                            notes.set(vec![]);
                            handle_fetch();
                            return;
                        }
                        let stored_events = client.database().query(filters.clone(), Order::Desc).await;
                        match stored_events {
                            Ok(events) => {
                                notes.set(events);
                            }
                            Err(e) => { // Rename the binding from Err to e
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
    }));
    let on_mounted = move |_| {
        if name.is_empty() {
            return;
        } else {
            let subs_map_lock = subs_map();
            if !subs_map_lock.contains_key(&name) {
                navigator().replace("/404");
            }
        }
        // handle_fetch();
    };

    use_effect(use_reactive(&reload_flag(), move |next_reload_flag| {
        handle_fetch();
    }));

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
