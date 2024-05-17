use std::collections::HashMap;
use crate::store::CAPYBASTR_DBNAME;
use crate::store::CBWebDatabase;
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
    pub async fn get_or_create(&mut self, name: &str) -> &nostr_sdk::Client {
        let database = CBWebDatabase::open(CAPYBASTR_DBNAME).await.unwrap();
        let db = WebDatabase::open("nostr-idb").await.unwrap();
        let client_builder = ClientBuilder::new().database(db);
        let client = client_builder.build();
        let relay_set_info = database.get_relay_set(name.to_string()).await.unwrap();
        client.add_relays(relay_set_info.relays).await.unwrap();
        self.register(name.to_string(), client);
        self.get(name).unwrap()
    }
}
