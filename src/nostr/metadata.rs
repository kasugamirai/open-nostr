use std::str::FromStr;
use std::time::Duration;

use nostr_sdk::nips::nip19::Nip19;
use nostr_sdk::{client, Client, FromBech32, JsonUtil, PublicKey};
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

pub async fn get_profile(id: &str, client: Client) -> Result<Metadata, String> {
    let public_key: Option<PublicKey> = match Nip19::from_bech32(id) {
        Ok(val) => match val {
            Nip19::Pubkey(pubkey) => Some(pubkey),
            Nip19::Profile(profile) => Some(profile.public_key),
            _ => None,
        },
        Err(_) => match PublicKey::from_str(id) {
            Ok(val) => Some(val),
            Err(_) => None,
        },
    };

    if let Some(author) = public_key {
        let filter = Filter::new().author(author).kind(Kind::Metadata).limit(1);
        let query = client
            .get_events_of(vec![filter], Some(Duration::from_secs(10)))
            .await;

        if let Ok(events) = query {
            if let Some(event) = events.first() {
                if let Ok(metadata) = Metadata::from_json(&event.content) {
                    Ok(metadata)
                } else {
                    Err("Parse metadata failed".into())
                }
            } else {
                let rand_metadata = Metadata::new();
                Ok(rand_metadata)
            }
        } else {
            Err("Get metadata failed".into())
        }
    } else {
        Err("Public Key is not valid".into())
    }
}
