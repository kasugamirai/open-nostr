// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2024 Rust Nostr Developers
// Distributed under the MIT software license

use async_utility::tokio;
use nostr_sdk::prelude::*;

const BECH32_SK: &str = "nsec1przf9ascez0rty5yyflh5lk6hfu2pc0e2tyh8ed97esf25gg7zrsneae83";

#[tokio::main]
async fn main() -> Result<()> {

    let secret_key = SecretKey::from_bech32(BECH32_SK)?;
    let my_keys = Keys::new(secret_key);

    let client = Client::new(&my_keys);
    client.add_relay("wss://nos.lol").await?;
    client.add_relay("wss://offchain.pub").await?;

    client.connect().await;

    // Publish a text note
    client.publish_text_note("Hello world", []).await?;

    // Create a text note POW event and broadcast to all connected relays
    let event: Event =
        EventBuilder::text_note("POW text note from nostr-sdk", []).to_pow_event(&my_keys, 20)?;
    client.send_event(event).await?;

    // Send multiple events at once (to all relays)
    let mut events: Vec<Event> = Vec::new();
    for i in 0..10 {
        events.push(EventBuilder::text_note(format!("Event #{i}"), []).to_event(&my_keys)?);
    }
    let opts = RelaySendOptions::default();
    client.batch_event(events, opts).await?;

    // Send event to specific relays
    let event: Event = EventBuilder::text_note("POW text note from nostr-sdk 16", [])
        .to_pow_event(&my_keys, 16)?;
    client
        .send_event_to(["wss://offchain.pub", "wss://nos.lol"], event)
        .await?;

    Ok(())
}
