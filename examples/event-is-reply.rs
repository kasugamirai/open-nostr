// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2024 Rust Nostr Developers
// Distributed under the MIT software license

use std::time::Duration;

use async_utility::tokio;
use nostr_sdk::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();


    let client = Client::default();
    client.add_relay("wss://relay.damus.io").await?;

    client.connect().await;

    // Get events from all connected relays
    let filter = Filter::new().id(EventId::from_hex("0076792624df92e4b0892722c282fdeddd5912e89d61af843e180f2dc02a5530").unwrap());
    let events = client
        .get_events_of(vec![filter], Some(Duration::from_secs(10)))
        .await?;

    let event = &events[0];
    //todo

    println!("{events:#?}");

    Ok(())
}
