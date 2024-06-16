use dioxus::prelude::*;
use std::collections::HashMap;
use crate::store::subscription::CustomSub;
use crate::components::RelaysManage;
use crate::components::icons::LOADING;
use crate::init::SUB_SYSTEM_FILERS;
fn filter_map_key(map: HashMap<String,CustomSub>, keywords: &[&str],is_contains: bool) -> Vec<String> {
  map.into_iter()
      .filter(|(key, _)| {
        let contains = keywords.iter().any(|&kw| key.eq(kw));
        if is_contains {
            contains
        } else {
            !contains
        }
      })
      .map(|(key, _)| key.to_string())
      .collect()
}

#[component]
pub fn Relay() -> Element {

    let subs_map: Signal<HashMap<String, CustomSub>> = use_context::<Signal<HashMap<String, CustomSub>>>();
    let mut system_subs = use_signal(|| Vec::new());
    let mut user_subs = use_signal(|| Vec::new());
    let mut sub_name_current = use_signal(|| "nostr".to_string());
    let mut is_loaded = use_signal(||true);

    use_effect(move || {
        spawn(async move {
          if system_subs().len()<=0 {
            system_subs.set(filter_map_key(subs_map().clone(),&SUB_SYSTEM_FILERS,true));
          }
          
          if user_subs().len()<=0 {
            user_subs.set(filter_map_key(subs_map().clone(),&SUB_SYSTEM_FILERS,false));
          }

          if is_loaded() {
            is_loaded.set(false);
          }
        });
    });


    rsx! {
       div{
        class:"relay-contnet",
        div{
          class:"built-in-function text-center font-size-16",
          "built-in function"

          for (index, sub_key) in system_subs.read().iter().enumerate(){
            
            div{
              class: format!("built-li radius-26 text-center mb-28 font-size-14 line-height-28 text-overflow {}", 
              if sub_key == &sub_name_current() { "built-li-checked" } else { "" }),
              onclick: move |_| {
                match system_subs.read().get(index) {
                  Some(_sub_key) => sub_name_current.set(_sub_key.clone()),
                  None =>{}
                }
              },
              "{sub_key}"
            }
          }
          
          div{
            class:"separate mt-20"
          }

          "Supscription"
          div{
            class:"separate mb-12"
          }

          for (index, sub_key) in user_subs.read().iter().enumerate(){
            div{
              class: format!("built-li radius-26 text-center mb-28 font-size-14 line-height-28 text-overflow {}", 
              if sub_key == &sub_name_current() { "built-li-checked" } else { "" }),
              onclick: move |_| {
                match user_subs.read().get(index) {
                  Some(_sub_key) => sub_name_current.set(_sub_key.clone()),
                  None =>{}
                }
              },
              "#{sub_key}"
            }
          }
        }
        div{
          class:"set-content ml-78 px-18 py-18 radius-26",
          if !is_loaded() {
            RelaysManage{
              key: "{sub_name_current()}",
              sub_name: sub_name_current()
            }
          } 
          
          if is_loaded() {
            div {
                class: "laoding-box",
                dangerous_inner_html: "{LOADING}"
            }
          }
          // RelaysInput {
          //   on_change: move |v: RelaySet| {
          //       // let mut sub = sub_current.write();
          //       // sub.relay_set = v.name.clone();
          //   },
          //   relay_name: "default",
          //   is_popup:false
          // }
        }
       }
    }
}