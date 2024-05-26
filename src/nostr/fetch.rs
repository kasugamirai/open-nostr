use futures::stream::{self, StreamExt};
use futures::{Future, Stream};
use nostr_indexeddb::database::Order;
use nostr_sdk::nips::nip04;
use nostr_sdk::{client, Metadata, Tag};
use nostr_sdk::{Alphabet, Client, Event, EventId, Filter, Kind, SingleLetterTag};
use nostr_sdk::{JsonUtil, Timestamp};
use nostr_sdk::{NostrSigner, PublicKey};
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Duration;

macro_rules! impl_from_error {
    ($src:ty, $variant:ident) => {
        impl From<$src> for Error {
            fn from(err: $src) -> Self {
                Self::$variant(err)
            }
        }
    };
}

macro_rules! create_encrypted_filters {
    ($kind:expr, $author:expr, $public_key:expr) => {{
        (
            Filter::new().kind($kind).author($author).custom_tag(
                SingleLetterTag::lowercase(Alphabet::P),
                vec![$author.to_hex()],
            ),
            Filter::new().kind($kind).author($author).custom_tag(
                SingleLetterTag::lowercase(Alphabet::P),
                vec![$public_key.to_hex()],
            ),
        )
    }};
}

/// Error enum to represent possible errors in the application.
#[derive(Debug)]
pub enum Error {
    Client(client::Error),
    MetadataConversion(nostr_sdk::types::metadata::Error),
    Database(nostr_indexeddb::IndexedDBError),
    Decryptor(nip04::Error),
    Signer(nostr_sdk::signer::Error),
    NotFound,
    UnableToSave,
}

impl_from_error!(nostr_sdk::signer::Error, Signer);
impl_from_error!(nip04::Error, Decryptor);
impl_from_error!(client::Error, Client);
impl_from_error!(nostr_sdk::types::metadata::Error, MetadataConversion);
impl_from_error!(nostr_indexeddb::IndexedDBError, Database);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Client(e) => write!(f, "Client error: {}", e),
            Self::MetadataConversion(e) => write!(f, "Metadata conversion error: {}", e),
            Self::Database(e) => write!(f, "Database error: {}", e),
            Self::Decryptor(e) => write!(f, "Decryptor error: {}", e),
            Self::Signer(e) => write!(f, "Signer error: {}", e),
            Self::NotFound => write!(f, "Not found"),
            Self::UnableToSave => write!(f, "Unable to save"),
        }
    }
}

#[derive(Debug)]
pub struct DecryptedMsg {
    /// Id
    pub id: EventId,
    /// Author
    pub pubkey: PublicKey,
    /// Timestamp (seconds)
    pub created_at: Timestamp,
    /// Kind
    pub kind: Kind,
    /// Vector of [`Tag`]
    pub tags: Vec<Tag>,
    /// Content
    pub content: String,
}

impl DecryptedMsg {
    pub fn new(
        id: EventId,
        pubkey: PublicKey,
        created_at: Timestamp,
        kind: Kind,
        tags: Vec<Tag>,
        content: String,
    ) -> Self {
        Self {
            id,
            pubkey,
            created_at,
            kind,
            tags,
            content,
        }
    }
}

pub struct EventPaginator<'a> {
    client: &'a Client,
    filters: Vec<Filter>,
    oldest_timestamp: Option<Timestamp>,
    done: bool,
    timeout: Option<Duration>,
    page_size: usize,
    last_event_ids: HashSet<EventId>,
}

impl<'a> EventPaginator<'a> {
    pub fn new(
        client: &'a Client,
        filters: Vec<Filter>,
        timeout: Option<Duration>,
        page_size: usize,
    ) -> Self {
        Self {
            client,
            filters,
            oldest_timestamp: None,
            done: false,
            timeout,
            page_size,
            last_event_ids: HashSet::new(),
        }
    }

