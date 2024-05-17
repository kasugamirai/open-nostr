use async_std::sync::Mutex;
use nostr_sdk::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

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

#[derive(Clone)]
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
        client
            .subscribe_with_id(sub_id.clone(), filters, opts)
            .await;
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
        tracing::info!("Register::handle_notifications");
        client
            .handle_notifications(|notification| {
                let register = self;
                async move { register.handle_notification(notification).await }
            })
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use js_sys::Promise;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::{spawn_local, JsFuture};
    use wasm_bindgen_test::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = window, js_name = setTimeout)]
        fn set_timeout(closure: &Closure<dyn FnMut()>, time: u32) -> i32;
    }

    async fn sleep(ms: u32) -> Result<(), JsValue> {
        let promise = Promise::new(&mut |resolve, _| {
            let closure = Closure::once(move || {
                resolve.call0(&JsValue::NULL).unwrap();
            });
            set_timeout(&closure, ms);
            // Keep the closure alive until it's called
            closure.forget();
        });
        JsFuture::from(promise).await?;
        Ok(())
    }

    #[wasm_bindgen_test(async)]
    async fn test_sub_for_two_clients() {
        let timeout = Some(std::time::Duration::from_secs(5));
        let event_id1 =
            EventId::from_hex("770e3b604de378c67570ce3c521e2fd51c1a59aa85c22ef9aeab7b5f5e2f5e1b")
                .unwrap();
        let event_id2 =
            EventId::from_hex("70cfdf05fa80ce6b4a54668788eef31ff7d5a23b74f54943ec9e5a91cb5806f1")
                .unwrap();
        let client1 = Client::default();
        client1.add_relay("wss://nos.lol").await.unwrap();
        client1.connect().await;
        let client2 = Client::default();
        client2.add_relay("wss://nos.lol").await.unwrap();
        client2.connect().await;
        let console_log_handler: NotificationHandler = Arc::new(|notification| {
            Box::pin(async move {
                match notification {
                    RelayPoolNotification::Message {
                        message: RelayMessage::Event { event, .. },
                        ..
                    } => {
                        console_log!("event: {:?}", event);
                    }
                    _ => {}
                }
                Ok(false)
            })
        });

        let filter1 = Filter::new().id(event_id1).limit(1);
        let filter2: Filter = Filter::new().id(event_id2).limit(1);

        let register = Register::default();

        //add the subscription
        register
            .add_subscription(
                &client1,
                SubscriptionId::new("test1"),
                vec![filter1],
                console_log_handler.clone(),
                None,
            )
            .await
            .unwrap();

        //add another subscription
        register
            .add_subscription(
                &client2,
                SubscriptionId::new("test2"),
                vec![filter2],
                console_log_handler.clone(),
                None,
            )
            .await
            .unwrap();

        let r1 = register.clone();
        spawn_local(async move {
            r1.handle_notifications(&client1).await.unwrap();
        });

        let r2 = register.clone();
        spawn_local(async move {
            r2.handle_notifications(&client2).await.unwrap();
        });

        sleep(5000).await.unwrap();
    }
}
