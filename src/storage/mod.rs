use std::rc::Rc;

use indexed_db_futures::{prelude::*, web_sys::DomException};
use serde::Serialize;
//use std::sync::Arc;
use wasm_bindgen::JsValue;

const DB_NAME: &str = "CAPYBASTR_DB";
const DB_VERSION: u32 = 1;

#[derive(Debug)]
pub enum CapybastrError {
    Dom(DomException),
    SerializationError(serde_json::Error),
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
}
