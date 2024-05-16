use dioxus::prelude::*;
use nostr_indexeddb::WebDatabase;
use nostr_sdk::{ClientBuilder, RelayPoolNotification};

use crate::store::subscription::{CustomSub, RelaySet};
use crate::store::CBWebDatabase;
use crate::{
    nostr::{multiclient::MultiClient, register::*},
    utils::js::alert,
    Route,
};

pub const NOSTR_DB: &str = "nostr-idb";

async fn init() {
    let database = CBWebDatabase::open("Capybastr-db").await.unwrap();

    let sub = CustomSub::default_with_opt(
        "Dog".to_string(),
        "wss://relay.damus.io".to_string(),
        vec!["dog".to_string()],
        true,
    );
    database.save_custom_sub(sub).await.unwrap();

    let sub = CustomSub::default_with_opt(
        "Car".to_string(),
        "wss://btc.klendazu.com".to_string(),
        vec!["car".to_string()],
        false,
    );
    database.save_custom_sub(sub).await.unwrap();

    let rs = RelaySet {
        name: "Damus".to_string(),
        relays: vec!["wss://relay.damus.io".to_string()],
    };
    database.save_relay_set(rs).await.unwrap();

    let rs = RelaySet {
        name: "Klendazu".to_string(),
        relays: vec!["wss://btc.klendazu.com".to_string()],
    };
    database.save_relay_set(rs).await.unwrap();
}

#[allow(non_snake_case)]
pub fn App() -> Element {
    tracing::info!("Welcome to Capybastr!!");
    let _register = use_context_provider(|| Signal::new(Register::new()));

    let mut multiclient = use_context_provider(|| Signal::new(MultiClient::new()));

    // all custom subscriptions
    let mut all_sub: Signal<Vec<CustomSub>> =
        use_context_provider(|| Signal::new(Vec::<CustomSub>::new()));

    // theme class name
    let theme = use_context_provider(|| Signal::new(String::from("light")));

    let mut router = use_signal(|| rsx! {div{}});
    // hook: on mounted
    let on_mounted = move |_| {
        spawn(async move {
            let database = CBWebDatabase::open("Capybastr-db").await.unwrap();

            match database.get_all_subs().await {
                Ok(subs) => {
                    if subs.len() == 0 {
                        // TODO: Initialize database, remove it in production
                        init().await;
                    }
                }
                Err(_) => {}
            }

            //this logic is wrong
            match database.get_all_subs().await {
                Ok(subs) => {
                    let mut clients = multiclient.write();
                    for i in subs.clone().iter() {
                        let client_builder = ClientBuilder::new()
                            .database(WebDatabase::open(NOSTR_DB).await.unwrap());
                        let c = client_builder.build();
                        c.add_relays(i.relay_set.relays.clone()).await.unwrap();
                        c.connect().await;
                        clients.register(i.name.clone(), c.clone());
                        

                        if i.live {
                            let name = i.name.clone();
                            spawn(async move {
                                tracing::info!("subscribing: {name}");
                                match c
                                    .handle_notifications(|notification| async {
                                        match notification {
                                            RelayPoolNotification::Event {
                                                relay_url,
                                                subscription_id,
                                                event,
                                            } => {
                                                if subscription_id.to_string() == name {
                                                    c.database().save_event(&event).await.unwrap();
                                                    tracing::info!("{relay_url}: {event:?}");
                                                }
                                            }
                                            _ => {}
                                        }
                                        Ok(false) // Set to true to exit from the loop
                                    })
                                    .await
                                {
                                    Ok(_) => {}
                                    Err(e) => {
                                        alert(e.to_string()).await;
                                    }
                                }
                            });
                        }
                    }

                    all_sub.set(subs);
                }
                Err(e) => {
                    alert(e.to_string()).await;
                }
            };
            router.set(rsx! {Router::<Route> {}});
        });
    };

    let mut root_click_pos = use_context_provider(|| Signal::new((0.0, 0.0)));

    let style = format!("\n{}", include_str!("../../assets/main.dev.css"),);

    rsx! {
        style { "{style}" }
        div {
            onmounted: on_mounted,
            onclick: move |event| {
                root_click_pos.set(event.screen_coordinates().to_tuple());
            },
            id: "app",
            class: "{theme}",
            {router}
        }
    }
}
