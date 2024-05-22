use crate::init::NOSTR_DB_NAME;
use crate::store::CBWebDatabase;
use crate::store::CAPYBASTR_DBNAME;
use nostr_indexeddb::WebDatabase;
use nostr_sdk::ClientBuilder;
use std::collections::HashMap;
use std::sync::Arc;
use futures::lock::Mutex;

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

    pub async fn register(&self, name: String, client: nostr_sdk::Client) {
        let mut clients = self.clients.lock().await;
        clients.insert(name, client);
    }

    pub async fn get(&self, name: &str) -> Option<nostr_sdk::Client> {
        let clients = self.clients.lock().await;
        clients.get(name).cloned()
    }

    pub async fn get_or_create(&self, name: &str) -> nostr_sdk::Client {
        // Check if the client already exists
        if let Some(client) = self.get(name).await {
            return client;
        }

        // Create a new client if it doesn't exist
        let database = CBWebDatabase::open(CAPYBASTR_DBNAME).await.unwrap();
        let db = WebDatabase::open(NOSTR_DB_NAME).await.unwrap();
        let client_builder = ClientBuilder::new().database(db);
        let client: nostr_sdk::Client = client_builder.build();
        let relay_set_info = database.get_relay_set(name.to_string()).await.unwrap();
        client.add_relays(relay_set_info.relays).await.unwrap();

        // Insert the new client into the map
        self.register(name.to_string(), client.clone()).await;
        client
    }
}
