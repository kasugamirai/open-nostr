use nostr_sdk::{
    nips::{nip65::RelayMetadata, nip94::FileMetadata},
    Client, Contact, Event, EventBuilder, EventId, Metadata, NostrSigner, PublicKey, Tag,
    Timestamp, UncheckedUrl, Url,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Client(#[from] nostr_sdk::client::Error),
    #[error(transparent)]
    Signer(#[from] nostr_sdk::signer::Error),
}

macro_rules! sign_and_send_event {
    ($client:expr, $signer:expr, $builder:expr) => {{
        let event = $signer.sign_event_builder($builder).await?;
        let eid = $client.send_event(event).await?;
        Ok(eid)
    }};
}

pub async fn publish_text_note(
    client: &Client,
    signer: &NostrSigner,
    content: &str,
    tags: Vec<Tag>,
) -> Result<EventId, Error> {
    let builder = EventBuilder::text_note(content, tags).custom_created_at(Timestamp::now());
    sign_and_send_event!(client, signer, builder)
}

pub async fn repost(
    client: &Client,
    signer: &NostrSigner,
    event: &Event,
    url: Option<UncheckedUrl>,
) -> Result<EventId, Error> {
    let builder = EventBuilder::repost(event, url);
    sign_and_send_event!(client, signer, builder)
}

pub async fn reaction(
    client: &Client,
    signer: &NostrSigner,
    event: &Event,
    reaction: &str,
) -> Result<EventId, Error> {
    let builder = EventBuilder::reaction(event, reaction);
    sign_and_send_event!(client, signer, builder)
}

pub async fn new_channel(
    client: &Client,
    signer: &NostrSigner,
    metadata: &Metadata,
) -> Result<EventId, Error> {
    let builder = EventBuilder::channel(metadata);
    sign_and_send_event!(client, signer, builder)
}

pub async fn set_channel_metadata(
    client: &Client,
    signer: &NostrSigner,
    channel_id: EventId,
    metadata: &Metadata,
    url: Option<Url>,
) -> Result<EventId, Error> {
    let builder = EventBuilder::channel_metadata(channel_id, url, metadata);
    sign_and_send_event!(client, signer, builder)
}

pub async fn send_channel_msg(
    client: &Client,
    signer: &NostrSigner,
    channel_id: EventId,
    msg: &str,
    relay_url: Url,
) -> Result<EventId, Error> {
    let builder = EventBuilder::channel_msg(channel_id, relay_url, msg);
    sign_and_send_event!(client, signer, builder)
}

pub async fn file_metadata(
    client: &Client,
    signer: &NostrSigner,
    metadata: FileMetadata,
    description: &str,
) -> Result<EventId, Error> {
    let builder = EventBuilder::file_metadata(description, metadata);
    sign_and_send_event!(client, signer, builder)
}

pub async fn send_private_msg(
    client: &Client,
    signer: &NostrSigner,
    receiver: PublicKey,
    message: &str,
    reply_to: Option<EventId>,
) -> Result<EventId, Error> {
    let builder = EventBuilder::private_msg_rumor(receiver, message, reply_to);
    sign_and_send_event!(client, signer, builder)
}

pub async fn delete_event(
    client: &Client,
    signer: &NostrSigner,
    event_ids: Vec<EventId>,
) -> Result<EventId, Error> {
    let builder = EventBuilder::delete(event_ids);
    sign_and_send_event!(client, signer, builder)
}

pub async fn set_relay_list(
    client: &Client,
    signer: &NostrSigner,
    relays: Vec<(Url, Option<RelayMetadata>)>,
) -> Result<EventId, Error> {
    let builder = EventBuilder::relay_list(relays);
    sign_and_send_event!(client, signer, builder)
}

pub async fn set_contact_list(
    client: &Client,
    signer: &NostrSigner,
    contacts: Vec<Contact>,
) -> Result<EventId, Error> {
    let builder = EventBuilder::contact_list(contacts);
    sign_and_send_event!(client, signer, builder)
}

#[cfg(test)]
mod tests {
    use nostr_sdk::bitcoin::hashes::sha256::Hash as Sha256Hash;
    use std::str::FromStr;

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
        let signer = &key.into();
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
        let signer = &key.into();
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
        let signer = &key.into();
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
        let signer = &key.into();
        let client = Client::default();
        client.add_relay("wss://relay.damus.io").await.unwrap();
        let metadata = Metadata::new();
        client.connect().await;
        let result = new_channel(&client, signer, &metadata).await;
        assert!(result.is_ok());
    }

    #[wasm_bindgen_test]
    async fn test_set_channel_metadata() {
        let private_key = SecretKey::from_bech32(
            "nsec1qrypzwmxp8r54ctx2x7mhqzh5exca7xd8ssnlfup0js9l6pwku3qacq4u3",
        )
        .unwrap();
        let key = Keys::new(private_key);
        let signer = &key.into();
        let client = Client::default();
        client.add_relay("wss://relay.damus.io").await.unwrap();
        let metadata = Metadata::new();
        let channel_id =
            EventId::from_bech32("note1zlsz37aggmsc2nfzqjdsdw77qwyfqm3erxag5f75nz8tndkvs0uqllhywm")
                .unwrap();
        client.connect().await;
        let result = set_channel_metadata(&client, signer, channel_id, &metadata, None).await;
        assert!(result.is_ok());
    }

    #[wasm_bindgen_test]
    async fn test_send_channel_msg() {
        let private_key = SecretKey::from_bech32(
            "nsec1qrypzwmxp8r54ctx2x7mhqzh5exca7xd8ssnlfup0js9l6pwku3qacq4u3",
        )
        .unwrap();
        let key = Keys::new(private_key);
        let signer = &key.into();
        let client = Client::default();
        client.add_relay("wss://relay.damus.io").await.unwrap();
        let channel_id =
            EventId::from_bech32("note1zlsz37aggmsc2nfzqjdsdw77qwyfqm3erxag5f75nz8tndkvs0uqllhywm")
                .unwrap();
        let url = Url::parse("wss://relay.damus.io").unwrap();
        client.connect().await;
        let result = send_channel_msg(&client, signer, channel_id, "Hello, world!", url).await;
        assert!(result.is_ok());
    }

    #[wasm_bindgen_test]
    async fn test_file_metadata() {
        const IMAGE_URL: &str = "https://image.nostr.build/99a95fcb4b7a2591ad32467032c52a62d90a204d3b176bc2459ad7427a3f2b89.jpg";
        const IMAGE_HASH: &str = "1aea8e98e0e5d969b7124f553b88dfae47d1f00472ea8c0dbf4ac4577d39ef02";
        let url = Url::parse(IMAGE_URL).unwrap();
        let hash = Sha256Hash::from_str(IMAGE_HASH).unwrap();
        let private_key = SecretKey::from_bech32(
            "nsec1qrypzwmxp8r54ctx2x7mhqzh5exca7xd8ssnlfup0js9l6pwku3qacq4u3",
        )
        .unwrap();
        let key = Keys::new(private_key);
        let signer = &key.into();
        let client = Client::default();
        client.add_relay("wss://relay.damus.io").await.unwrap();
        let metadata = FileMetadata::new(url, "image/jpeg", hash);
        client.connect().await;
        let result = file_metadata(&client, signer, metadata, "Hello, world!").await;
        assert!(result.is_ok());
    }

    #[wasm_bindgen_test]
    async fn test_send_private_msg() {
        let private_key = SecretKey::from_bech32(
            "nsec1qrypzwmxp8r54ctx2x7mhqzh5exca7xd8ssnlfup0js9l6pwku3qacq4u3",
        )
        .unwrap();
        let key = Keys::new(private_key);
        let signer = &key.into();
        let client = Client::default();
        client.add_relay("wss://relay.damus.io").await.unwrap();
        let receiver = PublicKey::from_bech32(
            "npub1awsnqr5338h497yam5m9hrgh9535yadj9zxglwk55xpsdtsn2c4syjruew",
        )
        .unwrap();
        client.connect().await;
        let result = send_private_msg(&client, signer, receiver, "Hello, world!", None).await;
        assert!(result.is_ok());
    }

    #[wasm_bindgen_test]
    async fn test_delete_event() {
        let private_key = SecretKey::from_bech32(
            "nsec1qrypzwmxp8r54ctx2x7mhqzh5exca7xd8ssnlfup0js9l6pwku3qacq4u3",
        )
        .unwrap();
        let key = Keys::new(private_key);
        let signer = &key.into();
        let client = Client::default();
        client.add_relay("wss://relay.damus.io").await.unwrap();
        let event_id =
            EventId::from_bech32("note1zlsz37aggmsc2nfzqjdsdw77qwyfqm3erxag5f75nz8tndkvs0uqllhywm")
                .unwrap();
        client.connect().await;
        let result = delete_event(&client, signer, vec![event_id]).await;
        assert!(result.is_ok());
    }

    #[wasm_bindgen_test]
    async fn test_set_relay_list() {
        let private_key = SecretKey::from_bech32(
            "nsec1qrypzwmxp8r54ctx2x7mhqzh5exca7xd8ssnlfup0js9l6pwku3qacq4u3",
        )
        .unwrap();
        let key = Keys::new(private_key);
        let signer = &key.into();
        let client = Client::default();
        client.add_relay("wss://relay.damus.io").await.unwrap();
        let relays = vec![
            (
                Url::parse("wss://relay.damus.io").unwrap(),
                Some(RelayMetadata::Read),
            ),
            (
                Url::parse("wss://relay.damus.io").unwrap(),
                Some(RelayMetadata::Write),
            ),
        ];
        client.connect().await;
        let result = set_relay_list(&client, signer, relays).await;
        assert!(result.is_ok());
    }

    #[wasm_bindgen_test]
    async fn test_set_contact_list() {
        let private_key = SecretKey::from_bech32(
            "nsec1qrypzwmxp8r54ctx2x7mhqzh5exca7xd8ssnlfup0js9l6pwku3qacq4u3",
        )
        .unwrap();
        let key = Keys::new(private_key);
        let signer = &key.into();
        let client = Client::default();
        client.add_relay("wss://relay.damus.io").await.unwrap();
        let contacts = vec![Contact::new(
            PublicKey::from_bech32(
                "npub1awsnqr5338h497yam5m9hrgh9535yadj9zxglwk55xpsdtsn2c4syjruew",
            )
            .unwrap(),
            None,
            None::<&str>,
        )];
        client.connect().await;
        let result = set_contact_list(&client, signer, contacts).await;
        assert!(result.is_ok());
    }
}
