use dioxus::prelude::*;
use std::collections::HashMap;
use crate::views::NoteList;
use crate::components::Notification;
use nostr_sdk::{Timestamp,PublicKey};
use crate::nostr::multiclient::MultiClient;
use crate::store::subscription::{CustomAccounts, CustomSub, FilterTemp,Account};
use crate::nostr::get_following;
#[component]
pub fn Home() -> Element {
    //init
    let relaod_flag: Signal<Timestamp> = use_signal(Timestamp::now);
    let subs_map: Signal<HashMap<String, CustomSub>> = use_context::<Signal<HashMap<String, CustomSub>>>();
    let all_sub: Signal<Vec<CustomSub>> = use_context::<Signal<Vec<CustomSub>>>();
    // let mut sub_name_nostr: Signal<String> = use_signal(|| "nostr".to_string());
    let is_cache: Signal<bool> = use_signal(|| true);
    let multiclient = use_context::<Signal<MultiClient>>();
    let hex_str = "5ee693398c21a9ab2cfb2bea3f1f9bbe6eeb8501c053db67f7a3e83a332a6ab0";
    let public_key: PublicKey = PublicKey::from_hex(hex_str).expect("publicKey");
    let sub_name = String::from("follwoing");
    let relay_name = String::from("default");
    let is_loaded = use_signal(|| false);
    //loading following users
    use_effect(use_reactive(
        (&public_key,&relay_name,&sub_name),
        move |(public_key,relay_name,sub_name)| {
            let mut subs_map = subs_map.clone();
            let mut all_sub = all_sub.clone();
            let multiclient = multiclient.clone();
            let mut is_loaded = is_loaded.clone();
            spawn(async move {
                let cloned_public_key = public_key.clone();
                let cloned_relay_name = relay_name.clone();
                let cloned_sub_name = sub_name.clone();
                let clients: MultiClient = multiclient();
                let client_result: Result<crate::nostr::multiclient::HashedClient, crate::nostr::multiclient::Error> = clients.get_or_create(&cloned_relay_name).await;
                match client_result {
                    Ok(hc) => {
                        let client: std::sync::Arc<nostr_sdk::Client> = hc.client();
                        match get_following(&client, &cloned_public_key, None).await {
                            Ok(following_users) => {
                                tracing::info!("getfollowing result: {:?}", following_users);
                                let accounts: Vec<Account> = following_users.iter().map(|user: &String| Account {
                                  alt_name: String::from(user),
                                  npub: String::from(user),
                                }).collect();

                                let custom_sub: CustomSub = CustomSub {
                                  name: cloned_sub_name,
                                  relay_set: cloned_relay_name,
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
                                  let mut subs_map_write = subs_map.write();
                                  subs_map_write.insert(sub_name.clone(), custom_sub.clone());
                                  is_loaded.set(true);
                                  // sub_name_nostr.set(sub_name);

                                }

                                {
                                  // let mut all_sub_write: Write<_, UnsyncStorage> = all_sub.write();
                                  // sub_name_nostr.set(sub_name);
                                }
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
        div{
          class:"flex-box-left",
          if *is_loaded.read() {
            NoteList {
                name: sub_name,
                reload_time: relaod_flag(),
                is_cache: is_cache(),
            }
          } else {
              div { "Loading..." }
          }
        }
        div{
          Notification{}
          // Author{}
        }
      }

    }
}
