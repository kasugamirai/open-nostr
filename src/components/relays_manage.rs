use crate::components::icons::{ADD, BOTTOMRIGHT, FALSE, LOADING, UPPERRIGHT};
use crate::store::subscription::CustomSub;
use crate::store::subscription::RelaySet;
use crate::store::{CBWebDatabase, DEFAULT_RELAY_SET_KEY};
use crate::utils::contants::WSS_REG;
use crate::utils::js::alert;
use dioxus::prelude::*;
use regex::Regex;
use std::collections::HashMap;

#[component]
pub fn RelaysManage(sub_name: String) -> Element {
    // tracing::info!("system_subs: {:?}", sub_name);
    let mut cb_database_db = use_context::<Signal<CBWebDatabase>>();
    let mut subs_map = use_context::<Signal<HashMap<String, CustomSub>>>();
    let mut relay_sets = use_signal(|| Vec::<RelaySet>::new());
    let mut relay_current_index = use_signal(|| 0);
    let current_relay_set = use_memo(move || match relay_sets.read().get(relay_current_index()) {
        Some(relay) => relay.clone(),
        None => RelaySet::new(&relay_sets.read().len()),
    });

    let sub_name_current = use_signal(|| sub_name);
    let mut new_relay = use_signal(String::new);
    let mut is_save_loaded = use_signal(|| false);
    let mut old_relay_sets: Signal<Vec<RelaySet>> = use_signal(Vec::new);

    use_effect(use_reactive(&relay_sets, move |relay_sets| {
        // tracing::info!("system_subs: {:?}", sub_name_current());
        let mut _relay_sets: Signal<Vec<RelaySet>> = relay_sets.clone();
        let _cb_database_db = cb_database_db.read().clone();
        spawn(async move {
            let sub_name = sub_name_current.read().clone();
            match _cb_database_db.get_all_relay_sets().await {
                Ok(relay_set_all) => {
                    // Update the relay sets state
                    _relay_sets.set(relay_set_all.clone());
                    old_relay_sets.set(relay_set_all.clone());

                    tracing::info!("sub_name: {:?}", &sub_name);
                    match subs_map().get(&sub_name) {
                        Some(sub) => {
                            let relay_name = sub.relay_set.clone();
                            for (i, relay_set) in relay_set_all.iter().enumerate() {
                                if relay_set.name == relay_name {
                                    relay_current_index.set(i);
                                    break;
                                }
                            }
                        }
                        None => {}
                    };
                }
                Err(e) => {
                    eprintln!("Failed to get relay sets: {:?}", e);
                }
            }
        });
    }));

    //verify relay-set wss url
    let relay_url_verify = move |realy_url: &String, check_val_exists: bool| -> bool {
        let realy_url = realy_url.clone();
        let _current_relay_set = current_relay_set();
        let wss_regx: Regex = Regex::new(WSS_REG).unwrap();
        if !wss_regx.is_match(&realy_url) {
            spawn(async move {
                alert("Invalid URL".to_string()).await;
            });
            return false;
        }

        if _current_relay_set
            .relays
            .iter()
            .any(|x| x == &realy_url.clone() && check_val_exists)
        {
            spawn(async move {
                alert("Relay already exists".to_string()).await;
            });
            return false;
        }

        return true;
    };

    let handle_save = move || {
        let relay_url_verify = relay_url_verify.clone();
        let mut _relay_current = current_relay_set().clone();
        let _relay_index = relay_current_index.clone();
        let _old_relay_sets = old_relay_sets.read().clone();
        let realy_name = _relay_current.name.clone();
        let mut _new_relay = new_relay.clone();

        //verify save data
        if _relay_current.relays.is_empty() && _new_relay().is_empty() {
            spawn(async move {
                alert("Relay-set wss:// cannot be empty.".to_string()).await;
            });
            return;
        }

        for (i, realy) in _old_relay_sets.iter().enumerate() {
            if realy.name == realy_name && i != _relay_index() {
                spawn(async move {
                    alert("Relay-set name already exists.".to_string()).await;
                });
                return;
            }
        }

        if !_new_relay().is_empty() {
            let verif = relay_url_verify.clone();
            if !verif(&_new_relay().clone(), true) {
                return;
            } else {
                _relay_current.relays.push(_new_relay().clone());
                _new_relay.set(String::new());
            }
        }

        for val in _relay_current.relays.iter() {
            let verif = relay_url_verify.clone();
            if !verif(&val.clone(), false) {
                return;
            }
        }

        let _relay_current: RelaySet = _relay_current.clone();
        let mut _relay_index = _relay_index.clone();
        let _sub_name = sub_name_current().clone();
        spawn(async move {
            is_save_loaded.set(true);
            let cb_database_db_write = cb_database_db.write();

            //check action type
            let mut is_add = true;
            let mut is_edit = false;
            for (i, realy) in _old_relay_sets.iter().enumerate() {
                if realy.name == realy_name {
                    if i == _relay_index() {
                        is_edit = true;
                        is_add = false;
                        break;
                    }
                }
            }

            //save relay  to db
            if is_add {
                cb_database_db_write
                    .save_relay_set(_relay_current.clone())
                    .await
                    .unwrap();
            }

            //edit relay  to db
            if is_edit {
                cb_database_db_write
                    .relay_set_change(realy_name.clone(), _relay_current.clone())
                    .await
                    .unwrap();
            }

            //update sub map and db
            tracing::info!(
                "relays_manage update sub map and db sub_name:{:?}",
                _sub_name
            );
            match subs_map().get(&_sub_name) {
                Some(sub) => {
                    let mut sub_new = sub.clone();
                    sub_new.relay_set = _relay_current.name.clone();
                    let mut subs_map_write = subs_map.write();
                    match cb_database_db_write
                        .update_custom_sub(sub_new.name.clone(), sub_new.clone())
                        .await
                    {
                        Ok(_) => {
                            subs_map_write.insert(sub_new.name.clone(), sub_new.clone());
                        }
                        Err(e) => {
                            tracing::error!("relays_manage update sub to db error:{:?}", e);
                        }
                    }
                }
                None => {
                    tracing::error!("relays_manage update sub subs_map get null");
                }
            };

            //save and refush relay data
            let updated_relay_sets: Vec<RelaySet> =
                cb_database_db_write.get_all_relay_sets().await.unwrap();
            let mut index = 0;
            for (i, realy) in updated_relay_sets.iter().enumerate() {
                if realy.name == realy_name {
                    index = i;
                    break;
                }
            }
            relay_sets.set(updated_relay_sets.clone());
            _relay_index.set(index);
            is_save_loaded.set(false);
        });
    };

    let handle_delete = move || {
        let mut _relay_current = current_relay_set().clone();
        let mut _subs_map = subs_map.clone();
        let mut _relay_index = relay_current_index.clone();
        let mut cb_database_db = cb_database_db.clone();

        //remove relay-set
        spawn(async move {
            is_save_loaded.set(true);
            let cb_database_db_write = cb_database_db.write();
            cb_database_db_write
                .remove_relay_set(_relay_current.name.clone())
                .await
                .unwrap();

            // update sub relay-set to default
            let mut new_map = HashMap::new();
            for (key, sub) in _subs_map() {
                if sub.relay_set == _relay_current.name {
                    let mut sub_new = sub.clone();
                    sub_new.relay_set = DEFAULT_RELAY_SET_KEY.to_string().clone();
                    match cb_database_db_write
                        .update_custom_sub(sub_new.name.clone(), sub_new.clone())
                        .await
                    {
                        Ok(_) => {
                            // let mut subs_map_write =_subs_map.write();
                            new_map.insert(sub_new.name.clone(), sub_new.clone());
                        }
                        Err(e) => {
                            tracing::error!(
                                "relays_manage remove relay set update sub to db error:{:?}",
                                e
                            );
                        }
                    }
                } else {
                    new_map.insert(key.clone(), sub.clone());
                }
            }
            _subs_map.set(new_map);

            // remove refush relay data
            relay_sets.remove(_relay_index());
            _relay_index.set(0);
            is_save_loaded.set(false);
        })
    };

    rsx! {
       div{
        class:"relay-contnet",
        div{
          class:"built-in-function text-center font-size-16",
          for (index, relay) in relay_sets.read().iter().enumerate(){

            div{
              class: format!("built-li radius-26 text-center mb-28 font-size-14 line-height-28 text-overflow {}",
              if index == relay_current_index() { "built-li-checked" } else { "" }),
              onclick: move |_| {
                new_relay.set(String::new());
                match relay_sets.read().get(index) {
                  Some(_relay) => relay_current_index.set(index),
                  None =>{}
                }
              },
              "{relay.name}"
            }
          }

          div{
            class:"built-li radius-26 text-center mb-28 font-size-14 line-height-28 text-overflow",
            onclick: move |_| {
              let mut _relay_sets = relay_sets.write();
              let new_relay_set = RelaySet::new(&_relay_sets.len());
              _relay_sets.push(new_relay_set);
              relay_current_index.set(&_relay_sets.len()-1);
              // tracing::info!("Add new relay set");
            },
            "New realy set",

          }
        }

        div{
          class:"relay-urls ml-41",
          if !is_save_loaded() {
            div {
                class:"relay-actions-bar flex mb-15",
                input {
                    class:"relay-name-ipt",
                    r#type: "text",
                    disabled: (current_relay_set().name == DEFAULT_RELAY_SET_KEY).to_string(),
                    value: current_relay_set().name,
                    oninput: move |event| {
                        let mut _relay_sets = relay_sets.write();
                        _relay_sets[relay_current_index()].name.clone_from(&event.value());
                    }
                }
                button {
                  class: "btn-circle btn-circle-success flex-right ml-5",
                  onclick: move |_| {
                  },
                  div {
                      dangerous_inner_html: "{BOTTOMRIGHT}"
                  }
                }
                button {
                    class: "btn-circle btn-circle-success flex-right ml-5",
                    onclick: move |_| {
                    },
                    div {
                      dangerous_inner_html: "{UPPERRIGHT}"
                    }
                }
            }
            for (i, relay_url) in current_relay_set().relays.iter().enumerate() {
                div {
                    class:"relay-url-item mb-10 flex items-center",
                    input {
                        class: "relay-ipt mr-10",
                        r#type: "text",
                        value: "{relay_url}",
                        placeholder: "wss://",
                        oninput: move |event| {
                            let mut _relay_sets = relay_sets.write();
                            _relay_sets[relay_current_index()].relays[i].clone_from(&event.value());
                        }
                    }
                    if relay_sets()[relay_current_index()].relays.len()>1 {
                      button {
                        class: "btn-circle btn-circle-false relay-url-del",
                        onclick: move |_| {
                            let mut _relay_sets = relay_sets.write();
                            _relay_sets[relay_current_index()].relays.remove(i);
                        },
                        div {
                            dangerous_inner_html: "{FALSE}"
                        }
                      }
                    }
                }
            }
            div {
                class:"relay-url-item flex items-center",
                input {
                    class: "relay-ipt mr-10",
                    r#type: "text",
                    placeholder: "wss://",
                    value: "{new_relay()}",
                    oninput: move |event| {
                        // tracing::info!("new_relay: {:?}", event.value());
                        new_relay.set(event.value());
                    }
                }
                button {
                    class: "btn-icon add relay-url-add flex-right",
                    onclick: move |_| {
                        let _new_relay = new_relay().clone();
                        if new_relay().is_empty() {
                        } else if !relay_url_verify(&_new_relay,true) {
                          new_relay.set(String::new());
                          return;
                        } else {
                            relay_sets.write()[relay_current_index()].relays.push(new_relay());
                            new_relay.set(String::new());
                        }
                        //  current_relay_set.relays.push(new_relay.clone());
                    },
                    div {
                        dangerous_inner_html: "{ADD}"
                    }
                }
            }

            div{
              class: "relay-contnet",
              button {
                class: "btn-circle-true built-li radius-26 text-center font-size-14 mr-8",
                onclick: move |_| {
                  handle_save();
                },
                div {
                  "Save"
                }
              }

              if &current_relay_set().name != DEFAULT_RELAY_SET_KEY {
                button {
                  class: "btn-circle-false built-li radius-26 text-center font-size-14",
                  onclick: move |_| {
                    handle_delete();
                  },
                  div {
                    "Delete"
                  }
                }
              }

            }
          }
          if is_save_loaded() {
            div {
                class: "laoding-box",
                dangerous_inner_html: "{LOADING}"
            }
          }
        }
      }
    }
}
