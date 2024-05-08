#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;

use capybastr::{CustomSub, Route};

fn main() {
    // Init debug
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    tracing::info!("Welcome to Capybastr!!");

    let client = use_context_provider(|| Signal::new(nostr_sdk::Client::default()));

    // all custom subscriptions
    let mut all_sub: Signal<Vec<CustomSub>> =
        use_context_provider(|| Signal::new(Vec::<CustomSub>::new()));

    // theme class name
    let theme = use_context_provider(|| Signal::new(String::from("light")));

    // hook: on mounted
    let on_mounted = move |_| {

        spawn(async move {
            let c = client();
            c.add_relay("wss://relay.damus.io").await.unwrap();
            c.connect().await;

            // TODO: Step 1, read cache from indexeddb else create new subscription

            all_sub.push(CustomSub::default_with_hashtags(
                "Dog".to_string(),
                vec!["dog".to_string()],
            ));
            all_sub.push(CustomSub::default_with_hashtags(
                "Car".to_string(),
                vec!["car".to_string()],
            ));
        });
    };

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