    pub fn are_all_event_ids_present(&self, events: &[Event]) -> bool {
        events
            .iter()
            .all(|event| self.last_event_ids.contains(&event.id))
    }

    pub async fn next_page(&mut self) -> Option<Result<Vec<Event>, Error>> {
        if self.done {
            return None;
        }

        // Update filters with the oldest timestamp and limit
        let updated_filters = self
            .filters
            .iter()
            .map(|f| {
                let mut f = f.clone();
                if let Some(timestamp) = self.oldest_timestamp {
                    f = f.until(timestamp);
                }
                f = f.limit(self.page_size);
                f
            })
            .collect::<Vec<_>>();

        // Fetch events
        match self
            .client
            .get_events_of(updated_filters.clone(), self.timeout)
            .await
        {
            Ok(events) => {
                if events.is_empty() || self.are_all_event_ids_present(&events) {
                    self.done = true;
                    return None;
                }

                // Update the oldest timestamp
                if let Some(oldest_event) = get_oldest_event(&events) {
                    self.oldest_timestamp = Some(oldest_event.created_at());
                }

                // Update the filters
                self.filters = updated_filters;

                self.last_event_ids = events.iter().map(|event| event.id).collect();

                Some(Ok(events))
            }
            Err(e) => {
                self.done = true;
                Some(Err(Error::Client(e)))
            }
        }
    }
}

/*
impl<'a> Stream for EventPaginator<'a> {
    type Item = Result<Vec<Event>, Error>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let fut = self.next_page();
        futures::pin_mut!(fut);
        match fut.poll(cx) {
            std::task::Poll::Ready(res) => std::task::Poll::Ready(res),
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }
}
*/

pub struct DecryptedMsgPaginator<'a> {
    signer: &'a NostrSigner,
    target_pub_key: PublicKey,
    paginator: EventPaginator<'a>,
}

impl<'a> DecryptedMsgPaginator<'a> {
    pub async fn new(
        signer: &'a NostrSigner,
        client: &'a Client,
        target_pub_key: PublicKey,
        timeout: Option<Duration>,
        page_size: usize,
    ) -> Result<DecryptedMsgPaginator<'a>, Error> {
        let public_key = signer.public_key().await?;

        let (me, target) =
            create_encrypted_filters!(Kind::EncryptedDirectMessage, target_pub_key, public_key);
        let filters = vec![me, target];

        let paginator = EventPaginator::new(client, filters, timeout, page_size);
        Ok(DecryptedMsgPaginator {
            signer,
            target_pub_key,
            paginator,
        })
    }

    async fn decrypt_dm_event(&self, event: &Event) -> Result<String, Error> {
        let msg = self
            .signer
            .nip04_decrypt(self.target_pub_key, &event.content)
            .await?;
        Ok(msg)
    }

    async fn convert_events(&self, events: Vec<Event>) -> Result<Vec<DecryptedMsg>, Error> {
        let futures: Vec<_> = events
            .into_iter()
            .map(|event| async move {
                match self.decrypt_dm_event(&event).await {
                    Ok(msg) => Ok(DecryptedMsg::new(
                        event.id,
                        event.author(),
                        event.created_at,
                        event.kind,
                        event.tags.clone(),
                        msg,
                    )),
                    Err(err) => Err(err),
                }
            })
            .collect();

        futures::future::join_all(futures)
            .await
            .into_iter()
            .collect()
    }

    pub async fn next_page(&mut self) -> Option<Result<Vec<DecryptedMsg>, Error>> {
        if self.paginator.done {
            return None;
        }

        if let Some(Ok(events)) = self.paginator.next_page().await {
            let decrypt_results = self.convert_events(events).await;
            return Some(decrypt_results);
        }

        None
    }
}

pub async fn get_event_by_id(
    client: &Client,
    event_id: &EventId,
    timeout: Option<std::time::Duration>,
) -> Result<Option<Event>, Error> {
    let filters = Filter::new().id(*event_id).limit(1);
    let events = client.get_events_of(vec![filters], timeout).await?;
    Ok(events.into_iter().next())
}

