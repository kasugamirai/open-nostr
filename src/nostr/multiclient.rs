use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::hash::{DefaultHasher, Hash, Hasher};
use nostr_sdk::client::Error;

#[derive(Debug, Clone)]
pub struct HashedClient {
    client: nostr_sdk::Client,
    hash: u64,
}

impl HashedClient {
    pub async fn new(client: nostr_sdk::Client) -> Self {
        let hash = Self::_hash(&client).await;
        Self { client, hash }
    }
    
    async fn _hash(client: &nostr_sdk::Client) -> u64 {
        let relays = client.relays().await;
        if relays.is_empty() {
            return 0;
        }
        let mut sorted_keys: Vec<_> = relays.keys().collect();
        sorted_keys.sort();
        let mut hasher = DefaultHasher::new();
        for key in sorted_keys {
            key.hash(&mut hasher);
        }
        hasher.finish()
    }

    pub fn client(&self) -> nostr_sdk::Client {
        self.client.clone()
    }

    pub fn hash(&self) -> u64 {
        self.hash
    }

    pub async fn add_relay(&mut self, url: &str) -> Result<bool, Error> {
        let result = self.client.add_relay(url).await?;
        if result {
            self.hash = Self::_hash(&self.client).await;
        }
        Ok(result)
    }

    pub async fn add_relays(&mut self, urls: Vec<&str>) -> Result<(), Error> {
        self.client.add_relays(urls).await?;
        self.hash = Self::_hash(&self.client).await;
        Ok(())
    }

    pub async fn remove_relay(&mut self, url: &str) -> Result<(), Error> {
        self.client.remove_relay(url).await?;
        self.hash = Self::_hash(&self.client).await;
        Ok(())
    }

    pub async fn remove_all_relays(&mut self) -> Result<(), Error> {
        self.client.remove_all_relays().await?;
        Ok(())
    }

}

#[derive(Debug, Clone)]
pub struct MultiClient {
    clients: Rc<RefCell<HashMap<String, HashedClient>>>, 
}

impl Default for MultiClient {
    fn default() -> Self {
        Self::new()
    }
}

impl MultiClient {
    pub fn new() -> Self {
        Self {
            clients: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn register(&self, name: String, hc: HashedClient) {
        let mut clients = self.clients.borrow_mut();
        clients.insert(name, hc);
    }

    pub fn get_client(&self, name: &str) -> Option<HashedClient> {
        let clients = self.clients.borrow();
        clients.get(name).cloned()
    }
}