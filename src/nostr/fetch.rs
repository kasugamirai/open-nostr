use std::collections::HashMap;
use std::f64::consts::E;
use std::time::Duration;

use super::note::ReplyTrees;
use nostr_indexeddb::database::Order;
use nostr_indexeddb::WebDatabase;
use nostr_sdk::{
    client, event, Alphabet, Client, Event, EventId, Filter, FilterOptions, JsonUtil, Kind, Marker,
    Metadata, PublicKey, SingleLetterTag, Tag,
};
use nostr_sdk::{NostrDatabase, Timestamp};
use web_sys::console;

/// Error enum to represent possible errors in the application.
#[derive(Debug)]
pub enum Error {
    NostrSdkClient(client::Error),
    Database(nostr_indexeddb::IndexedDBError),
    Metadata(nostr_sdk::types::metadata::Error),
    Custom(String),
}

impl From<nostr_sdk::types::metadata::Error> for Error {
    fn from(err: nostr_sdk::types::metadata::Error) -> Self {
        Self::Metadata(err)
    }
}

impl From<client::Error> for Error {
    fn from(err: client::Error) -> Self {
        Self::NostrSdkClient(err)
    }
}

impl From<nostr_indexeddb::IndexedDBError> for Error {
    fn from(err: nostr_indexeddb::IndexedDBError) -> Self {
        Self::Database(err)
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Self::Custom(err)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NostrSdkClient(err) => write!(f, "NostrSdkClientError: {}", err),
            Self::Database(err) => write!(f, "DatabaseError: {}", err),
            Self::Metadata(err) => write!(f, "MetadataError: {}", err),
            Self::Custom(err) => write!(f, "CustomError: {}", err),
        }
    }
}

/// The `Fetcher` struct is responsible for fetching data from a specified source.
pub struct Fetcher {}

impl Default for Fetcher {
    fn default() -> Self {
        Self::new()
    }
}

impl Fetcher {
    pub fn new() -> Self {
        Self {}
    }
    pub async fn fetch_note_and_replies(
        &self,
        client: Client,
        event_id: EventId,
        timeout: Option<std::time::Duration>,
    ) -> Result<ReplyTrees<'static>, Error> {
        // Fetch the main note
        let main_note = self
            .get_single_event(client.clone(), event_id, timeout)
            .await?;
        // Fetch replies
        let replies = self.get_replies(client, event_id, timeout).await?;

        // Combine the main note and replies into one vector
        let mut events = Vec::new();
        if let Some(note) = main_note {
            events.push(note);
        }
        events.extend(replies);