pub async fn get_events_by_ids(
    client: &Client,
    event_ids: &[EventId],
    timeout: Option<std::time::Duration>,
) -> Result<Vec<Event>, Error> {
    let filters: Vec<Filter> = event_ids.iter().map(|id| Filter::new().id(*id)).collect();
    let events = client.get_events_of(filters, timeout).await?;
    Ok(events)
}

pub async fn get_metadata(
    client: &Client,
    public_key: &PublicKey,
    timeout: Option<std::time::Duration>,
) -> Result<Metadata, Error> {
    let filters = Filter::new().author(*public_key).kind(Kind::Metadata);
    let events = client.get_events_of(vec![filters], timeout).await?;
    if let Some(event) = get_newest_event(&events) {
        let metadata = Metadata::from_json(&event.content)?;
        Ok(metadata)
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
    event_id: &EventId,
    timeout: Option<std::time::Duration>,
) -> Result<Vec<Event>, Error> {
    let filters = Filter::new().kind(Kind::TextNote).custom_tag(
        SingleLetterTag::lowercase(Alphabet::E),
        vec![event_id.to_hex()],
    );
    let events = client.get_events_of(vec![filters], timeout).await?;
    // TODO: filter out the mentions if necessary
    Ok(events)
}

pub async fn query_events_from_db(
    client: &Client,
    filters: Vec<Filter>,
) -> Result<Vec<Event>, Error> {
    let events = client.database().query(filters, Order::Desc).await;
    events.map_err(|e| Error::Database(nostr_indexeddb::IndexedDBError::Database(e)))
}

pub fn get_newest_event(events: &[Event]) -> Option<&Event> {
    events.iter().max_by_key(|event| event.created_at())
}

pub fn get_oldest_event(events: &[Event]) -> Option<&Event> {
    events.iter().min_by_key(|event| event.created_at())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testhelper::test_data::*;
    use crate::{
        init::NOSTR_DB_NAME,
        nostr::note::{DisplayOrder, ReplyTrees},
        testhelper::event_from,
    };
    use nostr_indexeddb::WebDatabase;
    use nostr_sdk::key::SecretKey;
    use nostr_sdk::{ClientBuilder, FromBech32, Keys};
    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_get_event_by_id() {
        let timeout = Some(std::time::Duration::from_secs(5));
        let event_id =
            EventId::from_hex("ff25d26e734c41fa7ed86d28270628f8fb2f6fb03a23eed3d38502499c1a7a2b")
                .unwrap();
        let client = Client::default();
        client.add_relay("wss://relay.damus.io").await.unwrap();
        client.connect().await;
        let event = get_event_by_id(&client, &event_id, timeout).await.unwrap();
        assert!(event.is_some());
    }

    #[wasm_bindgen_test]
    async fn test_get_replies() {
        let timeout = Some(std::time::Duration::from_secs(5));
        let event_id =
            EventId::from_hex("ff25d26e734c41fa7ed86d28270628f8fb2f6fb03a23eed3d38502499c1a7a2b")
                .unwrap();
        let client = Client::default();
        client.add_relay("wss://relay.damus.io").await.unwrap();
        client.connect().await;
        let replies = get_replies(&client, &event_id, timeout).await.unwrap();
        assert_eq!(replies.len(), 4);
    }

    #[wasm_bindgen_test]
    async fn test_get_replies_into_tree() {
        let timeout = Some(std::time::Duration::from_secs(5));
        let event_id =
            EventId::from_hex("57938b39678af44bc3ae76cf4b815bcdb65ffe71bb84ce35706f0c6fca4ed394")
                .unwrap();
        let client = Client::default();
        client.add_relay("wss://nos.lol").await.unwrap();
        client.add_relay("wss://relay.damus.io").await.unwrap();
        client.connect().await;
        let root = get_event_by_id(&client, &event_id, timeout)
            .await
            .unwrap()
            .unwrap();
        let replies = get_replies(&client, &event_id, timeout).await.unwrap();
        assert_eq!(replies.len(), 3);
        let mut tree = ReplyTrees::default();
        tree.accept(vec![root]);
        tree.accept(replies);
        let lv1_replies = tree.get_replies(&event_id, Some(DisplayOrder::NewestFirst));
        console_log!("lv1_replies {:?}", lv1_replies);
        assert!(lv1_replies.len() == 3);
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
        let reactions = get_reactions(&client, &event_id, timeout).await.unwrap();
        let length = reactions.len();
        console_log!("Reactions: {:?}", reactions);
        assert_eq!(reactions.len(), length);
    }

    #[wasm_bindgen_test]
    async fn test_fetch_from_db() {
        let db = WebDatabase::open(NOSTR_DB_NAME).await.unwrap();
        let client_builder = ClientBuilder::new().database(db);
        let client: nostr_sdk::Client = client_builder.build();

        //save event to db
        let event = event_from(REPLY_WITH_MARKER);
        client.database().save_event(&event).await.unwrap();

        //query from db
        let filter = Filter::new().id(event.id).limit(1);
        let event_result = client
            .database()
            .query(vec![filter], Order::Desc)
            .await
            .unwrap();
        assert!(event_result.len() == 1);
        assert!(event_result[0].id == event.id);
    }

    #[wasm_bindgen_test]
    async fn test_event_page_iterator() {
        let client = Client::default();
        client.add_relay("wss://relay.damus.io").await.unwrap();
        client.connect().await;

        let public_key = PublicKey::from_bech32(
            "npub1xtscya34g58tk0z605fvr788k263gsu6cy9x0mhnm87echrgufzsevkk5s",
        )
        .unwrap();

        let filter: Filter = Filter::new()
            .kind(Kind::EncryptedDirectMessage)
            .author(public_key);

        let page_size = 100;
        let timeout = Some(std::time::Duration::from_secs(5));
        let mut paginator = EventPaginator::new(&client, vec![filter], timeout, page_size);

        let mut count = 0;
        while let Some(result) = paginator.next_page().await {
            match result {
                Ok(events) => {
                    if paginator.done {
                        break;
                    }
                    console_log!("events are: {:?}", events);
                    count += events.len();
                }
                Err(e) => {
                    console_log!("Error fetching events: {:?}", e);
                    break;
                }
            }
        }

        assert!(count > 100);
    }

    #[wasm_bindgen_test]
    async fn test_encrypted_direct_message_filters_iterator() {
        let private_key = SecretKey::from_bech32(
            "nsec1qrypzwmxp8r54ctx2x7mhqzh5exca7xd8ssnlfup0js9l6pwku3qacq4u3",
        )
        .unwrap();
        let key = Keys::new(private_key);
        let target_pub_key = PublicKey::from_bech32(
            "npub155pujvquuuy47kpw4j3t49vq4ff9l0uxu97fhph9meuxvwc0r4hq5mdhkf",
        )
        .unwrap();
        let client = Client::new(&key);
        client.add_relay("wss://relay.damus.io").await.unwrap();
        client.connect().await;
        let singer = client.signer().await.unwrap();
        let page_size = 3;
        let timeout = Some(std::time::Duration::from_secs(5));
        let mut paginator =
            DecryptedMsgPaginator::new(&singer, &client, target_pub_key, timeout, page_size)
                .await
                .unwrap();
        let mut count = 0;
        while let Some(result) = paginator.next_page().await {
            match result {
                Ok(events) => {
                    if paginator.paginator.done {
                        break;
                    }
                    console_log!("events are: {:?}", events);
                    for e in &events {
                        console_log!("event: {:?}", e.content);
                    }
                    count += events.len();
                }
                Err(e) => {
                    console_log!("Error fetching events: {:?}", e);
                    break;
                }
            }
        }
        assert!(count > 7);
    }
}
