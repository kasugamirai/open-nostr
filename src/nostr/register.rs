use anyhow::anyhow;
use anyhow::Result;
use lazy_static::lazy_static;
use nostr_sdk::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

type NotificationHandler = Arc<
    dyn Fn(
            RelayPoolNotification,
        )
            -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<bool>> + Send>>
        + Send
        + Sync,
>;
pub struct Register {
    clients: Arc<Mutex<HashMap<String, Client>>>,
    subscriptions: Arc<Mutex<HashMap<SubscriptionId, String>>>,
    handlers: Arc<Mutex<HashMap<SubscriptionId, NotificationHandler>>>,
}

impl Register {
    fn new() -> Self {
        Self {
            clients: Arc::new(Mutex::new(HashMap::new())),
            subscriptions: Arc::new(Mutex::new(HashMap::new())),
            handlers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_client(&self, key: String, client: Client) {
        let mut clients = self.clients.lock().await;
        clients.insert(key, client);
    }

    pub async fn get_client(&self, key: &str) -> Option<Client> {
        let clients = self.clients.lock().await;
        clients.get(key).cloned()
    }

    pub async fn remove_client(&self, key: &str) {
        let mut clients = self.clients.lock().await;
        clients.remove(key);

        let mut subscriptions = self.subscriptions.lock().await;
        let mut handlers = self.handlers.lock().await;

        // Remove associated subscriptions and handlers
        subscriptions.retain(|_, client_key| client_key != key);
        handlers.retain(|sub_id, _| !subscriptions.contains_key(sub_id));
    }

    pub async fn add_subscription(
        &self,
        client_key: &str,
        sub_id: Option<SubscriptionId>,
        filters: Vec<Filter>,
        handler: NotificationHandler,
        opts: Option<SubscribeAutoCloseOptions>,
    ) -> Result<SubscriptionId> {
        let clients = self.clients.lock().await;
        let client = clients
            .get(client_key)
            .ok_or_else(|| anyhow!("Client not found for key: {}", client_key))?;

        let subscription_id = if let Some(id) = sub_id {
            client.subscribe_with_id(id.clone(), filters, opts).await;
            id
        } else {
            client.subscribe(filters, opts).await
        };

        let mut subscriptions = self.subscriptions.lock().await;
        subscriptions.insert(subscription_id.clone(), client_key.to_string());

        let mut handlers = self.handlers.lock().await;
        handlers.insert(subscription_id.clone(), handler);

        Ok(subscription_id)
    }

    pub async fn remove_subscription(&self, sub_id: &SubscriptionId) {
        let mut subscriptions = self.subscriptions.lock().await;
        subscriptions.remove(sub_id);

        let mut handlers = self.handlers.lock().await;
        handlers.remove(sub_id);
    }

    async fn handle_notification(&self, notification: RelayPoolNotification) -> Result<bool> {
        if let RelayPoolNotification::Message {
            message: RelayMessage::Event {
                subscription_id, ..
            },
            ..
        } = &notification
        {
            let handlers = self.handlers.lock().await;
            if let Some(handler) = handlers.get(subscription_id) {
                return (handler)(notification.clone()).await;
            }
        }
        Ok(false)
    }

    pub async fn handle_notifications(&self, client_key: &str) -> Result<()> {
        let client = self
            .get_client(client_key)
            .await
            .ok_or_else(|| anyhow!("Client not found for key: {}", client_key))?;

        client
            .handle_notifications(|notification| {
                let register = self;
                async move {
                    register
                        .handle_notification(notification)
                        .await
                        .map_err(|e| e.into())
                }
            })
            .await?;

        Ok(())
    }
}

// Singleton instance
lazy_static! {
    pub static ref SUB_REIGISTER: Register = Register::new();
}
