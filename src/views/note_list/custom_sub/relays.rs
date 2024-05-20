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
    let relay_name = use_signal(|| props.relay_name.clone());
    // let relay_sets.read() = relay_sets.read();
    let current_relay_set: RelaySet = match relay_sets.read().get(relay_curent_index()) {
        Some(relay) => relay.clone(),
        None => RelaySet::new(&relay_sets.read().len()),
    };
    let wss_regx = Regex::new(WSS_REG).unwrap();
    use_effect(move || {
        spawn(async move {
            let cb_database_db_write = cb_database_db.read();
            let _relay_sets: Vec<RelaySet> =
                cb_database_db_write.get_all_relay_sets().await.unwrap();
            relay_sets.set((|| _relay_sets.clone())());
            old_relay_sets.set(_relay_sets); // Clone the value before using it
            match relay_sets.iter().position(|x| x.name == relay_name()) {
                Some(i) => {
                    relay_curent_index.set(i);
                }
                None => {
                    relay_curent_index.set(0);
                }
            }
        });
    });
    // // update value and cancel editing when parent data has changed
    // use_effect(use_reactive(
    //     (&relay_name, &edit),
    //     // move |(mut valuezz, relay_set, mut edit)| {
    //     //     value.set(relay_set.clone());
    //     //     bak.set(relay_set.clone());
    //     //     edit.set(false);
    //     // },

    //     move |mut new_relay_name, mut _edit| {
    //         spawn(async move {
    //             let cb_database_db_write = cb_database_db.write();
    //             let _relay_set: RelaySet = cb_database_db_write.get_relay_set(new_relay_name()).await.unwrap();
    //             // relay_set.set(_relay_set)
    //             // value.set(&relay_set);
    //             // bak.set(relay_set.clone());
    //             // edit.set(false);
    //         });

    //         return ();
    //     },
    // ));

    // let handle_export = move |text: String| {
    //     export_to_clipboard(text);
    // };

    // let handle_import = move || {
    //     // tracing::error!("import {:?}", _);
    //     // spawn(async move {
    //     //     let text = import_from_clipboard().await;
    //     //     if !text.is_empty() {
    //     //         let rs: Vec<String> = text.split(',').map(|s| s.trim().to_string()).collect();
    //     //         let mut v = value.write();
    //     //         v.relays = rs;
    //     //     }
    //     // });
    // };
    // let mut relay_sets: Signal<Vec<RelaySet>> = use_signal(Vec::new);

    let handle_save = move || {

        let mut duplicate_names = HashSet::new();
        let mut names_set = HashSet::<String>::new();
        for relay in relay_sets.read().iter() {
            if !names_set.insert(relay.name.to_string()) {
                // 如果插入失败，说明该 name 已经存在于集合中
                duplicate_names.insert(relay.name.to_string());
            }
        }

        // 如果有重复的 name，则打印出来并从 new_added 中移除
        if !duplicate_names.is_empty() {
            spawn(async move {
                alert(format!("Duplicate names: {:?}", duplicate_names)).await;
            });
            return;
        }
        spawn(async move {
            let cb_database_db_write = cb_database_db.write();
            let current_relay_sets = &*
            relay_sets.read();
            let previous_relay_sets = &old_relay_sets.read();

            // 将 previous_relay_sets 转换为 HashMap 以便快速查找
            let previous_map: HashMap<_, _> = previous_relay_sets
                .iter()
                .map(|relay| (&relay.name, relay))
                .collect();
            // 初始化新增加的值和被修改的值的集合
            let mut new_added: Vec<RelaySet> = Vec::new();
            let mut modified: Vec<ModifiedRelaySet> = Vec::new();

            // 找出新增加和被修改的值
            for relay in current_relay_sets.iter() {
                match previous_map.get(&relay.name) {
                    Some(prev_relay) => {
                        if **prev_relay != *relay {
                            modified.push(ModifiedRelaySet {
                                old_name: prev_relay.name.clone(),
                                new_relay: relay.clone(),
                            });
                        }
                    }
                    None => {
                        new_added.push(relay.clone());
                    }
                }
            }

            // 找出删除的值
            let current_ids: HashMap<_, _> = current_relay_sets
                .iter()
                .map(|relay| (&relay.name, relay))
                .collect();
            let deleted: Vec<RelaySet> = previous_relay_sets
                .iter()
                .filter(|relay| !current_ids.contains_key(&relay.name))
                .cloned()
                .collect();

            // duplicate_names
            tracing::info!("new_added: {:#?}", new_added);
            tracing::info!("modified: {:#?}", modified);
            tracing::info!("deleted: {:#?}", deleted);

            // new add
            // cb_database_db_write.save_relay_set()
            let mut tips = String::new();
            for relay in new_added.iter() {
                if relay.relays.is_empty() {
                    tips.push_str(&format!("Relay set {} must have at least one relay\n", relay.name));
                }
            }
            for relay in modified.iter() {
                if relay.new_relay.relays.is_empty() {
                    tips.push_str(&format!("Relay set {} must have at least one relay\n", relay.new_relay.name));
                }
            }
            if !tips.is_empty() {
                alert(tips).await;
                return;
            }
            // add 
            for relay in new_added.iter() {
                cb_database_db_write.save_relay_set(relay.clone()).await.unwrap();
            }
            // modify
            for relay in modified.iter() {
                cb_database_db_write.relay_set_change(relay.old_name.clone(), relay.new_relay.clone()).await.unwrap();
            }

        });
    };
    rsx! {
        div {
            class: "relay-btn relative",
            div {
                class: format!("subInput cursor-{}", if allow_edit() { "pointer" } else { "disabled" }),
                onclick: move |_| {
                  if allow_edit() { // 判断是否编辑状态
                    edit.set(!edit());
                  }
                },
                "{props.relay_name}"
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
                                disabled: if current_relay_set.name == DEFAULT_RELAY_SET_KEY { true } else { false },
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
                                    // 1. todo save relays
                                    // 2.
                                    // bak.set(value());
                                    // props.on_change.call(relay_sets.read().get(relay_curent_index()).unwrap().clone());
                                    // edit.set(false);
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
