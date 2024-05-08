use async_utility::tokio;
use capybastr::nostr::client::*;
use dioxus::html::p;
use nostr_sdk::prelude::*;
use std::sync::Arc;
use tokio::sync::Notify;

const BECH32_SK: &str = "nsec1przf9ascez0rty5yyflh5lk6hfu2pc0e2tyh8ed97esf25gg7zrsneae83";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up tracing and other initializations
    let (command_tx, command_rx) = tokio::sync::mpsc::channel(32);

    // Create a client
    let client = ClientBuilder::new()
        .opts(Options::new().wait_for_send(false))
        .build();
    client.add_relay("wss://relay.damus.io").await?;
    client.connect().await;

    let secret_key = SecretKey::from_bech32(BECH32_SK)?;
    let keys = Keys::new(secret_key);
    let public_key = keys.public_key();

    let filter_text_note = Filter::new()
        .author(public_key)
        .kind(Kind::TextNote)
        .since(Timestamp::now());

    // Spawn the client worker
    tokio::spawn(client_worker(client, "test", command_rx));

    // Use the channel to send commands to the worker
    command_tx
        .send(WorkerCommand::Subscribe {
            filters: vec![filter_text_note.clone()],
            handler: Arc::new(|notification| {
                Box::pin(async move {
                    println!("Received notification: {:?}", notification);
                    Ok(false)
                })
            }),
        })
        .await?;

    command_tx.send(WorkerCommand::Start).await?;

    println!("---------------------------------------------------------------");

    // add a command after start
    command_tx
        .send(WorkerCommand::Subscribe {
            filters: vec![filter_text_note.clone()],
            handler: Arc::new(|notification| {
                Box::pin(async move {
                    println!("Received notification222: {:?}", notification);
                    Ok(false)
                })
            }),
        })
        .await?;

    // Notify for controlled shutdown
    let notify = Notify::new();
    notify.notified().await;

    Ok(())
}
