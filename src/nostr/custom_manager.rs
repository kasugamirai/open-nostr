use nostr_sdk::Client;
use std::collections::HashMap;
//use std::sync::Arc;
use nostr_sdk::Filter;
use nostr_sdk::Result;
use nostr_sdk::SubscribeAutoCloseOptions;
use nostr_sdk::SubscriptionId;
use tokio::sync::Mutex;

pub struct ClientManager {
    clients: Mutex<HashMap<String, Client>>,
}

impl Default for ClientManager {
    fn default() -> Self {
        Self {
            clients: Mutex::new(HashMap::new()),
        }
    }
}

impl ClientManager {
    pub async fn add_client(&self, name: String, client: Client) {
        let mut clients = self.clients.lock().await;
        clients.insert(name, client);
    }

    pub async fn get_client(&self, name: &str) -> Option<Client> {
        let clients = self.clients.lock().await;
        clients.get(name).cloned()
    }

    pub async fn remove_client(&self, name: &str) {
        let mut clients = self.clients.lock().await;
        clients.remove(name);
    }

    pub async fn add_subscription(
        &self,
        name: &str,
        filters: Vec<Filter>,
        opt: Option<SubscribeAutoCloseOptions>,
    ) -> Result<SubscriptionId> {
        if let Some(client) = self.get_client(name).await {
            let sub_id = client.subscribe(filters, opt).await;
            Ok(sub_id)
        } else {
            Err("Client not found".into())
        }
    }

    pub async fn remove_subscription(&self, name: &str, subscription_id: SubscriptionId) {
        if let Some(client) = self.get_client(name).await {
            client.unsubscribe(subscription_id).await;
        }
    }

    pub async fn update_subscription(
        &self,
        name: &str,
        subscription_id: SubscriptionId,
        filters: Vec<Filter>,
        opt: Option<SubscribeAutoCloseOptions>,
    ) {
        if let Some(client) = self.get_client(name).await {
            client
                .subscribe_with_id(subscription_id, filters, opt)
                .await;
        }
    }

    pub async fn handle_notifications(&self, name: &str) {
        if let Some(client) = self.get_client(name).await {}
    }

    pub async fn update_handle_notifications(&self, name: &str) {
        if let Some(client) = self.get_client(name).await {}
    }

    pub async fn del_handle_notifications(&self, name: &str) {
        if let Some(client) = self.get_client(name).await {}
    }
}
