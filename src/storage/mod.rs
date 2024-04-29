use indexed_db_futures::{prelude::*, web_sys::DomException};
use serde::Serialize;
use wasm_bindgen::JsValue;

const DB_NAME: &str = "CAPYBASTR_DB";
const DB_VERSION: u32 = 1;


pub enum Error{
    //todo
}

#[derive(Debug, Clone)]
pub struct CapybastrDb {
    store_name: String,
}

impl CapybastrDb {
    pub fn new(store_name: impl Into<String>) -> Self {
        Self {
            store_name: store_name.into(),
        }
    }

    async fn open(&self) -> Result<IdbDatabase, DomException> {
        let mut db_req: OpenDbRequest = IdbDatabase::open_u32(DB_NAME, DB_VERSION)?;
        let store_name = self.store_name.clone();
        db_req.set_on_upgrade_needed(Some(
            move |evt: &IdbVersionChangeEvent| -> Result<(), JsValue> {
                // Check if the object store exists; create it if it doesn't
                if !evt.db().object_store_names().any(|n| n == store_name) {
                    evt.db().create_object_store(&store_name)?;
                }
                Ok(())
            },
        ));

        db_req.await
    }

    pub async fn add_data<T: Serialize>(&self, key: &str, value: &T) -> Result<(), DomException> {
        let db = self.open().await?;
        let tx = db.transaction_on_one_with_mode(&self.store_name, IdbTransactionMode::Readwrite)?;
        let store = tx.object_store(&self.store_name)?;
        let value_str = serde_json::to_string(value).map_err(|e| {
            DomException::new_with_message(&format!("Serialization error: {}", e)).unwrap()
        })?;
        let value_js: JsValue = JsValue::from_str(&value_str);
        store.add_key_val_owned(key, &value_js)?;
        tx.await.into_result()?;
        Ok(())
    }

    pub async fn delete_data(&self, key: &str) -> Result<(), DomException> {
        let db = self.open().await?;
        let tx = db.transaction_on_one_with_mode(&self.store_name, IdbTransactionMode::Readwrite)?;
        let store = tx.object_store(&self.store_name)?;
        store.delete_owned(key)?;
        tx.await.into_result()?;
        Ok(())
    }
}

