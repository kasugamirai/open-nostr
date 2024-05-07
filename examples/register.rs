use anyhow::Result;
use capybastr::nostr::register::*;
use nostr_sdk::prelude::*;
//use std::error::Error as StdError;
use std::sync::Arc;

const BECH32_SK: &str = "nsec1przf9ascez0rty5yyflh5lk6hfu2pc0e2tyh8ed97esf25gg7zrsneae83";

async fn handler_text_note(notification: RelayPoolNotification) -> Result<bool> {
    if let RelayPoolNotification::Message {
        message: RelayMessage::Event { event, .. },
        ..
    } = notification
    {
        println!("TextNote: {:?}", event);
    }
    Ok(false)
}

async fn handler_text_note2(notification: RelayPoolNotification) -> Result<bool> {
    if let RelayPoolNotification::Message {
        message: RelayMessage::Event { event, .. },
        ..
    } = notification
    {
        println!("TextNote again !!!: {:?}", event);
    }
    Ok(false)
}

async fn handler_repost(notification: RelayPoolNotification) -> Result<bool> {
    if let RelayPoolNotification::Message {
        message: RelayMessage::Event { event, .. },
        ..
    } = notification
    {
        println!("Repost: {:?}", event);
    }
    Ok(false)
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let secret_key = SecretKey::from_bech32(BECH32_SK)?;
    let keys = Keys::new(secret_key);
    let public_key = keys.public_key();

    let registry = &*SUB_REIGISTER;

    let client_for_text_note = ClientBuilder::new()
        .opts(Options::new().wait_for_send(false))
        .build();
    client_for_text_note
        .add_relay("wss://relay.damus.io")
        .await?;
    client_for_text_note.connect().await;

    let client_for_repost = ClientBuilder::new()
        .opts(Options::new().wait_for_send(false))
        .build();
    client_for_repost.add_relay("wss://relay.damus.io").await?;
    client_for_repost.connect().await;

    registry
        .add_client(String::from("client_for_text_note"), client_for_text_note)
        .await;
    registry
        .add_client(String::from("client_for_repost"), client_for_repost)
        .await;

    let filter_text_note = Filter::new()
        .author(public_key)
        .kind(Kind::TextNote)
        .since(Timestamp::now());

    let filter_repost: Filter = Filter::new()
        .author(public_key)
        .kind(Kind::Repost)
        .since(Timestamp::now());

    registry
        .add_subscription(
            "client_for_text_note",
            None,
            vec![filter_text_note.clone()],
            Arc::new(|notification| Box::pin(handler_text_note(notification))),
            None,
        )
        .await?;

    registry
        .add_subscription(
            "client_for_text_note",
            None,
            vec![filter_text_note],
            Arc::new(|notification| Box::pin(handler_text_note2(notification))),
            None,
        )
        .await?;

    registry
        .add_subscription(
            "client_for_repost",
            None,
            vec![filter_repost],
            Arc::new(|notification| Box::pin(handler_repost(notification))),
            None,
        )
        .await?;

    // Handle subscription notifications with `handle_notifications` method
    let handle1 = tokio::spawn(async move {
        registry
            .handle_notifications("client_for_text_note")
            .await
            .unwrap();
    });

    let handle2 = tokio::spawn(async move {
        registry
            .handle_notifications("client_for_repost")
            .await
            .unwrap();
    });

    handle1.await?;
    handle2.await?;

    Ok(())
}
