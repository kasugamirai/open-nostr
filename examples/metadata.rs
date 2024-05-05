use std::time::Duration;

use nostr_sdk::prelude::*;

use async_utility::tokio;

pub async fn get_metadata(public_key: PublicKey) -> Result<Metadata> {
    let client = Client::default();

    client.add_relay("wss://nostr.oxtr.dev").await?;
    client.add_relay("wss://relay.damus.io").await?;
    client.add_relay("wss://nostr.mom").await?;
    client.add_relay("wss://nostr.wine").await?;

    client.connect().await;

    let filter = Filter::new().author(public_key).kind(Kind::Metadata);

    let events = client
        .get_events_from(
            ["wss://relay.damus.io", "wss://nostr.oxtr.dev"],
            vec![filter],
            Some(Duration::from_secs(10)),
        )
        .await?;

    let content = events[0].content.to_string();
    let metadata = Metadata::from_json(content).unwrap();
    println!("{:#?}", metadata);

    Ok(metadata)
}

#[tokio::main]
async fn main() -> Result<(), nostr_sdk::nips::nip19::Error> {
    tracing_subscriber::fmt::init();

    let public_key = "npub1dd668dyr9un9nzf9fjjkpdcqmge584c86gceu7j97nsp4lj2pscs0xk075";
    let public_key = PublicKey::from_bech32(public_key)?;
    get_metadata(public_key).await.unwrap();

    Ok(())
}
