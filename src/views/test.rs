use std::{collections::HashMap, time::Duration};

use dioxus::prelude::*;
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
        self.clients.get(name)
    }
}

#[component]
pub fn Test(id: i32) -> Element {
    let mut clients = use_context_provider(|| Signal::new(Clients::new()));

    let on_mounted = move |_| {
        spawn(async move {
            let c1 = Client::default();
            c1.add_relay("wss://relay.damus.io").await.unwrap();
            c1.connect().await;

            let c2 = Client::default();
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
                    Children {
                        name: "Dog".to_string()
                    }
                }
                div {
                    style: "border: 1px solid #333; border-radius: 10px; padding: 10px; flex: 1; word-wrap:break-word;",
                    Children {
                        name: "Cat".to_string()
                    }
                }
            }
        }
    }
}

#[component]
pub fn Children(name: String) -> Element {
    let clients = use_context::<Signal<Clients>>();

    let mut events = use_signal(|| vec![]);

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

            events.set(data);
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
