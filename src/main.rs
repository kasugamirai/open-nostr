#![allow(non_snake_case)]

use dioxus::prelude::*;
use nostr_sdk::{Client, Keys};
use tracing::{debug, Level};

use capybastr::{CustomSub, Route};

fn main() {
    // Init debug
    dioxus_logger::init(Level::DEBUG).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    tracing::info!("Welcome to Capybastr!!");

    // theme class name
    let theme = use_context_provider(|| Signal::new(String::from("light")));

    // custom subscription relays and filters
    let custom_sub_global = use_context_provider(|| Signal::new(CustomSub::default()));

    // TODO: user private key, set by user in settings page
    let pk: &str = "nsec1dmvtj7uldpeethalp2ttwscy32jx36hr9jslskwdqreh2yk70anqhasx64";
    let my_keys = Keys::parse(pk).unwrap();

    // create nostr client
    let mut client = use_context_provider(|| Signal::new(Client::new(&my_keys)));

    // hook: on mounted
    let on_mounted = move |_| {
        spawn(async move {
            // TODO: Step 1, read cache from indexeddb else create new subscription

            // Step 2, connect to relays
            let c = client.write();
            for i in custom_sub_global.read().relay_set.relays.iter() {
                c.add_relay(i.clone().as_str()).await.unwrap();
            }
            debug!("Connecting...");
            c.connect().await;
            debug!("Connected");
        });
    };

    // hook: before drop
    use_drop(move || {
        spawn(async move {
            debug!("Disconnecting...");
            let _ = client.write().disconnect().await;
            debug!("Disconnected");
        });
    });

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
            id: "app",
            class: "{theme}",
            Router::<Route> {}
        }
    }
}
