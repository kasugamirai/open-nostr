#![allow(non_snake_case)]

use std::collections::HashMap;

use dioxus::prelude::*;
use nostr_sdk::{Client, Keys};
use tracing::Level;

use capybastr::{CustomSub, Route};

fn main() {
    // Init debug
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    tracing::info!("Welcome to Capybastr!!");

    let mut all_sub: Signal<Vec<CustomSub>> =
        use_context_provider(|| Signal::new(Vec::<CustomSub>::new()));
    let _current_sub: Signal<usize> = use_context_provider(|| Signal::new(0));

    // TODO: all events, read from indexeddb
    let _all_events =
        use_context_provider(|| Signal::new(HashMap::<String, Vec<nostr_sdk::Event>>::new()));

    // theme class name
    let theme = use_context_provider(|| Signal::new(String::from("light")));

    // TODO: user private key, set by user in settings page
    let pk: &str = "nsec1dmvtj7uldpeethalp2ttwscy32jx36hr9jslskwdqreh2yk70anqhasx64";
    let my_keys = Keys::parse(pk).unwrap();

    // create nostr client
    let mut client = use_context_provider(|| Signal::new(Client::new(&my_keys)));

    // hook: on mounted
    let on_mounted = move |_| {
        spawn(async move {
            // TODO: Step 1, read cache from indexeddb else create new subscription

            all_sub.push(CustomSub::default_with_hashtags(
                "Dog".to_string(),
                vec!["dog".to_string()],
            ));
            all_sub.push(CustomSub::default_with_hashtags(
                "Car".to_string(),
                vec!["car".to_string()],
            ));

            // Step 2, connect to relays
            let c = client.write();
            c.add_relay("wss://btc.klendazu.com").await.unwrap();
            c.connect().await;
        });
    };

    // hook: before drop
    use_drop(move || {
        spawn(async move {
            let _ = client.write().disconnect().await;
        });
    });

    let mut root_click_pos = use_context_provider(|| Signal::new((0.0, 0.0)));

    let style = format!(
        "\n{}\n{}\n{}\n{}\n{}\n{}\n",
        include_str!("../assets/style/tailwind.css"),
        include_str!("../assets/style/main.css"),
        include_str!("../assets/style/components.css"),
        include_str!("../assets/style/layout-left.css"),
        include_str!("../assets/style/layout-main.css"),
        include_str!("../assets/style/layout-right.css"),
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
