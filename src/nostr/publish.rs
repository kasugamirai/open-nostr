use nostr_sdk::{self, Client, EventBuilder, NostrSigner, Tag, Timestamp};
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
) -> Result<(), Error> {
    let builder = EventBuilder::text_note(content, tags).custom_created_at(Timestamp::now());
    let event = signer.sign_event_builder(builder).await?;
    client.send_event(event).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use nostr_sdk::{FromBech32, Keys, SecretKey};
    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_publish_text_note() {
        let private_key = SecretKey::from_bech32(
            "nsec1qrypzwmxp8r54ctx2x7mhqzh5exca7xd8ssnlfup0js9l6pwku3qacq4u3",
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
}
