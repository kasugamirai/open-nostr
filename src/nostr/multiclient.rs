use cached::{Cached, TimedCache};
use nostr_indexeddb::WebDatabase;
use nostr_sdk::{Client, ClientBuilder, Event, Filter};
use std::collections::{HashMap, HashSet};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Mutex, Notify};

use crate::init::NOSTR_DB_NAME;
use crate::store::{self, CBWebDatabase, CAPYBASTR_DBNAME};

use super::utils::hash_filter;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Client error: {0}")]
    Client(#[from] nostr_sdk::client::Error),
    #[error("Store error: {0}")]
    Store(#[from] store::error::CBwebDatabaseError),
    #[error("IndexDb error: {0}")]
    IndexDb(#[from] nostr_indexeddb::IndexedDBError),
    #[error("Client not found")]
    ClientNotFound,
    #[error("query failed or not cached")]
    QueryFailedOrNotCahed,
}

#[derive(Debug, Clone)]
pub struct HashedClient {
    client: Arc<Client>,
    hash: u64,
}

unsafe impl Send for HashedClient {}
unsafe impl Sync for HashedClient {}

#[allow(clippy::arc_with_non_send_sync)]
impl HashedClient {
    pub async fn new(client: Client) -> Self {
        let hash = Self::_hash(&client).await;
        Self {
            client: Arc::new(client),
            hash,
        }
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

    pub fn client(&self) -> Arc<Client> {
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
    clients: Arc<Mutex<HashMap<String, HashedClient>>>,
}

impl Default for MultiClient {
    fn default() -> Self {
        Self::new()
    }
}

impl MultiClient {
    pub fn new() -> Self {
        Self {
            clients: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn register(&self, name: String, hc: HashedClient) {
        let mut clients = self.clients.lock().await;
        clients.insert(name, hc);
    }

    pub async fn change_key(&self, old_key: &str, new_key: String) -> Result<(), String> {
        let mut clients = self.clients.lock().await;
        if let Some(client) = clients.remove(old_key) {
            clients.insert(new_key, client);
            Ok(())
        } else {
            Err(format!("Client with key '{}' not found", old_key))
        }
    }

    pub async fn get_client(&self, name: &str) -> Option<HashedClient> {
        let clients = self.clients.lock().await;
        clients.get(name).cloned()
    }

    pub async fn get_or_create(&self, name: &str) -> Result<HashedClient, Error> {
        {
            let clients = self.clients.lock().await;
            if let Some(client) = clients.get(name) {
                return Ok(client.clone());
            }
        }

        let database = CBWebDatabase::open(CAPYBASTR_DBNAME).await?;
        let db = WebDatabase::open(NOSTR_DB_NAME).await?;
        let client_builder = ClientBuilder::new().database(db);
        let client = client_builder.build();
        let relay_set_info = database.get_relay_set(name.to_string()).await?;

        let mut hc = HashedClient::new(client).await;
        let relays: Vec<&str> = relay_set_info.relays.iter().map(|s| s.as_str()).collect();
        hc.add_relays(relays).await?;
        self.register(name.to_string(), hc.clone()).await;

        Ok(hc)
    }
}

type Cache = Arc<Mutex<TimedCache<NostrQuery, Vec<Event>>>>;
type PendingQueries = Arc<Mutex<HashSet<NostrQuery>>>;

#[derive(Debug, Clone)]
pub struct EventCache {
    cache: Cache,
    pending_queries: PendingQueries,
    notify: Arc<Notify>,
}

impl EventCache {
    pub fn new(lifespan: u64, capacity: usize) -> Self {
        Self {
            cache: Arc::new(Mutex::new(TimedCache::with_lifespan_and_capacity(
                lifespan, capacity,
            ))),
            pending_queries: Arc::new(Mutex::new(HashSet::new())),
            notify: Arc::new(Notify::new()),
        }
    }

    pub async fn cached_get_events_of(
        &self,
        client: &HashedClient,
        filters: Vec<Filter>,
        timeout: Option<Duration>,
    ) -> Result<Vec<Event>, Error> {
        let query = NostrQuery::new(client.hash(), &filters);

        // First, check the cache
        {
            let mut cache = self.cache.lock().await;
            if let Some(cached_result) = cache.cache_get(&query) {
                return Ok(cached_result.clone());
            }
        }

        // If not cached, check if query is already pending
        {
            let mut pending_queries = self.pending_queries.lock().await;
            if pending_queries.contains(&query) {
                drop(pending_queries); // Drop the lock before waiting
                self.notify.notified().await; // Wait for the ongoing query to finish
                let mut cache = self.cache.lock().await;
                if let Some(cached_result) = cache.cache_get(&query) {
                    return Ok(cached_result.clone());
                } else {
                    return Err(Error::QueryFailedOrNotCahed);
                }
            } else {
                pending_queries.insert(query.clone());
            }
        }

        // Perform the query
        let result = match client.client.get_events_of(filters.clone(), timeout).await {
            Ok(result) => result,
            Err(e) => {
                let mut pending_queries = self.pending_queries.lock().await;
                pending_queries.remove(&query);
                self.notify.notify_waiters();
                return Err(Error::Client(e));
            }
        };

        // Cache the result
        {
            let mut cache = self.cache.lock().await;
            cache.cache_set(query.clone(), result.clone());
        }

        // Remove from pending and notify other waiters
        {
            let mut pending_queries = self.pending_queries.lock().await;
            pending_queries.remove(&query);
        }
        self.notify.notify_waiters();

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nostr::fetch::EventPaginator;
    use nostr_sdk::FromBech32;
    use nostr_sdk::Kind;
    use nostr_sdk::PublicKey;
    //use tokio::sync::oneshot;
    use wasm_bindgen_futures::spawn_local;
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
        let multi_client = MultiClient::new();

        let public_key = PublicKey::from_bech32(
            "npub1xtscya34g58tk0z605fvr788k263gsu6cy9x0mhnm87echrgufzsevkk5s",
        )
        .unwrap();
        // Register the client
        multi_client
            .register("client1".to_string(), hashed_client.clone())
            .await;

        let filter: Filter = Filter::new()
            .kind(Kind::TextNote)
            .author(public_key)
            .limit(1);
        // Prepare filters
        let filters = vec![filter];

        let cache = EventCache::new(30, 300);

        // Perform the first query (this should not hit the cache)
        let result1 = cache
            .cached_get_events_of(
                &hashed_client,
                filters.clone(),
                Some(Duration::from_secs(10)),
            )
            .await;
        assert!(result1.is_ok());
        console_log!("First query result: {:?}", result1);

        // Perform the second query (this should hit the cache)
        let result2 = cache
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
        let multi_client = MultiClient::new();

        let public_key = PublicKey::from_bech32(
            "npub1xtscya34g58tk0z605fvr788k263gsu6cy9x0mhnm87echrgufzsevkk5s",
        )
        .unwrap();
        // Register the client
        multi_client
            .register("client1".to_string(), hashed_client.clone())
            .await;

        let filter: Filter = Filter::new()
            .kind(Kind::TextNote)
            .author(public_key)
            .limit(1);
        // Prepare filters
        let filters = vec![filter];

        let cache = EventCache::new(30, 300);

        for _ in 0..100 {
            let result1 = cache
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

    #[wasm_bindgen_test]
    async fn test_spawn_eventpaginator_multi_client() {
        let public_key = PublicKey::from_bech32(
            "npub1q0uulk2ga9dwkp8hsquzx38hc88uqggdntelgqrtkm29r3ass6fq8y9py9",
        )
        .unwrap();

        let client = nostr_sdk::Client::default();
        let mut hashed_client = HashedClient::new(client).await;
        let _ = hashed_client.add_relay("wss://relay.damus.io").await;
        let multi_client = MultiClient::new();
        multi_client
            .register("client1".to_string(), hashed_client.clone())
            .await;
        let filter: Filter = Filter::new().kind(Kind::TextNote).author(public_key);
        let hc = multi_client.get_client("client1").await.unwrap();
        let c = hc.client();

        // Create a oneshot channel.
        let (tx, rx) = tokio::sync::oneshot::channel();
        let mut paginator = EventPaginator::new(c, vec![filter], None, 10);

        spawn_local(async move {
            let e = paginator.next_page().await.unwrap();
            console_log!("Events: {:?}", e);
            tx.send(e.len()).unwrap(); // Send the length of `e` through the channel.
        });

        // Await the result from the oneshot channel.
        let len = rx.await.unwrap();
        assert!(len > 3, "Event length is not greater than 3");
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
