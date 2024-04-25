use std::time::Duration;

use dioxus::prelude::*;
use nostr_sdk::prelude::*;

use crate::{
    components::{Button, Post, PostData},
    state::subscription::CustomSub,
};

#[component]
pub fn Home() -> Element {
    let custom_sub_global = use_context::<Signal<CustomSub>>();
    let mut post_datas = use_signal(Vec::<PostData>::new);
    let get_events = move || {
        spawn(async move {
            let pk: &str = "nsec1dmvtj7uldpeethalp2ttwscy32jx36hr9jslskwdqreh2yk70anqhasx64";
            // pk to hex
            let my_keys = Keys::parse(pk).unwrap();

            let client = Client::new(&my_keys);

            for i in custom_sub_global.read().relay_set.relays.iter() {
                client.add_relay(i.clone().as_str()).await.unwrap();
            }

            client.connect().await;

            let filters = custom_sub_global.read().to_sub();

            let events_result = client
                .get_events_of(filters, Some(Duration::from_secs(30)))
                .await;

            match events_result {
                Ok(events) => {
                    for event in events {
                        let post_data = PostData {
                            id: event.id().to_hex(),
                            author: event.author().to_hex(),
                            created_at: event.created_at().as_u64(),
                            kind: "".to_string(),
                            tags: vec![],
                            content: event.content.to_string(),
                        };
                        post_datas.push(post_data);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to get events: {}", e);
                }
            }
            let _ = client.disconnect().await;
        });
    };

    let handle_get_events = move |_| {
        get_events();
    };

    rsx! {
        ul {
            style: "display: flex; flex-direction: column; gap: 10px;",
            for i in post_datas.iter() {
                Post { data: i.clone() }
            }
        }
        Button { on_click: handle_get_events, "Get Events" }
    }
}
