#![allow(non_snake_case)]

use std::collections::HashMap;

use dioxus::{html::sub, prelude::*};
use nostr_sdk::{Client, Keys};
use tracing::Level;

use capybastr::{CustomSub, Route, User};

fn main() {
    // Init debug
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    tracing::info!("Welcome to Capybastr!!");

    // all users
    let all_user: Signal<Vec<User>> = use_context_provider(|| Signal::new(Vec::<User>::new()));

    // current user, default is MAX to cancel the change event when init
    let current_user: Signal<usize> = use_context_provider(|| Signal::new(usize::MAX));

    // all custom subscriptions
    let mut all_sub: Signal<Vec<CustomSub>> =
        use_context_provider(|| Signal::new(Vec::<CustomSub>::new()));

    // current subscription index, default is MAX to cancel the change event when init
    let mut current_sub: Signal<usize> = use_context_provider(|| Signal::new(usize::MAX));

    // TODO: all events, read from indexeddb
    let _all_events =
        use_context_provider(|| Signal::new(HashMap::<String, Vec<nostr_sdk::Event>>::new()));

    // theme class name
    let theme = use_context_provider(|| Signal::new(String::from("light")));

    // create nostr client
    let mut client = use_context_provider(|| Signal::new(Client::default()));

    let _last_reload: Signal<i32> = use_context_provider(|| Signal::new(0));

    // current user has changed
    use_effect(use_reactive(
        (&all_user(), &all_sub(), &current_sub()),
        move |(users, subs, current_sub)| {
            spawn(async move {
                let index = *current_user.read();
                if index != usize::MAX && index < users.len() {
                    let user = users.get(index).unwrap();
                    if let Some(pk) = user.public_key.clone() {
                        let keys = Keys::parse(pk).unwrap();

                        let new_client = Client::new(keys);
                        let subscription = subs.get(current_sub).unwrap();
                        let relays = subscription.relay_set.relays.clone();
                        let _ = new_client.add_relays(relays).await;
                        new_client.connect().await;

                        client.set(new_client);
                    }
                }
            });
        },
    ));

    // current subscription has changed
    use_effect(use_reactive(&all_sub(), move |subs| {
        spawn(async move {
            tracing::info!("Current sub index ====== {}", *current_sub.read());
            let index = *current_sub.read();
            if index != usize::MAX && index < subs.len() {
                let subscription = subs.get(index).unwrap();
                let relays = subscription.relay_set.relays.clone();
                let client = client.read();
                let _ = client.disconnect().await;
                let _ = client.remove_all_relays().await;
                let _ = client.add_relays(relays).await;
                client.connect().await;
            }
        });
    }));

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

            // default use the first subscription
            current_sub.set(0);
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
