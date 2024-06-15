use dioxus::prelude::*;
use dioxus_elements::sub;
use nostr_sdk::Timestamp;
use crate::init::NEW_CUSTOM_SUB_KEY;
use crate::views::note_list::custom_sub::CustomSubscription;
use crate::views::NoteList;

#[component]
pub fn Subscription(name: String) -> Element {
    tracing::error!("loading sub_name: {:?}", name);
    // let
    let mut relaod_flag = use_signal(Timestamp::now);
    let mut sub_name = use_signal(|| name.clone());
    let mut is_cache = use_signal(|| true);
    let mut is_new_sub = use_signal(|| name.eq("new"));
    let _name = name.clone();

    let handle_save = move |_| {
        sub_name.set(name.clone());
        //todo
        relaod_flag.set(Timestamp::now());
        
    };
    let handle_reload = move |_| {
        relaod_flag.set(Timestamp::now());
        is_cache.set(false);
    };


    let mut _sub_name = sub_name.clone();
    use_effect(use_reactive(
        (&_name,&_sub_name),
        move |(_name,_sub_name)| {
            tracing::error!("loading sub_name1: {:?}", is_new_sub);
            if !_name.eq(&_sub_name()) {
                tracing::error!("loading sub_name3: {:?}", _name);
                let mut _sub_name = _sub_name.clone();
                _sub_name.set(_name.clone());
                is_new_sub.set(_sub_name().eq(NEW_CUSTOM_SUB_KEY));
            }
        },
    ));

    rsx! {
        section {
            id: "{sub_name()}",
            class: "subscription h-full flex-box",

            if !is_new_sub() {
                NoteList{
                    name: sub_name(),
                    reload_time: relaod_flag(),
                    is_cache: is_cache(),
                }
            }

            if is_new_sub(){
                div {
                    "New Custmo Subscription"
                }
            }
            
            
            
            CustomSubscription {
                on_save: handle_save,
                on_reload: handle_reload,
                sub_name: sub_name,
                is_add: is_new_sub()
            }
        }
    }
}
