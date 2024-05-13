use std::{collections::HashMap, sync::Arc};

use dioxus::prelude::*;
use nostr_indexeddb::WebDatabase;
use nostr_sdk::{Client, ClientBuilder, RelayMessage, RelayPoolNotification, SubscriptionId};
use serde::ser::StdError;

use crate::store::subscription::{CustomSub, RelaySet, SubNames};
use crate::store::CBWebDatabase;
use crate::{nostr::register::*, Route};

async fn init() {
    let database = CBWebDatabase::open("Capybastr-db").await.unwrap();

    let name_list = SubNames::new(vec!["Dog".to_string(), "Car".to_string()]);
    database.save_sub_name_list(name_list).await.unwrap();

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
    let mut register = use_context_provider(|| Signal::new(Register::new()));

    let mut clients = use_context_provider(|| Signal::new(HashMap::<String, Client>::new()));

    // all custom subscriptions
    let mut all_sub: Signal<Vec<CustomSub>> =
        use_context_provider(|| Signal::new(Vec::<CustomSub>::new()));

    // theme class name
    let theme = use_context_provider(|| Signal::new(String::from("light")));

    // hook: on mounted
    let on_mounted = move |_| {
        spawn(async move {
            // TODO: init().await;

            let database = CBWebDatabase::open("Capybastr-db").await.unwrap();

            let name_list = database.get_sub_name_list().await.unwrap();

            let mut subs = vec![];
            for i in name_list.names.iter() {
                let sub = database.get_custom_sub(i.to_string()).await.unwrap();
                subs.push(sub);
            }

            async fn handler_text_note(
                notification: RelayPoolNotification,
            ) -> Result<bool, Box<dyn StdError>> {
                if let RelayPoolNotification::Message {
                    message: RelayMessage::Event { event, .. },
                    ..
                } = notification
                {
                    println!("TextNote: {:?}", event);
                    tracing::info!("TextNote: {:?}", event);
                }
                Ok(false)
            }

            let mut cs = clients.write();
            for i in subs.iter() {
                let client_builder =
                    ClientBuilder::new().database(WebDatabase::open("EVENTS_DB").await.unwrap());
                let c = client_builder.build();
                c.add_relays(i.relay_set.relays.clone()).await.unwrap();
                c.connect().await;
                cs.insert(i.name.clone(), c.clone());

                if i.live {
                    let s = i.clone();
                    use_coroutine(|_: UnboundedReceiver<()>| async move {
                        (*register.read())
                            .add_subscription(
                                &c.clone(),
                                SubscriptionId::new(s.name.clone()),
                                s.get_filters(),
                                Arc::new(|notification| Box::pin(handler_text_note(notification))),
                                None,
                            )
                            .await
                            .unwrap();
                        (*register.read())
                            .handle_notifications(&c.clone())
                            .await
                            .unwrap();
                    });
                }
            }

            for i in subs.iter() {
                all_sub.push(i.clone());
            }
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
            Router::<Route> {}
        }
    }
}
