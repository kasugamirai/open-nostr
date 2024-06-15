use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use dioxus::prelude::*;
use nostr_indexeddb::WebDatabase;
use nostr_sdk::{ClientBuilder, SubscriptionId};

use crate::components::{ModalManager, ModalManagerProvider};
use crate::nostr::Register;
use crate::nostr::{EventCache, HashedClient, MultiClient};
use crate::store::subscription::{CustomHashTag, CustomSub, FilterTemp, RelaySet};
use crate::store::user::NoLogin;
use crate::store::{
    AccountType, CBWebDatabase, CBwebDatabaseError, User, CAPYBASTR_DBNAME, DEFAULT_RELAY_SET_KEY,
};
use crate::Route;

pub const EXAMPLE_SUB_KEY: &str = "nostr";
pub const FOLLOWING_SUB_KEY: &str = "following";
pub const EXAMPLE_SUB_TAG: &str = "nostr";
pub const NOSTR_DB_NAME: &str = "nostr-db";
pub const LAST_LOGINED_KEY: &str = "last_logined";
pub const NOT_LOGGED_IN_USER_NAME: &str = "NOT_LOGGED_IN";

#[derive(Debug)]
pub struct Counter {
    counts: Arc<RwLock<HashMap<SubscriptionId, usize>>>,
}

impl Counter {
    pub fn new() -> Self {
        Self {
            counts: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    pub fn get(&self, id: &SubscriptionId) -> Option<usize> {
        let counts = self.counts.read().unwrap();
        counts.get(id).map(|count| *count)
    }
    pub fn inc(&self, id: &SubscriptionId) {
        let mut counts = self.counts.write().unwrap();
        let count = counts.entry(id.clone()).or_insert(0);
        *count += 1;
    }
    pub fn dec(&self, id: &SubscriptionId) {
        let mut counts = self.counts.write().unwrap();
        let count = counts.entry(id.clone()).or_insert(0);
        *count -= 1;
    }
    pub fn clear(&self, id: &SubscriptionId) {
        let mut counts = self.counts.write().unwrap();
        let count = counts.entry(id.clone()).or_insert(0);
        *count = 0;
    }
    pub fn clear_all(&self) {
        let mut counts = self.counts.write().unwrap();
        counts.clear();
    }
}

// Atoms and AtomRefs have been replaced with GlobalSignals
pub static SUB_COUNTERS: GlobalSignal<Counter> = Signal::global(|| Counter::new());

pub static MODAL_MANAGER: GlobalSignal<ModalManager> = Signal::global( || ModalManager::new());

#[allow(non_snake_case)]
pub fn App() -> Element {
    tracing::info!("Welcome to Capybastr!!");
    let mut multiclient = use_context_provider(|| Signal::new(MultiClient::new()));
    let mut all_sub: Signal<Vec<CustomSub>> =
        use_context_provider(|| Signal::new(Vec::<CustomSub>::new()));
    let mut subs_map: Signal<HashMap<String, CustomSub>> =
        use_context_provider(|| Signal::new(HashMap::<String, CustomSub>::new()));
    let mut all_users: Signal<Vec<User>> = use_context_provider(|| Signal::new(Vec::<User>::new()));

    // theme class name
    let theme = use_context_provider(|| Signal::new(String::from("light")));

    let mut router = use_signal(|| rsx! {div{}});

    use_context_provider(|| Signal::new(EventCache::new(300, 300)));

    // use_context_provider(|| Signal::new(Counter::new(true)));
    use_context_provider(|| Signal::new(Register::new()));



    // hook: on mounted
    let on_mounted = move |_| {
        // init treading
        spawn(async move {
            let _database = CBWebDatabase::open(CAPYBASTR_DBNAME).await.unwrap();
            // global database
            let cb_database_db = use_context_provider(|| Signal::new(_database));
            // global multiclient

            let db = cb_database_db.read();

            // check if there is default relay sets
            if let Err(CBwebDatabaseError::NotFound) =
                db.get_relay_set(DEFAULT_RELAY_SET_KEY.to_string()).await
            {
                db.save_relay_set(RelaySet {
                    name: DEFAULT_RELAY_SET_KEY.to_string(),
                    relays: vec!["wss://nos.lol".to_string(), "wss://nostr.wine".to_string()],
                })
                .await
                .unwrap();
            }

            //init nostr db
            let nostr_db = WebDatabase::open(NOSTR_DB_NAME).await.unwrap();

            //init multiclient
            let relay_sets: Vec<RelaySet> = db.get_all_relay_sets().await.unwrap();
            if !relay_sets.is_empty() {
                let mut _multiclient = multiclient.write(); // Await the write lock
                for rs in relay_sets {
                    let client = _multiclient.get_client(&rs.name).await;
                    if client.is_none() {
                        let client_builder = ClientBuilder::new().database(nostr_db.clone());
                        let c: nostr_sdk::Client = client_builder.build();
                        c.add_relays(rs.relays).await.unwrap();
                        c.connect().await;
                        let hc = HashedClient::new(c).await;
                        _multiclient.register(rs.name, hc).await;
                    }
                }
            }
            //init custom sub
            match db.get_all_subs().await {
                Ok(subs) => {
                    if subs.is_empty() {
                        let custom_sub = CustomSub {
                            name: EXAMPLE_SUB_KEY.to_string(),
                            relay_set: DEFAULT_RELAY_SET_KEY.to_string(),
                            live: false,
                            since: 0,
                            until: 0,
                            filters: vec![FilterTemp::HashTag(CustomHashTag {
                                r#type: String::from("hashtag"),
                                tags: vec![EXAMPLE_SUB_TAG.to_string()],
                            })],
                            keep_alive: true,
                        };
                        db.save_custom_sub(custom_sub.clone()).await.unwrap();
                        subs_map
                            .write()
                            .insert(EXAMPLE_SUB_KEY.to_string(), custom_sub.clone());
                        // TODO remove this line
                        all_sub.push(custom_sub);
                    } else {
                        for sub in subs {
                            subs_map.write().insert(sub.name.clone(), sub.clone());
                            // TODO remvoe this line
                            all_sub.push(sub);
                        }
                    }
                }
                Err(_) => {
                    //todo
                }
            }

            //init users
            match db.get_all_users().await {
                Ok(users) => {
                    if users.is_empty() {
                        let user = User {
                            name: NOT_LOGGED_IN_USER_NAME.to_string(),
                            inner: AccountType::NotLoggedIn(NoLogin::empty()),
                        };
                        db.save_user(user).await.unwrap();

                        //and record a last login user
                        db.save_misc(
                            LAST_LOGINED_KEY.to_string(),
                            NOT_LOGGED_IN_USER_NAME.to_string(),
                        )
                        .await
                        .unwrap();
                    } else {
                        for user in users {
                            all_users.push(user);
                        }
                    }
                }
                Err(_) => {
                    //todo
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
            
            ModalManagerProvider {}
        }
    }
}
