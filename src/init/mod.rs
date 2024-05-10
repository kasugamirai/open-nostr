use std::collections::HashMap;

use dioxus::prelude::*;
use nostr_sdk::Client;

use crate::{storage::CapybastrDb, CustomSub, Route};

#[allow(non_snake_case)]
pub fn App() -> Element {
    tracing::info!("Welcome to Capybastr!!");

    let mut clients = use_context_provider(|| Signal::new(HashMap::<String, Client>::new()));

    // all custom subscriptions
    let mut all_sub: Signal<Vec<CustomSub>> =
        use_context_provider(|| Signal::new(Vec::<CustomSub>::new()));

    // theme class name
    let theme = use_context_provider(|| Signal::new(String::from("light")));

    // hook: on mounted
    let on_mounted = move |_| {
        spawn(async move {
            // TODO: Step 1, read cache from indexeddb else create new subscription
            // let db = CapybastrDb::new("subscription list".to_string())
            //     .await
            //     .unwrap();
            // db.delete_data(&String::from("SUBSCRIPTION_LIST"))
            //     .await
            //     .unwrap();
            // db.add_data(
            //     &String::from("SUBSCRIPTION_LIST"),
            //     &String::from("[\"Dog\", \"Car\"]"),
            // )
            // .await
            // .unwrap();

            let subs = vec![
                CustomSub::default_with_opt(
                    "Dog".to_string(),
                    "wss://btc.klendazu.com".to_string(),
                    vec!["dog".to_string()],
                    true,
                ),
                CustomSub::default_with_opt(
                    "Car".to_string(),
                    "wss://relay.damus.io".to_string(),
                    vec!["car".to_string()],
                    false,
                ),
            ];

            let mut cs = clients.write();
            for i in subs.iter() {
                let c = Client::default();
                c.add_relays(i.relay_set.relays.clone()).await.unwrap();
                c.connect().await;
                cs.insert(i.name.clone(), c);
            }

            for i in subs.iter() {
                all_sub.push(i.clone());
            }
        });
    };

    let mut root_click_pos = use_context_provider(|| Signal::new((0.0, 0.0)));

    let style = format!(
        "\n{}\n{}\n{}\n{}\n{}\n{}\n",
        include_str!("../../assets/style/tailwind.css"),
        include_str!("../../assets/style/main.css"),
        include_str!("../../assets/style/components.css"),
        include_str!("../../assets/style/layout-left.css"),
        include_str!("../../assets/style/layout-main.css"),
        include_str!("../../assets/style/layout-right.css"),
    );

    rsx! {
        style { "{style}" }
        div {
            onmounted: on_mounted,
            onclick: move |event| {
                root_click_pos.set(event.screen_coordinates().to_tuple());
            },
            id: "app",
            class: "{theme}",
            Router::<Route> {}
        }
    }
}
