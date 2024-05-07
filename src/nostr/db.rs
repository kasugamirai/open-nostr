use core::fmt;

use nostr_indexeddb::WebDatabase;

pub struct Database {
    pub db: WebDatabase,
}

pub enum DatabaseError {
    WebDatabaseError(nostr_indexeddb::IndexedDBError),
}

impl From<nostr_indexeddb::IndexedDBError> for DatabaseError {
    fn from(err: nostr_indexeddb::IndexedDBError) -> Self {
        Self::WebDatabaseError(err)
    }
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WebDatabaseError(err) => write!(f, "WebDatabaseError: {}", err),
        }
    }
}

impl Database {
    async fn new() -> Result<Self, DatabaseError> {
        let db = WebDatabase::open("nostr-sdk-indexeddb-test").await?;
        Ok(Self { db })
    }
}

impl Database {
    async fn save_event(&self, event: nostr_sdk::Event) -> Result<(), DatabaseError> {
        self.db.save_event(&event).await?;
        Ok(())
    }
}

//todo
