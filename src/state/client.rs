use std::collections::HashMap;

use nostr_sdk::{Client, Keys, NostrSigner};
use tracing::info;

use crate::CustomSub;

pub struct NostrClient {
    pub clients: HashMap<String, Client>,
}

impl NostrClient {
    pub fn create() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }

    pub async fn new(subscription: Vec<CustomSub>) -> Self {
        let mut nc = NostrClient::create();

        for i in subscription.iter() {
            nc.add(i).await;
        }

        nc
    }

    pub async fn add(&mut self, subscription: &CustomSub) {
        // TODO: get current user or empty user
        let pk = "nsec1dmvtj7uldpeethalp2ttwscy32jx36hr9jslskwdqreh2yk70anqhasx64";
        let my_keys = Keys::parse(pk).unwrap();
        let client = Client::new(&my_keys);

        for i in subscription.relay_set.relays.iter() {
            client.add_relay(i.clone().as_str()).await.unwrap();
        }
    }

    pub async fn remove(&mut self, name: String) {
        if let Some(client) = self.clients.remove(&name) {
            let _ = client.disconnect().await;
        }
    }

    pub async fn connect(&mut self, name: String) -> Result<(), String> {
        if let Some(client) = self.clients.get_mut(&name) {
            client.connect().await;
            Ok(())
        } else {
            Err(String::from("No such client"))
        }
    }

    pub async fn connect_all(&mut self) {
        for (_, client) in self.clients.iter_mut() {
            client.connect().await;
        }
    }

    pub async fn disconnect(&mut self, name: String) -> Result<(), String> {
        if let Some(client) = self.clients.get_mut(&name) {
            let _ = client.disconnect().await;
            Ok(())
        } else {
            Err(String::from("No such client"))
        }
    }

    pub async fn disconnect_all(&mut self) {
        for (_, client) in self.clients.iter_mut() {
            let _ = client.disconnect().await;
        }
    }
}
