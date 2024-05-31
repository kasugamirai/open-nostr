use nostr_sdk::{
    self, Client, Event, EventBuilder, EventId, Metadata, NostrSigner, Tag, Timestamp, UncheckedUrl,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Client(#[from] nostr_sdk::client::Error),
    #[error(transparent)]
    Singer(#[from] nostr_sdk::signer::Error),
}

pub async fn publish_text_note(
    client: &Client,
    signer: NostrSigner,
    content: &str,
    tags: Vec<Tag>,
) -> Result<EventId, Error> {
    let builder = EventBuilder::text_note(content, tags).custom_created_at(Timestamp::now());
    let event = signer.sign_event_builder(builder).await?;
    let eid = client.send_event(event).await?;
    Ok(eid)
}

pub async fn repost(
    client: &Client,
    signer: NostrSigner,
    event: &Event,
    url: Option<UncheckedUrl>,
) -> Result<EventId, Error> {
    let builder = EventBuilder::repost(event, url);
    let event = signer.sign_event_builder(builder).await?;
    let eid = client.send_event(event).await?;
    Ok(eid)
}

pub async fn reaction(
    client: &Client,
    signer: NostrSigner,
    event: &Event,
    reaction: &str,
) -> Result<EventId, Error> {
    let builder = EventBuilder::reaction(event, reaction);
    let event = signer.sign_event_builder(builder).await?;
    let eid = client.send_event(event).await?;
    Ok(eid)
}

pub async fn new_channel(
    client: &Client,
    signer: NostrSigner,
    metadata: &Metadata,
) -> Result<EventId, Error> {
    let builder = EventBuilder::channel(metadata);
    let event = signer.sign_event_builder(builder).await?;
    let eid = client.send_event(event).await?;
    Ok(eid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nostr_sdk::{EventId, Filter, FromBech32, Keys, SecretKey, ToBech32};
    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_publish_text_note() {
        let private_key = SecretKey::from_bech32(
            "nsec19y0u0kgzwx4ygxpk04ktl6uc2daq2mts9w0rk2qrxnru5hhvpjeq20awgp",
        )
        .unwrap();
        let key = Keys::new(private_key);
        let signer = key.into();
        let client = Client::default();
        client.add_relay("wss://relay.damus.io").await.unwrap();
        client.connect().await;
        let result = publish_text_note(&client, signer, "Hello, world!", vec![]).await;
        assert!(result.is_ok());
    }

    #[wasm_bindgen_test]
    async fn test_repost() {
        let private_key = SecretKey::from_bech32(
            "nsec1qrypzwmxp8r54ctx2x7mhqzh5exca7xd8ssnlfup0js9l6pwku3qacq4u3",
        )
        .unwrap();
        let key = Keys::new(private_key);
        let signer = key.into();
        let client = Client::default();
        client.add_relay("wss://relay.damus.io").await.unwrap();
        let eid =
            EventId::from_bech32("note1zlsz37aggmsc2nfzqjdsdw77qwyfqm3erxag5f75nz8tndkvs0uqllhywm")
                .unwrap();
        let f = Filter::new().event(eid);
        client.connect().await;
        let event = client.get_events_of(vec![f], None).await.unwrap();
        //let url = UncheckedUrl::from("wss://relay.damus.io");
        let result = repost(&client, signer, &event[0], None).await;
        assert!(result.is_ok());
    }

    #[wasm_bindgen_test]
    async fn test_reaction() {
        let private_key = SecretKey::from_bech32(
            "nsec1qrypzwmxp8r54ctx2x7mhqzh5exca7xd8ssnlfup0js9l6pwku3qacq4u3",
        )
        .unwrap();
        let key = Keys::new(private_key);
        let signer = key.into();
        let client = Client::default();
        client.add_relay("wss://relay.damus.io").await.unwrap();
        let eid =
            EventId::from_bech32("note1zlsz37aggmsc2nfzqjdsdw77qwyfqm3erxag5f75nz8tndkvs0uqllhywm")
                .unwrap();
        let f = Filter::new().event(eid);
        client.connect().await;
        let event = client.get_events_of(vec![f], None).await.unwrap();
        let result = reaction(&client, signer, &event[0], "👍").await;
        if let Ok(event_id) = &result {
            console_log!("Event ID: {:?}", event_id.to_bech32());
        } else if let Err(e) = &result {
            console_log!("Error: {:?}", e);
        }
        assert!(result.is_ok());
    }

    #[wasm_bindgen_test]
    async fn test_new_channel() {
        let private_key = SecretKey::from_bech32(
            "nsec1qrypzwmxp8r54ctx2x7mhqzh5exca7xd8ssnlfup0js9l6pwku3qacq4u3",
        )
        .unwrap();
        let key = Keys::new(private_key);
        let signer = key.into();
        let client = Client::default();
        client.add_relay("wss://relay.damus.io").await.unwrap();
        let metadata = Metadata::new();
        client.connect().await;
        let result = new_channel(&client, signer, &metadata).await;
        assert!(result.is_ok());
    }
}
