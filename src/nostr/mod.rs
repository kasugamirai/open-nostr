pub mod note;
pub mod utils;
use std::time::Duration;

use nostr_sdk::prelude::*;

pub async fn get_metadata(public_key: PublicKey) -> Result<Metadata> {
    let client = Client::default();

    client.add_relay("wss://nostr.oxtr.dev").await?;
    client.add_relay("wss://relay.damus.io").await?;
    client.add_relay("wss://nos.lol/").await?;
    client.add_relay("wss://nostr.wine").await?;

    client.connect().await;

    let filter = Filter::new().author(public_key).kind(Kind::Metadata);

    let events = client
        .get_events_from(
            ["wss://relay.damus.io", "wss://nos.lol/"],
            vec![filter],
            Some(Duration::from_secs(10)),
        )
        .await?;

    let _ = client.disconnect().await;

    let content = events[0].content.to_string();
    let metadata = Metadata::from_json(content).unwrap();

    Ok(metadata)
}
