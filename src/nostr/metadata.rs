use std::time::Duration;

use nostr_sdk::{client, Client, JsonUtil};
use nostr_sdk::{event, Metadata};
use nostr_sdk::{Filter, Kind};

pub async fn get_current_user_profile(client: Client) -> Result<Metadata, String> {
    let signer = client.signer().await.unwrap();
    let public_key = signer.public_key().await.unwrap();
    let filter = Filter::new()
        .author(public_key)
        .kind(Kind::Metadata)
        .limit(1);

    match client
        .get_events_of(vec![filter], Some(Duration::from_secs(10)))
        .await
    {
        Ok(events) => {
            if let Some(event) = events.first() {
                if let Ok(metadata) = Metadata::from_json(&event.content) {
                    Ok(metadata)
                } else {
                    Err("Parse metadata failed".into())
                }
            } else {
                Err("Not found".into())
            }
        }
        Err(_) => Err("Not found".into()),
    }
}
