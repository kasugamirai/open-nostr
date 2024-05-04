use std::fmt;
use std::rc::Rc;

use indexed_db_futures::{prelude::*, web_sys::DomException};
use serde::{de::DeserializeOwned, Serialize};
use serde_wasm_bindgen;
use wasm_bindgen::JsValue;

const DB_NAME: &str = "CAPYBASTR_DB";
const DB_VERSION: u32 = 1;

#[derive(Debug)]
pub enum CapybastrError {
    Dom(DomException),
    DomError(String),
    SerializationError(serde_json::Error),
    DeserializationError(serde_wasm_bindgen::Error),
}

impl fmt::Display for CapybastrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CapybastrError::Dom(e) => write!(f, "DOM Exception: {}", e.message().as_str()),
            CapybastrError::DomError(msg) => write!(f, "DOM Error: {}", msg),
            CapybastrError::SerializationError(e) => write!(f, "Serialization Error: {}", e),
            CapybastrError::DeserializationError(e) => write!(f, "Deserialization Error: {}", e),
        }
    }
}

impl From<DomException> for CapybastrError {
    fn from(e: DomException) -> Self {
        CapybastrError::Dom(e)
    }
}

impl From<serde_json::Error> for CapybastrError {
    fn from(e: serde_json::Error) -> Self {
        CapybastrError::SerializationError(e)
    }
}

impl From<serde_wasm_bindgen::Error> for CapybastrError {
    fn from(e: serde_wasm_bindgen::Error) -> Self {
        CapybastrError::DeserializationError(e)
    }
}

#[derive(Debug, Clone)]
pub struct CapybastrDb {
    db: Rc<IdbDatabase>,
    store_name: String,
}

impl CapybastrDb {
    pub async fn new(store_name: String) -> Result<Self, CapybastrError> {
        let db = CapybastrDb::open(store_name.clone()).await?;
        Ok(Self {
            db: Rc::new(db),
            store_name,
        })
    }

    async fn open(store_name: String) -> Result<IdbDatabase, CapybastrError> {
        let mut db_req = IdbDatabase::open_u32(DB_NAME, DB_VERSION)?;
        db_req.set_on_upgrade_needed(Some(
            move |evt: &IdbVersionChangeEvent| -> Result<(), JsValue> {
                if !evt.db().object_store_names().any(|n| n == store_name) {
                    evt.db().create_object_store(&store_name)?;
                }
                Ok(())
            },
        ));

        db_req.await.map_err(CapybastrError::from)
    }

    pub async fn add_data<T: Serialize>(&self, key: &str, value: &T) -> Result<(), CapybastrError> {
        let tx = self
            .db
            .transaction_on_one_with_mode(&self.store_name, IdbTransactionMode::Readwrite)?;
        let store = tx.object_store(&self.store_name)?;
        let value_str = serde_json::to_string(value).map_err(CapybastrError::from)?;
        let value_js: JsValue = JsValue::from_str(&value_str);
        store.add_key_val_owned(key, &value_js)?;
        tx.await.into_result().map_err(CapybastrError::from)
    }

    pub async fn delete_data(&self, key: &str) -> Result<(), CapybastrError> {
        let tx = self
            .db
            .transaction_on_one_with_mode(&self.store_name, IdbTransactionMode::Readwrite)?;
        let store = tx.object_store(&self.store_name)?;
        store.delete_owned(key)?;
        tx.await.into_result().map_err(CapybastrError::from)
    }

    pub async fn read_data<T: DeserializeOwned>(&self, key: &str) -> Result<T, CapybastrError> {
        let tx = self
            .db
            .transaction_on_one_with_mode(&self.store_name, IdbTransactionMode::Readonly)?;
        let store = tx.object_store(&self.store_name)?;
        let value_js_opt = store.get_owned(key)?.await.map_err(CapybastrError::from)?;

        let value_js = value_js_opt
            .ok_or_else(|| CapybastrError::DomError(format!("No entry found for key: {}", key)))?;

        serde_wasm_bindgen::from_value(value_js).map_err(CapybastrError::DeserializationError)
    }
}