        // Create reply trees
        let mut reply_trees = ReplyTrees::default();
        //reply_trees.accept(&events.iter().cloned().collect::<Vec<_>>());
        Ok(reply_trees)
    }

    // Fetches a single event by its ID
    async fn get_single_event(
        &self,
        client: Client,
        event_id: EventId,
        timeout: Option<std::time::Duration>,
    ) -> Result<Option<Event>, Error> {
        let filter = Filter::new().id(event_id);
        let events = client.get_events_of(vec![filter], timeout).await?;
        Ok(events.into_iter().next())
    }

    // Fetches all replies to a given event ID
    pub async fn get_replies(
        &self,
        client: Client,
        event_id: EventId,
        timeout: Option<std::time::Duration>,
    ) -> Result<Vec<Event>, Error> {
        let filter = Filter::new().kind(Kind::TextNote).custom_tag(
            SingleLetterTag::lowercase(Alphabet::E),
            vec![event_id.to_hex()],
        );
        let events = client.get_events_of(vec![filter], timeout).await?;
        let ret = filter_root_replies(&events);
        Ok(ret)
    }

    /// Fetches the metadata of a user with the given public key.
    pub async fn get_metadata(
        &self,
        client: Client,
        public_key: PublicKey,
        timeout: Option<Duration>,
    ) -> Result<Metadata, Error> {
        let filter = Filter::new().author(public_key).kind(Kind::Metadata);
        let events = client.get_events_of(vec![filter], timeout).await?;
        let event = get_newest_event(&events);
        if let Some(event) = event {
            let m = Metadata::from_json(&event.content)?;
            Ok(m)
        } else {
            Err("MetaData not found".to_string().into())
        }
    }

    /// Fetches the reactions of an event with the given event ID.
    pub async fn get_reactions(
        &self,
        client: Client,
        event_id: EventId,
        timeout: Option<std::time::Duration>,
    ) -> Result<HashMap<String, i32>, Error> {
        let filter = Filter::new().kind(Kind::Reaction).custom_tag(
            SingleLetterTag::lowercase(Alphabet::E),
            vec![event_id.to_hex()],
        );
        let events = client.get_events_of(vec![filter], timeout).await?;

        let ret = count_events(&events)?;
        Ok(ret)
    }

    /// Fetches the reposts of an event with the given event ID.
    pub async fn get_repost(
        &self,
        client: Client,
        event_id: EventId,
        timeout: Option<std::time::Duration>,
    ) -> Result<Vec<Event>, Error> {
        let filter = Filter::new().kind(Kind::Repost).custom_tag(
            SingleLetterTag::lowercase(Alphabet::E),
            vec![event_id.to_hex()],
        );
        let events = client.get_events_of(vec![filter], timeout).await?;
        Ok(events)
    }

    /// Fetches the following of a user with the given public key.
    pub async fn get_following(
        &self,
        client: Client,
        public_key: PublicKey,
    ) -> Result<Vec<PublicKey>, Error> {
        todo!()
    }

    /// get events from db
    pub async fn get_events_from_db(
        &self,
        db: WebDatabase,
        filters: Filter,
    ) -> Result<Vec<Event>, Error> {
        let events = db.query(vec![filters], Order::Desc).await?;
        Ok(events)
    }

    /// get events and save them to db
    pub async fn get_events(
        &self,
        client: Client,
        db: WebDatabase,
        filters: Vec<Filter>,
        save_opts: bool,
        opts: FilterOptions,
        timeout: Option<std::time::Duration>,
    ) -> Result<Vec<Event>, Error> {
        let events = client
            .get_events_of_with_opts(filters, timeout, opts)
            .await?;

        if save_opts {
            save_all_events(&events, &db).await?;
        }
        Ok(events)
    }

    // sync data and save them to db
    pub async fn sync_data_saved(
        &self,
        client: Client,
        db: WebDatabase,
        mut filters: Vec<Filter>,
        opts: FilterOptions,
        timeout: Option<std::time::Duration>,
        //filters_transformer: impl Fn(&Vec<Filter>) -> Vec<Filter>,
        //enum_stop_conditions: Vec<impl Fn(&Vec<Event>) -> bool>,
    ) -> Result<(), Error> {
        //let mut all_events = Vec::new();
        let conditions = vec![StopCondition::NoEvents, StopCondition::DataInDb];

        'outer: loop {
            let events = self
                .get_events(
                    client.clone(),
                    db.clone(),
                    filters.clone(),
                    false,
                    opts,
                    timeout,
                )
                .await?;
            //all_events.extend(events.clone());
            for condition in &conditions {
                if condition.check(&events, &db).await {
                    break 'outer;
                }
            }
            save_all_events(&events, &db).await?;
            filters = filters_transformer(&filters, &events);
        }

        Ok(())
    }
}

/// Transforms the filters based on the events.
fn filters_transformer(filters: &[Filter], events: &[Event]) -> Vec<Filter> {
    let earliest_event_date = get_earliest_event_date(events);
    let mut updated_filters = Vec::new();
    for filter in filters.iter() {
        let updated_filter = <Filter as Clone>::clone(filter)
            .until(earliest_event_date)
            .limit(2);
        updated_filters.push(updated_filter);
    }
    console::log_1(&format!("Updated filters: {:?}", updated_filters).into());
    updated_filters
}

