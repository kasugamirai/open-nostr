use cached::{Cached, TimedCache};
use core::fmt;
use nostr_indexeddb::WebDatabase;
use nostr_sdk::{ClientBuilder, Event, Filter};
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::rc::Rc;
use std::time::Duration;

use crate::init::NOSTR_DB_NAME;
use crate::store::{self, CBWebDatabase, CAPYBASTR_DBNAME};

use super::utils::hash_filter;

#[derive(Debug)]
pub enum Error {
    Client(nostr_sdk::client::Error),
    Store(store::error::CBwebDatabaseError),
    IndexDb(nostr_indexeddb::IndexedDBError),
    ClientNotFound,
}

impl From<nostr_indexeddb::IndexedDBError> for Error {
    fn from(e: nostr_indexeddb::IndexedDBError) -> Self {
        Error::IndexDb(e)
    }
}

impl From<nostr_sdk::client::Error> for Error {
    fn from(e: nostr_sdk::client::Error) -> Self {
        Error::Client(e)
    }
}

impl From<store::error::CBwebDatabaseError> for Error {
    fn from(e: store::error::CBwebDatabaseError) -> Self {
        Error::Store(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Client(e) => write!(f, "Client error: {}", e),
            Error::Store(e) => write!(f, "Store error: {}", e),
            Error::IndexDb(e) => write!(f, "IndexDb error: {}", e),
            Error::ClientNotFound => write!(f, "Client not found"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HashedClient {
    client: nostr_sdk::Client,
    hash: u64,
}

impl HashedClient {
    pub async fn new(client: nostr_sdk::Client) -> Self {
        let hash = Self::_hash(&client).await;
        Self { client, hash }
    }

    async fn _hash(client: &nostr_sdk::Client) -> u64 {
        let relays = client.relays().await;
        if relays.is_empty() {
            return 0;
        }
        let mut sorted_keys: Vec<_> = relays.keys().collect();
        sorted_keys.sort();
        let mut hasher = DefaultHasher::new();
        for key in sorted_keys {
            key.hash(&mut hasher);
        }
        hasher.finish()
    }

    pub fn client(&self) -> nostr_sdk::Client {
        self.client.clone()
    }

    pub fn hash(&self) -> u64 {
        self.hash
    }

    //connect after add_relay
    pub async fn add_relay(&mut self, url: &str) -> Result<bool, Error> {
        let result = self.client.add_relay(url).await?;
        if result {
            self.hash = Self::_hash(&self.client).await;
        }
        self.client.connect().await;
        //todo add db operation
        Ok(result)
    }

    //connect afeter add_relays
    pub async fn add_relays(&mut self, urls: Vec<&str>) -> Result<(), Error> {
        self.client.add_relays(urls).await?;
        self.hash = Self::_hash(&self.client).await;
        self.client.connect().await;
        //todo add db operation
        Ok(())
    }

    pub async fn remove_relay(&mut self, url: &str) -> Result<(), Error> {
        self.client.remove_relay(url).await?;
        self.hash = Self::_hash(&self.client).await;
        Ok(())
    }

    pub async fn remove_all_relays(&mut self) -> Result<(), Error> {
        self.client.remove_all_relays().await?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct NostrQuery {
    client_hash: u64,
    filters_hash: u64,
}

impl NostrQuery {
    pub fn new(client_hash: u64, filters: &Vec<Filter>) -> Self {
        let filters_hash = hash_filter(filters);
        Self {
            client_hash,
            filters_hash,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MultiClient {
    clients: Rc<RefCell<HashMap<String, HashedClient>>>,
    cache: TimedCache<NostrQuery, Vec<Event>>,
}

impl Default for MultiClient {
    fn default() -> Self {
        Self::new()
    }
}

impl MultiClient {
    pub fn new() -> Self {
        Self {
            clients: Rc::new(RefCell::new(HashMap::new())),
            cache: TimedCache::with_lifespan_and_capacity(300, 300), // Initialize cache
        }
    }

    pub fn register(&self, name: String, hc: HashedClient) {
        let mut clients = self.clients.borrow_mut();
        clients.insert(name, hc);
    }

    pub fn get_client(&self, name: &str) -> Option<HashedClient> {
        let clients = self.clients.borrow();
        clients.get(name).cloned()
    }

    pub async fn get_or_create(&mut self, name: &str) -> Result<HashedClient, Error> {
        match self.get_client(name) {
            Some(client) => Ok(client),
            None => {
                let database = CBWebDatabase::open(CAPYBASTR_DBNAME).await?;
                let db = WebDatabase::open(NOSTR_DB_NAME).await?;
                let client_builder = ClientBuilder::new().database(db);
                let client = client_builder.build();
                let relay_set_info = database.get_relay_set(name.to_string()).await?;
                // client.add_relays(relay_set_info.relays).await.unwrap();
                // client.connect().await;
                let mut hc = HashedClient::new(client).await;
                let relays: Vec<&str> = relay_set_info.relays.iter().map(|s| s.as_str()).collect();
                hc.add_relays(relays).await?;
                self.register(name.to_string(), hc);
                // The return type has been changed to remove Option
                self.get_client(name).ok_or(Error::ClientNotFound)
            }
        }
    }

    pub async fn cached_get_events_of(
        &mut self,
        client: &HashedClient,
        filters: Vec<Filter>,
        timeout: Option<Duration>,
    ) -> Result<Vec<Event>, Error> {
        let query = NostrQuery::new(client.hash(), &filters);

        if let Some(cached_result) = self.cache.cache_get(&query) {
            return Ok(cached_result.clone());
        }

        let result = client
            .client
            .get_events_of(filters.clone(), timeout)
            .await?;

        self.cache.cache_set(query, result.clone());

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nostr_sdk::FromBech32;
    use nostr_sdk::Kind;
    use nostr_sdk::PublicKey;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);
    use wasm_bindgen_test::console_log;

    #[wasm_bindgen_test]
    async fn test_hashed_client1() {
        let client = nostr_sdk::Client::default();
        let hc = HashedClient::new(client).await;
        assert_eq!(hc.hash(), 0);
    }

    #[wasm_bindgen_test]
    async fn test_hashed_client2() {
        let client = nostr_sdk::Client::default();
        let mut hc = HashedClient::new(client).await;
        assert_eq!(hc.hash(), 0);
        let result = hc.add_relay("wss://relay.damus.io").await;
        assert!(result.is_ok());
        console_log!("hash: {:?}", hc.hash());
        assert_ne!(hc.hash(), 0);
    }

    #[wasm_bindgen_test]
    async fn test_hashed_client3() {
        let client = nostr_sdk::Client::default();
        let mut hc = HashedClient::new(client).await;
        assert_eq!(hc.hash(), 0);
        let _ = hc.add_relay("wss://relay.damus.io").await;
        console_log!("hash: {:?}", hc.hash());
        let _ = hc.add_relay("wss://nos.lol").await;
        console_log!("hash: {:?}", hc.hash());
        assert_ne!(hc.hash(), 0);
    }

    #[wasm_bindgen_test]
    async fn test_multi_client_cached_query() {
        let client = nostr_sdk::Client::default();
        let mut hashed_client = HashedClient::new(client).await;
        let _ = hashed_client.add_relay("wss://relay.damus.io").await;
        let mut multi_client = MultiClient::new();

        let public_key = PublicKey::from_bech32(
            "npub1xtscya34g58tk0z605fvr788k263gsu6cy9x0mhnm87echrgufzsevkk5s",
        )
        .unwrap();
        // Register the client
        multi_client.register("client1".to_string(), hashed_client.clone());

        let filter: Filter = Filter::new()
            .kind(Kind::TextNote)
            .author(public_key)
            .limit(1);
        // Prepare filters
        let filters = vec![filter];

        // Perform the first query (this should not hit the cache)
        let result1 = multi_client
            .cached_get_events_of(
                &hashed_client,
                filters.clone(),
                Some(Duration::from_secs(10)),
            )
            .await;
        assert!(result1.is_ok());
        console_log!("First query result: {:?}", result1);

        // Perform the second query (this should hit the cache)
        let result2 = multi_client
            .cached_get_events_of(&hashed_client, filters, Some(Duration::from_secs(10)))
            .await;
        assert!(result2.is_ok());
        console_log!("Second query result: {:?}", result2);

        // The results should be the same and the second one should come from the cache
        assert_eq!(result1.unwrap(), result2.unwrap());
    }

    #[wasm_bindgen_test]
    async fn test_multi_client_cached_query_many_times() {
        let client = nostr_sdk::Client::default();
        let mut hashed_client = HashedClient::new(client).await;
        let _ = hashed_client.add_relay("wss://relay.damus.io").await;
        let mut multi_client = MultiClient::new();

        let public_key = PublicKey::from_bech32(
            "npub1xtscya34g58tk0z605fvr788k263gsu6cy9x0mhnm87echrgufzsevkk5s",
        )
        .unwrap();
        // Register the client
        multi_client.register("client1".to_string(), hashed_client.clone());

        let filter: Filter = Filter::new()
            .kind(Kind::TextNote)
            .author(public_key)
            .limit(1);
        // Prepare filters
        let filters = vec![filter];

        for _ in 0..100 {
            let result1 = multi_client
                .cached_get_events_of(
                    &hashed_client,
                    filters.clone(),
                    Some(Duration::from_secs(10)),
                )
                .await;
            assert!(result1.is_ok());
            console_log!("First query result: {:?}", result1);
        }
    }

    // this test is no needed
    // #[wasm_bindgen_test]
    // async fn test_multi_client_cached_query_many_times_no_cache() {
    //     let client = nostr_sdk::Client::default();
    //     let mut hashed_client = HashedClient::new(client).await;
    //     hashed_client.add_relay("wss://relay.damus.io").await;

    //     let public_key = PublicKey::from_bech32(
    //         "npub1xtscya34g58tk0z605fvr788k263gsu6cy9x0mhnm87echrgufzsevkk5s",
    //     )
    //     .unwrap();
    //     // Register the client

    //     let filter: Filter = Filter::new()
    //         .kind(Kind::TextNote)
    //         .author(public_key)
    //         .limit(1);
    //     // Prepare filters
    //     let filters = vec![filter];

    //     for _ in 0..10 {
    //         let result1 = hashed_client.client.get_events_of( filters.clone(), Some(Duration::from_secs(10))).await;
    //         assert!(result1.is_ok());
    //         console_log!("First query result: {:?}", result1);
    //     }
    // }
}
