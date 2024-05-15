use nostr_sdk::prelude::*;
use nostr_sdk::{client, Metadata};
use std::collections::HashMap;

/// Error enum to represent possible errors in the application.
#[derive(Debug)]
pub enum Error {
    Client(client::Error),
    MetadataConversion(nostr_sdk::types::metadata::Error),
    Database(nostr_indexeddb::IndexedDBError),
    NotFound,
    UnableToSave,
}

impl From<client::Error> for Error {
    fn from(err: client::Error) -> Self {
        Self::Client(err)
    }
}

impl From<nostr_sdk::types::metadata::Error> for Error {
    fn from(err: nostr_sdk::types::metadata::Error) -> Self {
        Self::MetadataConversion(err)
    }
}

impl From<nostr_indexeddb::IndexedDBError> for Error {
    fn from(err: nostr_indexeddb::IndexedDBError) -> Self {
        Self::Database(err)
    }
}

pub fn get_newest_event(events: &[Event]) -> Option<&Event> {
    events.iter().max_by_key(|event| event.created_at())
}

pub fn get_oldest_event(events: &[Event]) -> Option<&Event> {
    events.iter().min_by_key(|event| event.created_at())
}

pub async fn get_event_by_id(
    client: &Client,
    event_id: &EventId,
    timeout: Option<std::time::Duration>,
) -> Result<Option<Event>, Error> {
    let filter = Filter::new().id(*event_id).limit(1);
    let events = client.get_events_of(vec![filter], timeout).await?;
    Ok(events.into_iter().next())
}

pub async fn get_events_by_ids(
    client: &Client,
    event_ids: &[EventId],
    timeout: Option<std::time::Duration>,
) -> Result<Vec<Event>, Error> {
    let mut filters = Vec::with_capacity(event_ids.len());
    for id in event_ids {
        filters.push(Filter::new().id(*id));
    }
    let events = client.get_events_of(filters, timeout).await?;
    Ok(events)
}

pub async fn get_metadata(
    client: &Client,
    public_key: &PublicKey,
    timeout: Option<std::time::Duration>,
) -> Result<Metadata, Error> {
    let filter = Filter::new().author(*public_key).kind(Kind::Metadata);
    let events = client.get_events_of(vec![filter], timeout).await?;
    let event = get_newest_event(&events);
    if let Some(event) = event {
        let m = Metadata::from_json(&event.content)?;
        Ok(m)
    } else {
        Err(Error::NotFound)
    }
}

pub async fn get_reactions(
    client: &Client,
    event_id: &EventId,
    timeout: Option<std::time::Duration>,
) -> Result<HashMap<String, i32>, Error> {
    let reaction_filter = Filter::new().kind(Kind::Reaction).custom_tag(
        SingleLetterTag::lowercase(Alphabet::E),
        vec![event_id.to_hex()],
    );

    let events = client.get_events_of(vec![reaction_filter], timeout).await?;

    let mut reaction_counts = HashMap::new();
    for event in events.iter() {
        let content = event.content().to_string();
        *reaction_counts.entry(content).or_insert(0) += 1;
    }

    Ok(reaction_counts)
}



pub async fn get_replies(
    client: &Client,
    event_id: EventId,
    timeout: Option<std::time::Duration>,
) -> Result<Vec<Event>, Error> {
    let filter = Filter::new().kind(Kind::TextNote).custom_tag(
        SingleLetterTag::lowercase(Alphabet::E),
        vec![event_id.to_hex()],
    );
    let events = client.get_events_of(vec![filter], timeout).await?;
    //todo filter out the mentions
    Ok(events)
}