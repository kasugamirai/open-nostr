// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2024 Rust Nostr Developers
// Distributed under the MIT software license

use std::time::Duration;

use async_utility::tokio;
use nostr_sdk::prelude::*;

//there are some reply ids
//1. current nip, with root and reply marker: e36817d0509cdd99d854391027bef6f3a0af1d87bdbdb1d9eb73201ff1719e09
//2. old nip, with no markers (this is a reply for reply): 0646ee437c5fc88d90a8c9b846edce3611e8a6e8545e952dbd7975f4a52925bb
//3. old nip, with no markers (a reply for root): 0076792624df92e4b0892722c282fdeddd5912e89d61af843e180f2dc02a5530

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let client = Client::default();
    client.add_relay("wss://relay.damus.io").await?;
    client.add_relay("wss://nostr.mo").await?;

    client.connect().await;

    // Get events from all connected relays
    let filter = Filter::new().id(EventId::from_hex(
        "e36817d0509cdd99d854391027bef6f3a0af1d87bdbdb1d9eb73201ff1719e09",
    )
    .unwrap());
    let events = client
        .get_events_of(vec![filter], Some(Duration::from_secs(10)))
        .await?;

    let event = &events[0];
    let etags: Vec<&Tag> = event
        .iter_tags()
        .filter(|t| {
            matches!(
                t,
                nostr::Tag::Event {
                    event_id: _,
                    relay_url: _,
                    marker: _
                }
            )
        })
        .collect();

    println!("{:?}", etags);

    Ok(())
}
