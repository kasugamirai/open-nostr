use nostr_sdk::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub enum RegisterError {
    SubscriptionNotFound,
}

pub type NotificationHandler = Arc<
    dyn Fn(
            RelayPoolNotification,
        ) -> std::pin::Pin<
            Box<dyn std::future::Future<Output = Result<bool, Box<dyn std::error::Error>>> + Send>,
        > + Send
        + Sync,
>;

pub struct Register {
    handlers: Arc<Mutex<HashMap<SubscriptionId, NotificationHandler>>>,
}

impl Default for Register {
    fn default() -> Self {
        Self::new()
    }
}

impl Register {
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_subscription(
        &self,
        client: &Client,
        sub_id: SubscriptionId,
        filters: Vec<Filter>,
        handler: NotificationHandler,
        opts: Option<SubscribeAutoCloseOptions>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        client.subscribe_with_id(sub_id.clone(), filters, opts).await;
        let mut handlers = self.handlers.lock().await;
        handlers.insert(sub_id.clone(), handler);
        Ok(())
    }

    pub async fn remove_subscription(&self, sub_id: &SubscriptionId) {
        let mut handlers = self.handlers.lock().await;
        handlers.remove(sub_id);
    }

    async fn handle_notification(
        &self,
        notification: RelayPoolNotification,
    ) -> Result<bool, Box<dyn std::error::Error>> {
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

    pub async fn handle_notifications(
        &self,
        client: &Client,
    ) -> Result<(), Box<dyn std::error::Error>> {
        client
            .handle_notifications(|notification| {
                let register = self;
                async move { register.handle_notification(notification).await }
            })
            .await?;

        Ok(())
    }
}