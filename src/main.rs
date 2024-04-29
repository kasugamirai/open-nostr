#![allow(non_snake_case)]

use dioxus::prelude::*;
use nostr_sdk::{Client, Keys};
use tracing::info;
use tracing::Level;

use capybastr::Route;

use capybastr::CustomSub;

fn main() {
    // Init debug
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    tracing::info!("Starting Dioxus");
    launch(App);
}

fn App() -> Element {
    tracing::info!("Welcome to Capybastr!!");
    let state = use_context_provider(|| Signal::new(String::from("light")));
    let custom_sub_global = use_context_provider(|| Signal::new(CustomSub::default()));

    let pk: &str = "nsec1dmvtj7uldpeethalp2ttwscy32jx36hr9jslskwdqreh2yk70anqhasx64";
    let my_keys = Keys::parse(pk).unwrap();
    let mut client = use_context_provider(|| Signal::new(Client::new(&my_keys)));

    use_before_render(move || {
        spawn(async move {
            let c = client.write();
            for i in custom_sub_global.read().relay_set.relays.iter() {
                c.add_relay(i.clone().as_str()).await.unwrap();
            }
            info!("Connecting...");
            c.connect().await;
        });
    });

    use_drop(move || {
        spawn(async move {
            info!("Disconnecting...");
            let _ = client.write().disconnect().await;
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
            id: "app",
            class: "{state}",
            Router::<Route> {}
        }
    }
}
