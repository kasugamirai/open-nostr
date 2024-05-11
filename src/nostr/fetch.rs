use nostr_indexeddb::database::Order;
use nostr_indexeddb::WebDatabase;
use nostr_sdk::{client, Client, Event, Filter, FilterOptions};
use nostr_sdk::{NostrDatabase, Timestamp};

#[derive(Debug)]
pub enum FetcherError {
    NostrSdkClientError(client::Error),
    DatabaseError(nostr_indexeddb::IndexedDBError),
}

impl From<client::Error> for FetcherError {
    fn from(err: client::Error) -> Self {
        Self::NostrSdkClientError(err)
    }
}

impl From<nostr_indexeddb::IndexedDBError> for FetcherError {
    fn from(err: nostr_indexeddb::IndexedDBError) -> Self {
        Self::DatabaseError(err)
    }
}

impl std::fmt::Display for FetcherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NostrSdkClientError(err) => write!(f, "NostrSdkClientError: {}", err),
            Self::DatabaseError(err) => write!(f, "DatabaseError: {}", err),
        }
    }
}

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

    pub async fn get_events_from_db(
        &self,
        db: WebDatabase,
        filters: Filter,
    ) -> Result<Vec<Event>, FetcherError> {
        let events = db.query(vec![filters], Order::Desc).await?;
        Ok(events)
    }

    pub async fn get_events(
        &self,
        client: Client,
        db: WebDatabase,
        filters: Vec<Filter>,
        save_opts: bool,
        opts: FilterOptions,
        timeout: Option<std::time::Duration>,
    ) -> Result<Vec<Event>, FetcherError> {
        let events = client
            .get_events_of_with_opts(filters, timeout, opts)
            .await?;

        if save_opts {
            for event in events.iter() {
                db.save_event(event).await?;
            }
        }
        Ok(events)
    }

    pub async fn sync_data_saved(
        &self,
        client: Client,
        db: WebDatabase,
        mut filters: Vec<Filter>,
        opts: FilterOptions,
        timeout: Option<std::time::Duration>,
        //filters_transformer: impl Fn(&Vec<Filter>) -> Vec<Filter>,
        //enum_stop_conditions: Vec<impl Fn(&Vec<Event>) -> bool>,
    ) -> Result<(), FetcherError> {
        //let mut all_events = Vec::new();
        let conditions = vec![StopCondition::NoEvents, StopCondition::DataInDb];

        'outer: loop {
            let events = self
                .get_events(
                    client.clone(),
                    db.clone(),
                    filters.clone(),
                    true,
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
            filters = self.filters_transformer(&filters, &events);
        }

        Ok(())
    }

    pub fn filters_transformer(&self, filters: &[Filter], events: &[Event]) -> Vec<Filter> {
        let earliest_event_date = get_earliest_event_date(events);
        let mut updated_filters = Vec::new();
        for filter in filters.iter() {
            let updated_filter = <nostr_sdk::Filter as Clone>::clone(filter)
                .until(earliest_event_date)
                .limit(500);
            updated_filters.push(updated_filter);
        }
        updated_filters
    }
}

fn get_earliest_event_date(events: &[Event]) -> Timestamp {
    let mut event_time: Option<Timestamp> = None;
    for event in events.iter() {
        let e_time = event.created_at();
        if event_time.is_none() || e_time < event_time.unwrap() {
            event_time = Some(e_time);
        }
    }
    event_time.unwrap()
}

enum StopCondition {
    NoEvents,
    DataInDb,
}

impl StopCondition {
    pub async fn check(&self, events: &[Event], db: &WebDatabase) -> bool {
        match self {
            Self::NoEvents => events.is_empty(),
            Self::DataInDb => {
                let mut filters = Vec::new();
                for event in events.iter() {
                    let filter = vec![Filter::new().id(event.id)];
                    filters.extend(filter);
                }

                let ret = db.query(filters, Order::Desc).await.unwrap();
                if !ret.is_empty() {
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

        let events = fetcher
            .get_events(client, db, vec![filter], true, opts, timeout)
            .await
            .unwrap();

        assert_eq!(events.len(), 2);
    }

    #[wasm_bindgen_test]

    async fn test_sync_data_saved() {
        let public_key = PublicKey::from_bech32(
            "npub1drvpzev3syqt0kjrls50050uzf25gehpz9vgdw08hvex7e0vgfeq0eseet",
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
            .limit(500);
        let opts = FilterOptions::default();
        let timeout = None;
        let ret = fetcher
            .sync_data_saved(client, db, vec![filter], opts, timeout)
            .await;
        assert!(ret.is_ok());
    }
}
