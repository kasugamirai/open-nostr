use std::collections::HashMap;
use crate::store::CAPYBASTR_DBNAME;
use crate::store::CBWebDatabase;
use dioxus::hooks::use_context;
use dioxus::signals::Readable;
use dioxus::signals::Signal;
use nostr_indexeddb::WebDatabase;
use nostr_sdk::ClientBuilder;

#[derive(Debug, Clone)]
pub struct MultiClient {
    clients: HashMap<String, nostr_sdk::Client>,
}

impl Default for MultiClient {
    fn default() -> Self {
        Self::new()
    }
}

impl MultiClient {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: String, client: nostr_sdk::Client) {
        self.clients.insert(name, client);
    }

    pub fn get(&self, name: &str) -> Option<&nostr_sdk::Client> {
        //todo lazy init
        self.clients.get(name)
    }
    pub async fn get_or_create(&mut self, name: &str) -> Option<&nostr_sdk::Client> {
        // First, check if the client already exists with an immutable borrow
        if self.get(name).is_some() {
            return self.get(name);
        }
    
        // At this point, the client does not exist, and we can proceed with a mutable borrow
        {
            let cb_database_db = use_context::<Signal<CBWebDatabase>>();
            let db = WebDatabase::open("nostr-idb").await.unwrap();
            let client_builder = ClientBuilder::new().database(db);
            let client = client_builder.build();
            let cb_database_db_lock = cb_database_db.read();
            let relay_set_info = cb_database_db_lock.get_relay_set(name.to_string()).await.unwrap();
            client.add_relays(relay_set_info.relays).await.unwrap();
            client.connect().await;
            self.register(name.to_string(), client);
        }
    
        // Return the newly created client
        self.get(name)
    }
    
    
}
