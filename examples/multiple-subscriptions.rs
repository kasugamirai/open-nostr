use async_utility::tokio;
use nostr_sdk::{bitcoin::XKeyIdentifier, prelude::*};
use std::{
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};
use tokio::spawn;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt::init();
    let dm_count = Arc::new(Mutex::new(0));
    let mut notification_handles = vec![];
    let mut send_handles = vec![];
    let publish_success_count = Arc::new(Mutex::new(0));
    let publish_failed_count: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let mut saved_keys = vec![];

    for i in 0..50 {
        let keys = Keys::generate();
        saved_keys.push(keys.clone());
        let public_key = keys.public_key();
        println!("Public key: {}", public_key);
        println!("Secret key: {}", keys.secret_key().unwrap());

        let client = Client::new(&keys);
        //client.add_relay("wss://nostr.oxtr.dev").await.unwrap();
        client.add_relay("ws://127.0.0.1:8080").await.unwrap();
        //client.add_relay("wss://relay.damus.io").await.unwrap();

        client.connect().await;

        let subscription: Filter = Filter::new()
            .author(public_key)
            .kind(Kind::EncryptedDirectMessage);

        //let success_count = Arc::clone(&publish_success_count);
        //let failed_count = Arc::clone(&publish_failed_count);

        let sub_id = client.subscribe(vec![subscription], None).await;
        println!("Subscription ID: {:?}, {}", sub_id.clone(), i + 1);

        let notification_handle: tokio::task::JoinHandle<()> = spawn({
            let dm_count = Arc::clone(&dm_count);
            async move {
                client
                    .handle_notifications(move |notification| {
                        handle_event(notification, sub_id.clone(), keys.clone(), dm_count.clone())
                    })
                    .await
                    .unwrap();
            }
        });
        notification_handles.push(notification_handle);
    }

    //for handle in notification_handles {
    //    handle.await.unwrap();
    //}

    println!("All subscriptions are done!");

    for key in saved_keys {
        let client = Client::new(key.clone());
        let public_key = key.public_key();
        //client.add_relay("wss://nostr.oxtr.dev").await.unwrap();
        client.add_relay("ws://127.0.0.1:8080").await.unwrap();
        //client.add_relay("wss://relay.damus.io").await.unwrap();
        client.connect().await;
        let kind = Kind::EncryptedDirectMessage;
        let msg = "Hello, World!";
        let content = nip04::encrypt(key.secret_key()?, &public_key, msg).unwrap();
        let tags = vec![Tag::public_key(public_key)];
        let builder = EventBuilder::new(kind, content, tags);
        let success_count = Arc::clone(&publish_success_count);
        let failed_count = Arc::clone(&publish_failed_count);

        let handle = spawn(async move {
            match client.send_event_builder(builder).await {
                Ok(_) => {
                    let mut success = success_count.lock().unwrap();
                    *success += 1;
                }
                Err(e) => {
                    dbg!("Failed to send event: {}", e);
                    let mut failed: std::sync::MutexGuard<'_, i32> = failed_count.lock().unwrap();
                    *failed += 1;
                }
            }
        });
        send_handles.push(handle);
    }
    for h in send_handles {
        h.await.unwrap();
    }

    println!("Success count: {}", *publish_success_count.lock().unwrap());
    println!("Failed count: {}", *publish_failed_count.lock().unwrap());

    for handle in notification_handles {
        handle.await.unwrap();
    }

    Ok(())
}

async fn handle_event(
    notification: RelayPoolNotification,
    sub_id_1: SubscriptionId,
    keys: Keys,
    dm_count: Arc<Mutex<i32>>,
) -> Result<bool, Box<dyn std::error::Error>> {
    if let RelayPoolNotification::Event {
        subscription_id,
        event,
        ..
    } = notification
    {
        if subscription_id == sub_id_1 && event.kind() == Kind::EncryptedDirectMessage {
            if let Ok(msg) = nip04::decrypt(keys.secret_key()?, event.author_ref(), event.content())
            {
                let mut count = dm_count.lock().unwrap();
                *count += 1;
                println!("DM: {msg}, {}", *count);
            } else {
                tracing::error!("Impossible to decrypt direct message");
            }
        } else if event.kind() == Kind::TextNote {
            println!("TextNote: {:?}", event);
        } else {
            println!("{:?}", event);
        }
    }
    Ok(false) // Set to true to exit from the loop
}