/// Returns the earliest event date from the given list of events.
///
/// The function iterates over the events and keeps track of the earliest event date.
/// It initializes an `event_time` variable as `None`. Then, for each event, it compares
/// the event's creation time with the current earliest event time. If the `event_time`
/// is `None` or the event's time is earlier than the current earliest time, the `event_time`
/// is updated to the event's time. Finally, the function logs the earliest event date
/// to the console using `console::log_1`, and returns the earliest event time.
///
/// # Arguments
///
/// * `events` - A slice of events for which to find the earliest event date.
///
/// # Returns
///
/// The earliest event date as a `Timestamp`.
fn get_earliest_event_date(events: &[Event]) -> Timestamp {
    let mut event_time: Option<Timestamp> = None;
    for event in events.iter() {
        let e_time = event.created_at();
        if event_time.is_none() || e_time < event_time.unwrap() {
            event_time = Some(e_time);
        }
    }
    console::log_1(&format!("Earliest event date: {}", event_time.unwrap()).into());
    event_time.unwrap()
}

/// Returns the newest event from a slice of events.
///
/// # Arguments
///
/// * `events` - A slice of events to search from.
///
/// # Returns
///
/// The newest event as an option. If the slice is empty, returns `None`.
///
/// # Examples
///
/// ```rust
/// use my_module::Event;
/// let events = vec![
///     Event::new("Event 1", 1609459200),
///     Event::new("Event 2", 1609545600),
///     Event::new("Event 3", 1609632000),
/// ];
///
/// let newest_event = my_module::get_newest_event(&events);
///
/// assert_eq!(newest_event, Some(Event::new("Event 3", 1609632000)));
/// ```
fn get_newest_event(events: &[Event]) -> Option<Event> {
    let mut newest_event: Option<Event> = None;
    for event in events.iter() {
        if newest_event
            .as_ref()
            .map_or(true, |ne| event.created_at() > ne.created_at())
        {
            newest_event = Some(event.clone());
        }
    }
    newest_event
}

/// Saves a slice of events to the provided `WebDatabase`.
///
/// # Arguments
///
/// * `events` - A reference to a slice of `Event` structs.
/// * `db` - A reference to a `WebDatabase`.
///
/// # Returns
///
/// This function returns a `Result` indicating success or failure.
///
/// # Example
///
/// ```rust
/// let events = vec![
///     Event { id: 1, name: "Event 1" },
///     Event { id: 2, name: "Event 2" },
/// ];
///
/// let db = WebDatabase::new();
/// save_all_events(&events, &db).await?;
/// ```
/// If the function succeeds, it returns `Ok(())`.
async fn save_all_events(events: &[Event], db: &WebDatabase) -> Result<(), Error> {
    for event in events.iter() {
        db.save_event(event).await?;
    }
    Ok(())
}

fn filter_root_replies(events: &[Event]) -> Vec<Event> {
    let mut ret = Vec::new();
    for event in events.iter() {
        event.iter_tags().for_each(|t| {
            if let Tag::Event { marker, .. } = t {
                match marker {
                    Some(Marker::Root) => ret.push(event.clone()),
                    _ => (),
                }
            }
        });
    }
    ret
}

fn filter_reply_replies(events: &[Event]) -> Vec<Event> {
    let mut ret = Vec::new();
    for event in events.iter() {
        event.iter_tags().for_each(|t| {
            if let Tag::Event { marker, .. } = t {
                match marker {
                    Some(Marker::Reply) => ret.push(event.clone()),
                    _ => (),
                }
            }
        });
    }
    ret
}

/// Counts the number of occurrences of each event content.
///
/// This function takes a slice of events and returns a `Result` wrapping a `HashMap` where the keys are the event contents
/// and the values are the count of occurrences of each event content. If an error occurs during the execution, an `Error` is
/// returned.
///
/// # Arguments
///
/// * `events` - A slice of `Event` objects representing the events to count.
///
/// # Returns
///
/// * `Result<HashMap<String, i32>, Error>` - A `Result` that wraps a `HashMap` where the keys are the event contents and
///    the values are the count of occurrences of each event content. If successful, it returns `Ok(ret)`, where `ret` is the
///    resulting `HashMap`. If an error occurs, it returns `Err(Error)`.
///
/// # Examples
///
/// ```rust
/// use std::collections::HashMap;
///
/// struct Event {
///     content: String,
/// }
///
/// impl Event {
///     fn content(&self) -> &str {
///         &self.content
///     }
/// }
///
/// fn count_events(events: &[Event]) -> Result<HashMap<String, i32>, Error> {
///     // implementation here
/// }
///
/// let events = vec![
///     Event { content: String::from("event1") },
///     Event { content: String::from("event2") },
///     Event { content: String::from("event1") },
/// ];
///
/// let result = count_events(&events);
/// assert_eq!(result.unwrap().get("event1"), Some(&2));
/// assert_eq!(result.unwrap().get("event2"), Some(&1));
/// assert_eq!(result.unwrap().get("event3"), None);
/// ```
///
fn count_events(events: &[Event]) -> Result<HashMap<String, i32>, Error> {
    let mut ret = HashMap::new();
    for event in events.iter() {
        let content = event.content().to_string();
        *ret.entry(content).or_insert(0) += 1;
    }
    Ok(ret)
}

