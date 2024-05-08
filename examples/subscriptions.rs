use async_utility::tokio;
use nostr_sdk::prelude::*;
use std::io::{self, Read};

const BECH32_SK: &str = "nsec1przf9ascez0rty5yyflh5lk6hfu2pc0e2tyh8ed97esf25gg7zrsneae83";

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let my_pub: &str = "npub1q0uulk2ga9dwkp8hsquzx38hc88uqggdntelgqrtkm29r3ass6fq8y9py9";

    let secret_key = SecretKey::from_bech32(BECH32_SK)?;
    let keys = Keys::new(secret_key);
    let public_key = keys.public_key();
    let my_publicKey = PublicKey::from_bech32(my_pub)?;

    let opts = Options::new().wait_for_send(false);
    let client = ClientBuilder::new().opts(opts).build();

    client.add_relay("wss://nostr.oxtr.dev").await?;
    client.add_relay("wss://relay.damus.io").await?;
    client.add_relay("wss://nostr.openchain.fr").await?;

    client.connect().await;

    let subscription = Filter::new()
        .author(public_key)
        .kind(Kind::Metadata)
        .since(Timestamp::now());

    let sub_id_1 = client.subscribe(vec![subscription], None).await;

    let sub_id_2 = SubscriptionId::new("other-id");
    let filter = Filter::new()
        .author(public_key)
        .kind(Kind::TextNote)
        .since(Timestamp::now());
    client
        .subscribe_with_id(sub_id_2.clone(), vec![filter], None)
        .await;

    let filter = Filter::new()
        .author(public_key)
        .kind(Kind::EncryptedDirectMessage)
        .since(Timestamp::now());
    client
        .subscribe_with_id(sub_id_1.clone(), vec![filter], None)
        .await;

    // Spawn a new asynchronous task to handle notifications
    let client_clone = client.clone();
    tokio::spawn(async move {
        client_clone
            .handle_notifications(|notification| async {
                if let RelayPoolNotification::Event {
                    subscription_id,
                    event,
                    ..
                } = notification
                {
                    // Check subscription ID
                    if subscription_id == sub_id_1 {
                        // Handle
                    }

                    if subscription_id == sub_id_2 {
                        // Handle
                    }

                    // Check kind
                    if event.kind() == Kind::EncryptedDirectMessage {
                        if let Ok(msg) =
                            nip04::decrypt(keys.secret_key()?, event.author_ref(), event.content())
                        {
                            println!("DM: {msg}");
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
            })
            .await
            .unwrap();
    });

    let sub_id_3 = SubscriptionId::new("other-idsaa");
    let filter = Filter::new()
        .author(my_publicKey)
        .kind(Kind::TextNote)
        .since(Timestamp::now());
    client.subscribe_with_id(sub_id_3, vec![filter], None).await;

    // Wait for user input to exit
    println!("Press any key to exit...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    Ok(())
}
