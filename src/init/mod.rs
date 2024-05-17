use dioxus::prelude::*;
use nostr_indexeddb::WebDatabase;
use nostr_sdk::ClientBuilder;
use crate::store::subscription::{CustomSub, RelaySet};
use crate::store::CBWebDatabase;
use crate::utils::contants::{CAPYBASTR_DBNAME, DEFAULT_CUSTOM_SUBS, DEFAULT_RELAY_SET_NAMES};
use crate::{
    nostr::{multiclient::MultiClient, register::*},
    Route,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct DefaultCustomSub {
    name: String,
    relay_set_name: String,
    tags: Vec<String>,
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
        // init treading
        spawn(async move {
            let _database = CBWebDatabase::open(CAPYBASTR_DBNAME).await.unwrap();
            // global database
            let cb_database_db = use_context_provider(|| Signal::new(_database));
            // global multiclient

            let db = cb_database_db.read();

            let default_custom_subs: Vec<DefaultCustomSub> =
                serde_json::from_str(DEFAULT_CUSTOM_SUBS).expect("Failed to parse custom subs");
            let default_relay_sets: Vec<RelaySet> =
                serde_json::from_str(&DEFAULT_RELAY_SET_NAMES).expect("Failed to parse relay sets");
            for rs in default_relay_sets {
                match db.get_relay_set(rs.name.clone()).await {
                    NotFound => {
                        db.save_relay_set(rs).await.unwrap();
                    }
                }
            }

            // subs
            match db.get_all_subs().await {
                Ok(subs) => {
                    if subs.len() == 0 {
                        for custom_sub in default_custom_subs {
                            let sub = CustomSub::default_with_opt(
                                custom_sub.name,
                                custom_sub.relay_set_name,
                                custom_sub.tags.to_vec(),
                                false,
                            );
                            db.save_custom_sub(sub).await.unwrap();
                        }
                    }
                }
                Err(_) => {
                    tracing::error!("Failed to get all subs");
                }
            }
            let relay_sets: Vec<RelaySet> = db.get_all_relay_sets().await.unwrap();
            if !relay_sets.is_empty() {
                let mut _multiclient = multiclient.write();
                for rs in relay_sets {
                    let client = _multiclient.get(&rs.name);
                    if client.is_none() {
                        let client_builder = ClientBuilder::new()
                            .database(WebDatabase::open(rs.name.clone()).await.unwrap());
                        let c: nostr_sdk::Client = client_builder.build();
                        c.add_relays(rs.relays).await.unwrap();
                        c.connect().await;
                        _multiclient.register(rs.name, c);
                    }
                }
            }

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
