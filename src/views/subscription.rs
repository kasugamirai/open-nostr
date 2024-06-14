use dioxus::prelude::*;
use dioxus_elements::sub;
use nostr_sdk::Timestamp;

use crate::views::note_list::custom_sub::CustomSubscription;
use crate::views::NoteList;

#[component]
pub fn Subscription(name: String) -> Element {
    // let
    let mut relaod_flag = use_signal(Timestamp::now);
    let mut sub_name = use_signal(|| name.clone());
    let mut is_cache = use_signal(|| true);

    let handle_save = move |_| {
        sub_name.set(name.clone());
        //todo
        relaod_flag.set(Timestamp::now());
        
    };
    let handle_reload = move |_| {
        relaod_flag.set(Timestamp::now());
        is_cache.set(false);
    };
    rsx! {
        section {
            class: "subscription h-full flex-box",
            NoteList{
                name: sub_name(),
                reload_time: relaod_flag(),
                is_cache: is_cache(),
            }
            CustomSubscription {
                on_save: handle_save,
                on_reload: handle_reload,
                sub_name: sub_name,
            }
        }
    }
}
