use dioxus::prelude::*;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    usize,
};

use crate::{
    components::icons::{ADD, FALSE, TRUE},
    store::{subscription::RelaySet, CBWebDatabase, DEFAULT_RELAY_SET_KEY},
    utils::{contants::WSS_REG, js::alert},
};

#[derive(PartialEq, Clone, Props)]
pub struct RelaysInputProps {
    on_change: EventHandler<RelaySet>,
    relay_name: String,
    #[props(default = false)]
    edit: bool,
    #[props(default = 0)]
    index: usize,
}
#[derive(Clone, Debug)]
struct ModifiedRelaySet {
    old_name: String,
    new_relay: RelaySet,
}

#[component]
pub fn RelaysInput(props: RelaysInputProps) -> Element {
    let mut relay_sets: Signal<Vec<RelaySet>> = use_signal(Vec::new);
    let mut new_relay = use_signal(String::new);
    let mut edit = use_signal(|| props.edit);
    let mut relay_curent_index: Signal<usize> = use_signal(|| 0);
    let mut old_relay_sets: Signal<Vec<RelaySet>> = use_signal(Vec::new);
    let allow_edit = use_context::<Signal<bool>>();
    let mut cb_database_db = use_context::<Signal<CBWebDatabase>>();
    // let relay_sets.read() = relay_sets.read();
    let current_relay_set: RelaySet = match relay_sets.read().get(relay_curent_index()) {
        Some(relay) => relay.clone(),
        None => RelaySet::new(&relay_sets.read().len()),
    };
    let wss_regx = Regex::new(WSS_REG).unwrap();
    // tracing::info!("index: {:?}", );
    use_effect(use_reactive(&props.relay_name, move |_relay_name| {
        spawn(async move {
            // Reading from the database
            let cb_database_db_write = cb_database_db.read();
            match cb_database_db_write.get_all_relay_sets().await {
                Ok(relay_sets_vec) => {
                    // Update the relay sets state
                    relay_sets.set(relay_sets_vec.clone());
                    old_relay_sets.set(relay_sets_vec.clone());

                    // Find the relay set by name
                    let index = relay_sets_vec
                        .iter()
                        .position(|x| x.name == _relay_name)
                        .unwrap_or(0);

                    tracing::info!("index: {:?}", index);
                    tracing::info!("relay_name(): {:?}", _relay_name);
                    relay_curent_index.set(index);
                }
                Err(e) => {
                    // Handle the error (e.g., log it)
                    eprintln!("Failed to get relay sets: {:?}", e);
                }
            }
        });
    }));
    let handle_save = move || {
        let duplicate_names = {
            let relay_sets = relay_sets.read();
            let mut names_set = HashSet::new();
            let mut duplicates = HashSet::new();
            for relay in relay_sets.iter() {
                if !names_set.insert(relay.name.to_string()) {
                    duplicates.insert(relay.name.to_string());
                }
            }
            duplicates
        };

        if !duplicate_names.is_empty() {
            spawn(async move {
                alert(format!("Duplicate names: {:?}", duplicate_names)).await;
            });
            return;
        }

        spawn(async move {
            let cb_database_db_write = cb_database_db.write();
            let current_relay_sets = relay_sets.read().clone();
            let previous_relay_sets = old_relay_sets.read().clone();

            let previous_map: HashMap<_, _> = previous_relay_sets
                .iter()
                .map(|relay| (relay.name.clone(), relay.clone()))
                .collect();

            let mut new_added = Vec::new();
            let mut modified = Vec::new();

            for relay in current_relay_sets.iter() {
                match previous_map.get(&relay.name) {
                    Some(prev_relay) => {
                        if *prev_relay != *relay {
                            modified.push(ModifiedRelaySet {
                                old_name: prev_relay.name.clone(),
                                new_relay: relay.clone(),
                            });
                        }
                    }
                    None => {
                        // Check if the relay was renamed
                        let renamed = previous_relay_sets
                            .iter()
                            .find(|prev_relay| prev_relay.relays == relay.relays);

                        if let Some(prev_relay) = renamed {
                            modified.push(ModifiedRelaySet {
                                old_name: prev_relay.name.clone(),
                                new_relay: relay.clone(),
                            });
                        } else {
                            new_added.push(relay.clone());
                        }
                    }
                }
            }

            let current_names: HashSet<_> = current_relay_sets
                .iter()
                .map(|relay| relay.name.clone())
                .collect();
            let deleted: Vec<_> = previous_relay_sets
                .iter()
                .filter(|relay| !current_names.contains(&relay.name))
                .cloned()
                .collect();

            tracing::info!("new_added: {:#?}", new_added);
            tracing::info!("modified: {:#?}", modified);
            tracing::info!("deleted: {:#?}", deleted);

            if new_added.len() > 0 || modified.len() > 0 || deleted.len() > 0 {
                let mut tips = String::new();
                for relay in new_added
                    .iter()
                    .chain(modified.iter().map(|m| &m.new_relay))
                {
                    if relay.relays.is_empty() {
                        tips.push_str(&format!(
                            "Relay set {} must have at least one relay\n",
                            relay.name
                        ));
                    }
                }

                if !tips.is_empty() {
                    alert(tips).await;
                    return;
                }

                for relay in new_added.iter() {
                    cb_database_db_write
                        .save_relay_set(relay.clone())
                        .await
                        .unwrap();
                }

                for relay in modified.iter() {
                    cb_database_db_write
                        .relay_set_change(relay.old_name.clone(), relay.new_relay.clone())
                        .await
                        .unwrap();
                }

                for relay in deleted.iter() {
                    cb_database_db_write
                        .remove_relay_set(relay.name.clone())
                        .await
                        .unwrap();
                }
                // Refresh relay sets signal after database operations
                let updated_relay_sets: Vec<RelaySet> =
                    cb_database_db_write.get_all_relay_sets().await.unwrap();
                relay_sets.set(updated_relay_sets.clone());
                old_relay_sets.set(updated_relay_sets);
            }

            props
                .on_change
                .call(relay_sets.read()[relay_curent_index()].clone());
            edit.set(false);
        });
    };

    rsx! {
        div {
            class: "relay-btn relative",
            div {
                class: format!("sub-input cursor-{}", if allow_edit() { "pointer" } else { "disabled" }),
                onclick: move |_| {
                  if allow_edit() { // 判断是否编辑状态
                    edit.set(!edit());
                  }
                },
                "{current_relay_set.name}"
            }
            div {
                class:"show-{edit} hierarchical relay-edit--modal",
                div{
                    class:"relay-edit-mask",
                    onclick: move |_| {
                        edit.set(false);
                    },
                }
                div {
                    class: "modal-content p-24 relay-edit--content z-100 relative radius-26 flex",
                    div{
                        class:"relay-name-list p-10",
                        div{
                            class: "relay-name-list--content overflow-y-auto",
                            for (i, relay) in relay_sets.read().iter().enumerate() {
                                div{
                                    class: format!("mb-8 cursor-pointer radius-15 relay-name-item w-full px-8 text-overflow {}", if i == relay_curent_index() { "relay-name-item--active" } else { "" }),
                                    onclick: move |_| {
                                      relay_curent_index.set(i);
                                    },
                                    "{relay.name}",
                                }
                            }
                        }
                        // Add new relay set
                        div {
                            class: "relay-name-add w-full radius-15 px-8 cursor-pointer",
                            onclick: move |_| {
                                let mut _relay_sets = relay_sets.write();
                                let new_relay_set = RelaySet::new(&_relay_sets.len());
                                _relay_sets.push(new_relay_set);
                            },
                            "new relay set"
                        }
                    }
                    div{
                        class:"relay-urls ml-41",
                        div {
                            class:"relay-actions-bar flex mb-15",
                            input {
                                class:"relay-name-ipt",
                                r#type: "text",
                                disabled: (current_relay_set.name == DEFAULT_RELAY_SET_KEY).to_string(),
                                value: current_relay_set.name,
                                oninput: move |event| {
                                    let mut _relay_sets = relay_sets.write();
                                    _relay_sets[relay_curent_index()].name = event.value().clone();
                                }
                            }
                            button {
                                class: "btn-circle btn-circle-true ml-24",
                                onclick: move |_| {
                                    handle_save();
                                },
                                dangerous_inner_html: "{TRUE}"
                            }
                            button {
                                class: "btn-circle btn-circle-false ml-12",
                                onclick: move |_| {
                                    // handle_save();
                                    // 1. todo save relays
                                    // 2.
                                    // bak.set(value());
                                    // props.on_change.call(relay_sets.read().get(relay_curent_index()).unwrap().clone());
                                    // edit.set(false);
                                },
                                dangerous_inner_html: "{FALSE}"
                            }
                            // button {
                            //     class: "btn-circle btn-circle-success",
                            //     onclick: move |_| {
                            //         // handle_export(value.read().relays.join(",\n"));
                            //     },
                            //     div {
                            //         dangerous_inner_html: "{RIGHT}"
                            //     }
                            // }
                            // button {
                            //     class: "btn-circle btn-circle-success",
                            //     onclick: move |_| {
                            //         handle_import();
                            //     },
                            //     div {
                            //     dangerous_inner_html: "{LEFT}"
                            //     }
                            // }
                        }
                        for (i, relay_url) in current_relay_set.relays.iter().enumerate() {
                            div {
                                class:"relay-url-item mb-10 flex items-center",
                                input {
                                    class: "relay-ipt mr-10",
                                    r#type: "text",
                                    value: "{relay_url}",
                                    placeholder: "wss://",
                                    oninput: move |event| {
                                        let mut _relay_sets = relay_sets.write();
                                        _relay_sets[relay_curent_index()].relays[i] = event.value().clone();
                                    }
                                }
                                button {
                                    class: "btn-circle btn-circle-false relay-url-del",
                                    onclick: move |_| {
                                        let mut _relay_sets = relay_sets.write();
                                        _relay_sets[relay_curent_index()].relays.remove(i);
                                    },
                                    div {
                                        dangerous_inner_html: "{FALSE}"
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
                                class: "btn-icon add relay-url-add",
                                onclick: move |_| {
                                    if new_relay().is_empty() {
                                        return;
                                    } else if !wss_regx.is_match(&new_relay()) {
                                        spawn(async move {
                                            alert("Invalid URL".to_string()).await;
                                        });
                                    } else if current_relay_set.relays.iter().any(|x| x == &new_relay()){
                                        spawn(async move {
                                            alert("Relay already exists".to_string()).await;
                                        });
                                    } else {
                                        relay_sets.write()[relay_curent_index()].relays.push((new_relay.clone())());
                                        new_relay.set(String::new());
                                    }
                                    //  current_relay_set.relays.push(new_relay.clone());
                                },
                                div {
                                    dangerous_inner_html: "{ADD}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
