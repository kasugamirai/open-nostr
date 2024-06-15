use dioxus::prelude::*;
use dioxus_elements::sub;
use nostr_sdk::Timestamp;
use crate::init::NEW_CUSTOM_SUB_KEY;
use crate::views::note_list::custom_sub::CustomSubscription;
use crate::views::NoteList;

#[component]
pub fn Subscription(name: String) -> Element {
    // let
    let mut relaod_flag = use_signal(Timestamp::now);
    let mut sub_name = use_signal(|| name.clone());
    let mut is_cache = use_signal(|| true);

    let name_clone = name.clone();
    let handle_save = move |_| {
        sub_name.set(name_clone.clone());
        //todo
        relaod_flag.set(Timestamp::now());
    };
    let handle_reload = move |_| {
        relaod_flag.set(Timestamp::now());
        is_cache.set(false);
    };

    rsx! {
        section {
            id: "{sub_name()}",
            class: "subscription h-full flex-box",

            if !name.clone().eq(NEW_CUSTOM_SUB_KEY) {
                NoteList{
                    name: name.clone(),
                    reload_time: relaod_flag(),
                    is_cache: is_cache(),
                }
            } else {
                div {
                    "New Custmo Subscription"
                }
            }
            
            
            
            CustomSubscription {
                on_save: handle_save,
                on_reload: handle_reload,
                sub_name: name.clone(),
                is_add: name.clone().eq(NEW_CUSTOM_SUB_KEY)
            }
        }
    }
}