/// Represents the possible stop conditions for a process.
///
/// # Examples
///
/// ```rust
/// use crate::StopCondition;
///
/// let condition = StopCondition::NoEvents;
/// ```
///
/// ```rust
/// use crate::StopCondition;
///
/// let condition = StopCondition::DataInDb;
/// ```
///
enum StopCondition {
    NoEvents,
    DataInDb,
}

impl StopCondition {
    async fn check(&self, events: &[Event], db: &WebDatabase) -> bool {
        match self {
            Self::NoEvents => {
                if events.is_empty() {
                    let message = "No events found";
                    console::log_1(&message.into());
                    return true;
                }
                false
            }
            Self::DataInDb => {
                let mut filters = Vec::new();
                for event in events.iter() {
                    let filter = vec![Filter::new().id(event.id)];
                    filters.extend(filter);
                }

                let ret = match db.query(filters, Order::Desc).await {
                    Ok(result) => result,
                    Err(e) => {
                        console::log_1(&format!("Database query failed: {}", e).into());
                        return false;
                    }
                };
                if ret.len() == events.len() {
                    let message = "All data saved in db";
                    console::log_1(&message.into());
                    return true;
                }
                false
            }
        }
    }
}

/*
fn test() {
    let fetcher = Fetcher::new();
    let enum_stop_conditions: Vec<Box<dyn Fn(&Vec<Event>) -> bool>> = vec![
        Box::new(|events| fetcher.stop_when_no_events(events)),
        Box::new(|events| fetcher.stop_when_date_reached(events, 0, 0)),
    ];
}
*/

/// Test module for the Fetcher structure.
///
/// # Examples
///
/// ```
/// #[cfg(test)]
/// mod tests {
///     use super::Fetcher;
///     use nostr_indexeddb::WebDatabase;
///     use nostr_sdk::prelude::*;
///     use wasm_bindgen_test::*;
///
///     wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
///
///     #[wasm_bindgen_test]
///     async fn test_get_events() {
///         // Code goes here
///     }
///
///     #[wasm_bindgen_test]
///     async fn test_sync_data_saved() {
///         // Code goes here
///     }
///
///     #[wasm_bindgen_test]
///     async fn test_get_metadata() {
///         // Code goes here
///     }
///
///     #[wasm_bindgen_test]
///     async fn test_get_reactions() {
///         // Code goes here
///     }
///
///     #[wasm_bindgen_test]
///     async fn test_get_reply() {
///         // Code goes here
///     }
///
///     #[wasm_bindgen_test]
///     async fn test_get_repost() {
///         // Code goes here
///     }
/// }
/// ```
#[cfg(test)]
mod tests {
    //use std::pin::Pin;

