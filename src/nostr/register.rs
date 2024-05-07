use nostr_sdk::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use lazy_static::lazy_static;

pub type NotificationHandler = Arc<
    dyn Fn(RelayPoolNotification) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<bool, Box<dyn std::error::Error>>> + Send>>
    + Send
    + Sync,
>;

pub struct Register {
    subscriptions: Arc<Mutex<HashMap<SubscriptionId, String>>>,
    handlers: Arc<Mutex<HashMap<SubscriptionId, NotificationHandler>>>,
}

impl Register {
    fn new() -> Self {
        Self {
            subscriptions: Arc::new(Mutex::new(HashMap::new())),
            handlers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_subscription(
        &self,
        client: &Client,
        client_key: &str,
        sub_id: Option<SubscriptionId>,
        filters: Vec<Filter>,
        handler: NotificationHandler,
        opts: Option<SubscribeAutoCloseOptions>
    ) -> Result<SubscriptionId, Box<dyn std::error::Error>> {
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

    async fn handle_notification(
        &self,
        notification: RelayPoolNotification,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        if let RelayPoolNotification::Message { message: RelayMessage::Event { subscription_id, .. }, .. } = &notification {
            let handlers = self.handlers.lock().await;
            if let Some(handler) = handlers.get(subscription_id) {
                return (handler)(notification.clone()).await;
            }
        }
        Ok(false)
    }

    pub async fn handle_notifications(&self, client: &Client) -> Result<(), Box<dyn std::error::Error>> {
        client.handle_notifications(|notification| {
            let register = self;
            async move {
                register.handle_notification(notification).await
            }
        }).await?;

        Ok(())
    }
}

// Static Register instance
lazy_static! {
    pub static ref SUB_REGISTER: Register = Register::new();
}