use std::collections::HashMap;
use std::time::Duration;

use dioxus::prelude::*;
use nostr_sdk::prelude::*;

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
            class:"test-box",
            h1 {
                class:"font-weight-bold",
                "Parent"
            }
            div {
                class:"gap-20 display-flex-box flex-col",
                div {
                    class: "parent-li",
                    ChildrenKeep {
                        name: "Dog".to_string()
                    }
                }
                div {
                    class: "parent-li",
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

            events.set(data);
        });
    };
    let auotGetData: &'static str = "
        var getDataBtn = document.getElementById('get-data-test');
        if (getDataBtn){{
            getDataBtn.click();
        }}
        ";

    rsx! {
        div {
            button {
                id: "get-data-test",
                onclick: on_mounted,
                "Get Data"
            }
            h2 {
                class: "font-weight-bold",
                "Children: {n}"
            }
            div {
                // for (i, note) in events.read().clone().iter().enumerate() {
                //     Note {
                //         sub_name: "".to_string(),
                //         data: NoteData::from(note, i),
                //     }
                // }
            }
            script {
                {auotGetData}
            }
        }

    }
}

#[component]
pub fn ChildrenKeep(name: String) -> Element {
    let clients = use_context::<Signal<Clients>>();

    // let events = use_signal(|| vec![]);

    let n = name.clone();

    let on_mounted = move |_| {
        let name = name.clone();
        spawn(async move {
            // let database = WebDatabase::open("EVENTS_DB").await.unwrap();
            // let clients = clients();
            // let client = clients.get(&name).unwrap();

            // // let filter = Filter::new().hashtag(name).kind(Kind::TextNote).limit(2);

            // // let data = client.subscribe(sub_id_1.clone(), vec![filter], None).await;

            // // events.set(data);

            // let subscription = Filter::new()
            //     .hashtag(name)
            //     .kind(Kind::TextNote)
            //     .since(Timestamp::now());

            // // Subscribe
            // let sub_id = client.subscribe(vec![subscription], None).await;

            // tracing::info!("client: {client:?}");

            // client
            //     .handle_notifications(|notification| async {
            //         match notification {
            //             RelayPoolNotification::Event {
            //                 relay_url,
            //                 subscription_id,
            //                 event,
            //             } => {
            //                 if subscription_id == sub_id {
            //                     database.save_event(&event).await.unwrap();
            //                     tracing::info!("{relay_url}: {event:?}");
            //                 }
            //             }
            //             _ => {}
            //         }
            //         Ok(false) // Set to true to exit from the loop
            //     })
            //     .await
            //     .unwrap();
        });
    };

    rsx! {
        div {
            onmounted: on_mounted,
            h2 {
                class: "font-weight-bold",
                "Children: {n}"
            }
            div {
                // for (i, note) in events.read().clone().iter().enumerate() {
                //     Note {
                //         sub_name: "".to_string(),
                //         data: NoteData::from(note, i),
                //     }
                // }
            }
        }
    }
}