    use super::Fetcher;
    //use futures::Future;
    use nostr_indexeddb::WebDatabase;
    use nostr_sdk::prelude::*;
    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]

    async fn test_get_events() {
        let public_key = PublicKey::from_bech32(
            "npub1zfss807aer0j26mwp2la0ume0jqde3823rmu97ra6sgyyg956e0s6xw445",
        )
        .unwrap();
        let db = WebDatabase::open("EVENTS_DB").await.unwrap();
        let client = Client::default();
        client.add_relay("wss://relay.damus.io").await.unwrap();
        client.connect().await;
        let fetcher = Fetcher::new();
        let filter = Filter::new()
            .author(public_key)
            .kind(Kind::TextNote)
            .limit(2);
        let opts = FilterOptions::default();
        let timeout = None;

        let events = fetcher
            .get_events(client, db, vec![filter], true, opts, timeout)
            .await
            .unwrap();

        assert_eq!(events.len(), 2);
    }

    #[wasm_bindgen_test]

    async fn test_sync_data_saved() {
        let public_key = PublicKey::from_bech32(
            "npub1q0uulk2ga9dwkp8hsquzx38hc88uqggdntelgqrtkm29r3ass6fq8y9py9",
        )
        .unwrap();
        let db = WebDatabase::open("EVENTS_DB").await.unwrap();
        let client = Client::default();
        client.add_relay("wss://relay.damus.io").await.unwrap();
        client.connect().await;
        let fetcher = Fetcher::new();
        let filter = Filter::new()
            .author(public_key)
            .kind(Kind::TextNote)
            .limit(2);
        let opts = FilterOptions::default();
        let timeout = None;
        let ret = fetcher
            .sync_data_saved(client, db, vec![filter], opts, timeout)
            .await;
        assert!(ret.is_ok());
    }

    #[wasm_bindgen_test]
    async fn test_get_metadata() {
        let timeout = Some(std::time::Duration::from_secs(5));
        let name = "xy";
        let public_key = PublicKey::from_bech32(
            "npub1q0uulk2ga9dwkp8hsquzx38hc88uqggdntelgqrtkm29r3ass6fq8y9py9",
        )
        .unwrap();
        let client = Client::default();
        client.add_relay("wss://relay.damus.io").await.unwrap();
        client.connect().await;
        let fetcher = Fetcher::new();
        let metadata = fetcher
            .get_metadata(client, public_key, timeout)
            .await
            .unwrap();
        console_log!("Metadata: {:?}", metadata);
        assert_eq!(metadata.name, Some(name.to_string()));
    }

    #[wasm_bindgen_test]
    async fn test_get_reactions() {
        let timeout = Some(std::time::Duration::from_secs(5));
        let event_id =
            EventId::from_bech32("note1yht55eufy56v6twj4jzvs4kmplm6k3yayj3yyjzfs9mjhu2vlnms7x3x4h")
                .unwrap();
        let client = Client::default();
        client.add_relay("wss://relay.damus.io").await.unwrap();
        client.connect().await;
        let fetcher = Fetcher::new();
        let reactions = fetcher
            .get_reactions(client, event_id, timeout)
            .await
            .unwrap();
        let length = reactions.len();
        console_log!("Reactions: {:?}", reactions);
        assert_eq!(reactions.len(), length);
    }

    #[wasm_bindgen_test]
    async fn test_get_reply() {
        let timeout = Some(std::time::Duration::from_secs(5));
        let event_id =
            EventId::from_bech32("note1yht55eufy56v6twj4jzvs4kmplm6k3yayj3yyjzfs9mjhu2vlnms7x3x4h")
                .unwrap();
        let client = Client::default();
        client.add_relay("wss://relay.damus.io").await.unwrap();
        client.add_relay("wss://nos.lol").await.unwrap();
        client.connect().await;
        let fetcher = Fetcher::new();
        let replies = fetcher
            .get_replies(client, event_id, timeout)
            .await
            .unwrap();
        let length = replies.len();
        console_log!("Replies: {:?}", replies);
        assert_eq!(replies.len(), length);
    }

    #[wasm_bindgen_test]
    async fn test_get_repost() {
        let timeout = Some(std::time::Duration::from_secs(5));
        let event_id =
            EventId::from_bech32("note1emq5z2agsdqzhztd4t8k9wvjh7nzm7dtype5herygf8dran86fpsm39ncs")
                .unwrap();
        let client = Client::default();
        client.add_relay("wss://relay.damus.io").await.unwrap();
        client.connect().await;
        let fetcher = Fetcher::new();
        let reposts = fetcher.get_repost(client, event_id, timeout).await.unwrap();
        let length = reposts.len();
        console_log!("Reposts: {:?}", reposts);
        assert_eq!(reposts.len(), length);
    }
}
