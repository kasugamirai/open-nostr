use futures::{Future, Stream, StreamExt};
use nostr_sdk::prelude::*;
use nostr_sdk::Timestamp;
use nostr_sdk::{client, Metadata};
use std::collections::HashMap;
use std::time::Duration;
/// Error enum to represent possible errors in the application.
#[derive(Debug)]
pub enum Error {
    Client(client::Error),
    MetadataConversion(nostr_sdk::types::metadata::Error),
    Database(nostr_indexeddb::IndexedDBError),
    NotFound,
    UnableToSave,
}

macro_rules! impl_from_error {
    ($src:ty, $variant:ident) => {
        impl From<$src> for Error {
            fn from(err: $src) -> Self {
                Self::$variant(err)
            }
        }
    };
}

impl_from_error!(client::Error, Client);
impl_from_error!(nostr_sdk::types::metadata::Error, MetadataConversion);
impl_from_error!(nostr_indexeddb::IndexedDBError, Database);

pub fn get_newest_event(events: &[Event]) -> Option<&Event> {
    events.iter().max_by_key(|event| event.created_at())
}

pub fn get_oldest_event(events: &[Event]) -> Option<&Event> {
    events.iter().min_by_key(|event| event.created_at())
}

pub struct EventPaginator<'a> {
    client: &'a Client,
    filters: Vec<Filter>,
    oldest_timestamp: Option<Timestamp>,
    done: bool,
    timeout: Option<Duration>,
    page_size: usize,
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
        }
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
                if events.is_empty() || events.len() < self.page_size {
                    self.done = true;
                    return None;
                }

                // Update the oldest timestamp
                if let Some(oldest_event) = get_oldest_event(&events) {
                    self.oldest_timestamp = Some(oldest_event.created_at());
                }

                // Update the filters
                self.filters = updated_filters;
                Some(Ok(events))
            }
            Err(e) => {
                self.done = true;
                Some(Err(Error::Client(e)))
            }
        }
    }
}

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
    let filters: Vec<Filter> = event_ids.iter().map(|id| Filter::new().id(*id)).collect();
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
    let filter = Filter::new().kind(Kind::TextNote).custom_tag(
        SingleLetterTag::lowercase(Alphabet::E),
        vec![event_id.to_hex()],
    );
    let events = client.get_events_of(vec![filter], timeout).await?;
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
        tree.accept(vec![root]).unwrap();
        tree.accept(replies).unwrap();
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

        let filter = Filter::new().kind(Kind::TextNote).author(public_key);
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
}
