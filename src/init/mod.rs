use std::{collections::HashMap, sync::Arc};

use dioxus::prelude::*;
use nostr_indexeddb::WebDatabase;
use nostr_sdk::{Client, ClientBuilder, RelayMessage, RelayPoolNotification, SubscriptionId};
use serde::ser::StdError;

use crate::{nostr::register::*, storage::CapybastrDb, CustomSub, Route};

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
            // TODO: Step 1, read cache from indexeddb else create new subscription
            let db = CapybastrDb::new("subscription".to_string()).await.unwrap();
            let db2 = CapybastrDb::new("subscription2".to_string()).await.unwrap();
            // let sub_names: Vec<String> = db.read_data("SUBSCRIPTION_LIST").await.unwrap();
            db.add_data("key", &1);
            db2.add_data("key", &2);

            let sub_names: Vec<String> = vec!["Dog".to_string(), "Car".to_string()];

            let mut subs = vec![];
            for i in sub_names.iter() {
                match db2.read_data::<String>(i).await {
                    Ok(v) => subs.push(CustomSub::from(&v)),
                    Err(e) => eprintln!("Error reading data: {}", e),
                }
                match db.read_data::<String>(i).await {
                    Ok(v) => subs.push(CustomSub::from(&v)),
                    Err(e) => eprintln!("Error reading data: {}", e),
                }
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