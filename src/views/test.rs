use std::{collections::HashMap, time::Duration};

use dioxus::prelude::*;
use nostr_indexeddb::WebDatabase;
use nostr_sdk::prelude::*;

use crate::views::note_list::note::{Note, NoteData};

#[derive(Debug, Clone)]
pub struct Clients {
    clients: HashMap<String, Client>,
}

impl Clients {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: String, client: Client) {
        self.clients.insert(name, client);
    }

    pub fn get(&self, name: &str) -> Option<&Client> {
        tracing::info!("keys {:?}", self.clients.keys());
        self.clients.get(name)
    }
}

#[component]
pub fn Test(id: i32) -> Element {
    let mut clients = use_context_provider(|| Signal::new(Clients::new()));

    let on_mounted = move |_| {
        spawn(async move {
            let client_builder1 =
                ClientBuilder::new().database(WebDatabase::open("EVENTS_DB").await.unwrap());
            let c1 = client_builder1.build();
            c1.add_relay("wss://relay.damus.io").await.unwrap();
            c1.connect().await;

            let client_builder2 =
                ClientBuilder::new().database(WebDatabase::open("EVENTS_DB").await.unwrap());
            let c2 = client_builder2.build();
            c2.add_relay("wss://btc.klendazu.com").await.unwrap();
            c2.connect().await;

            let mut cs = clients.write();
            cs.register("Dog".to_string(), c1);
            cs.register("Cat".to_string(), c2);
        });
    };

    rsx! {
        div {
            onmounted: on_mounted,
            style: "display: flex; flex-direction: column; width: 100%; height: 100%; gap: 20px; overflow-y: auto;",
            h1 {
                style: "font-weight: bold;",
                "Parent"
            }
            div {
                style: "display: flex; flex-direction: column; gap: 20px;",
                div {
                    style: "border: 1px solid #333; border-radius: 10px; padding: 10px; flex: 1; word-wrap:break-word;",
                    ChildrenKeep {
                        name: "Dog".to_string()
                    }
                }
                div {
                    style: "border: 1px solid #333; border-radius: 10px; padding: 10px; flex: 1; word-wrap:break-word;",
                    Children {
                        name: "Dog".to_string()
                    }
                }
            }
        }
    }
}

#[component]
pub fn Children(name: String) -> Element {
    let clients = use_context::<Signal<Clients>>();

    let mut events = use_signal(std::vec::Vec::new);

    let n = name.clone();

    let on_mounted = move |_| {
        let name = name.clone();
        spawn(async move {
            let clients = clients();
            let client = clients.get(&name).unwrap();

            let filter = Filter::new().hashtag(name).kind(Kind::TextNote).limit(2);

            let data = client
                .get_events_of(vec![filter], Some(Duration::from_secs(30)))
                .await
                .unwrap();

            tracing::debug!("save1 {:?}", false);
            let res = client.database().save_event(&data[0]).await.unwrap();
            tracing::debug!("save2 {:?}", res);

            events.set(data);
        });
    };

    rsx! {
        div {
            button {
                onclick: on_mounted,
                "Get Data"
            }
            h2 {
                style: "font-weight: bold;",
                "Children: {n}"
            }
            div {
                for (i, note) in events.read().clone().iter().enumerate() {
                    Note {
                        data: NoteData::from(note, i),
                    }
                }
            }
        }
    }
}

#[component]
pub fn ChildrenKeep(name: String) -> Element {
    let clients = use_context::<Signal<Clients>>();

    let events = use_signal(std::vec::Vec::new);

    let n = name.clone();

    let on_mounted = move |_| {
        let name = name.clone();
        spawn(async move {
            let clients = clients();
            let client = match clients.get(&name) {
                Some(client) => client,
                None => {
                    eprintln!("No client found for name: {}", name);
                    return;
                }
            };
            // let filter = Filter::new().hashtag(name).kind(Kind::TextNote).limit(2);

            // let data = client.subscribe(sub_id_1.clone(), vec![filter], None).await;

            // events.set(data);

            let subscription = Filter::new()
                .hashtag(name)
                .kind(Kind::TextNote)
                .since(Timestamp::now());

            // Subscribe
            let sub_id = client.subscribe(vec![subscription], None).await;

            client
                .handle_notifications(|notification| async {
                    if let RelayPoolNotification::Event {
                        relay_url,
                        subscription_id,
                        event,
                    } = notification
                    {
                        if subscription_id == sub_id {
                            client.database().save_event(&event).await.unwrap();
                            tracing::info!("{relay_url}: {event:?}");
                        }
                    }
                    Ok(false) // Set to true to exit from the loop
                })
                .await
                .unwrap();
        });
    };

    rsx! {
        div {
            onmounted: on_mounted,
            h2 {
                style: "font-weight: bold;",
                "Children: {n}"
            }
            div {
                for (i, note) in events.read().clone().iter().enumerate() {
                    Note {
                        data: NoteData::from(note, i),
                    }
                }
            }
        }
    }
}
