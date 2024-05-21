use crate::store::CBWebDatabase;
use crate::store::CAPYBASTR_DBNAME;
use nostr_indexeddb::WebDatabase;
use nostr_sdk::ClientBuilder;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct MultiClient {
    clients: Arc<Mutex<HashMap<String, nostr_sdk::Client>>>,
}

impl Default for MultiClient {
    fn default() -> Self {
        Self::new()
    }
}

impl MultiClient {
    pub fn new() -> Self {
        Self {
            clients: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register(&self, name: String, client: nostr_sdk::Client) {
        let mut clients = self.clients.lock().unwrap();
        clients.insert(name, client);
    }

    pub fn get(&self, name: &str) -> Option<nostr_sdk::Client> {
        let clients = self.clients.lock().unwrap();
        clients.get(name).cloned()
    }

    pub async fn get_or_create(&self, name: &str) -> nostr_sdk::Client {
        let mut clients = self.clients.lock().unwrap();
        if let Some(client) = clients.get(name) {
            return client.clone();
        }

        drop(clients); // Release the lock before awaiting

        let database = CBWebDatabase::open(CAPYBASTR_DBNAME).await.unwrap();
        let db = WebDatabase::open(CAPYBASTR_DBNAME).await.unwrap();
        let client_builder = ClientBuilder::new().database(db);
        let client = client_builder.build();
        let relay_set_info = database.get_relay_set(name.to_string()).await.unwrap();
        client.add_relays(relay_set_info.relays).await.unwrap();

        // Acquire the lock again to update the map
        let mut clients = self.clients.lock().unwrap();
        clients.insert(name.to_string(), client.clone());
        client
    }
}
