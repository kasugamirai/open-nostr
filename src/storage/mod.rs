use indexed_db_futures::{prelude::*, web_sys::DomException};
use serde::Serialize;
use wasm_bindgen::JsValue;


const DB_NAME: &str = "CAPYBARA_DB";
const DB_VERSION: u32 = 1;
const OBJECT_STORE_NAME: &str = "CAPYBARA_STORE";


pub async fn open_db() -> Result<IdbDatabase, DomException> {
    // Open my_db v1
    let mut db_req: OpenDbRequest = IdbDatabase::open_u32(DB_NAME, DB_VERSION)?;
    db_req.set_on_upgrade_needed(Some(|evt: &IdbVersionChangeEvent| -> Result<(), JsValue> {
        // Check if the object store exists; create it if it doesn't
        if !evt.db().object_store_names().any(|n| n == OBJECT_STORE_NAME) {
            evt.db().create_object_store(OBJECT_STORE_NAME)?;
        }
        Ok(())
    }));

    db_req.await
}

pub async fn add_data<T: Serialize>(key: &str, value: &T) -> Result<(), DomException> {
    let db = open_db().await?;
    let tx = db.transaction_on_one_with_mode(OBJECT_STORE_NAME, IdbTransactionMode::Readwrite)?;
    let store = tx.object_store(OBJECT_STORE_NAME)?;
    let value_str = serde_json::to_string(value)
        .map_err(|e| DomException::new_with_message(&format!("Serialization error: {}", e)).unwrap())?;
    let value_js: JsValue = JsValue::from_str(&value_str);
    store.add_key_val_owned(key, &value_js)?;
    tx.await.into_result()?;
    Ok(())
}

pub async fn delete_data(key: &str) -> Result<(), DomException> {
    let db = open_db().await?;
    let tx = db.transaction_on_one_with_mode(OBJECT_STORE_NAME, IdbTransactionMode::Readwrite)?;
    let store = tx.object_store(OBJECT_STORE_NAME)?;
    store.delete_owned(key)?;
    tx.await.into_result()?;
    Ok(())
}