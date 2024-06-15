use dioxus::prelude::*;
use std::collections::HashMap;
use crate::views::NoteList;
use crate::components::Notification;
use nostr_sdk::{Timestamp,PublicKey};
use crate::nostr::MultiClient;
use crate::store::subscription::{CustomAccounts, CustomSub, FilterTemp,Account};
use crate::store::DEFAULT_RELAY_SET_KEY;
use crate::init::FOLLOWING_SUB_KEY;
use crate::nostr::get_following;
#[component]
pub fn Home() -> Element {
    //global component
    let subs_map = use_context::<Signal<HashMap<String, CustomSub>>>();
    let multiclient = use_context::<Signal<MultiClient>>();
    //default parameters
    let hex_str = "5ee693398c21a9ab2cfb2bea3f1f9bbe6eeb8501c053db67f7a3e83a332a6ab0";
    let public_key = PublicKey::from_hex(hex_str).expect("publicKey");
    let sub_name = String::from(FOLLOWING_SUB_KEY.to_string());
    let relay_name = String::from(DEFAULT_RELAY_SET_KEY.to_string());
    let mut is_loaded = use_signal(|| false);

    //loading following users
    use_effect(use_reactive(
        (&public_key,&relay_name,&sub_name,&subs_map),
        move |(public_key,relay_name,sub_name,subs_map)| {
            spawn(async move {
                let clients = multiclient();
                let client_result = clients.get_or_create(&relay_name).await;
                match client_result {
                    Ok(hc) => {
                        let client = hc.client();
                        match get_following(&client, &public_key, None).await {
                            Ok(following_users) => {
                                //format users Vec<String> ->  Account
                                let accounts = following_users.iter().map(|user: &String| Account {
                                  alt_name: String::from(user[0..5].to_string()),
                                  npub: String::from(user),
                                }).collect();
                                //init sub
                                let following_sub = CustomSub {
                                  name: sub_name.clone(),
                                  relay_set: relay_name.clone(),
                                  live: false,
                                  since: 0,
                                  until: 0,
                                  filters: vec![FilterTemp::Accounts(CustomAccounts {
                                      r#type: String::from("accounts"),
                                      kinds: vec![1],
                                      accounts: accounts,
                                  })],
                                  keep_alive: true,
                                };
                                {
                                  let mut _subs_map = subs_map.clone();
                                  _subs_map.write().insert(sub_name.clone(), following_sub.clone());  
                                }
                                is_loaded.set(true);
                            } 
                            Err(e) => {
                                tracing::error!("get following error: {:?}", e);
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("following client Error: {:?}", e);
                    }
                }
            });
        },
    ));

    

    rsx! {
      div{
        class:"flex-box",
        if *is_loaded.read() {
          div{
            class:"flex-box-left",
              NoteList {
                  name: sub_name.clone(),
                  reload_time: Timestamp::now(),
              }
          }
          div{
            Notification{
              public_key: public_key.clone(),
              relay_name: relay_name.clone(),
            }
            // Author{}
          }
        } else {
          div { "Loading..." }
        }
      }

    }
}
