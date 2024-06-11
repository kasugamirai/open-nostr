pub mod error;
pub mod subscription;
pub mod user;

use std::future::IntoFuture;
use std::rc::Rc;

pub use error::CBwebDatabaseError;
use indexed_db_futures::idb_object_store::IdbObjectStoreParameters;
use indexed_db_futures::request::{IdbOpenDbRequestLike, OpenDbRequest};
use indexed_db_futures::web_sys::IdbTransactionMode;
use indexed_db_futures::{IdbDatabase, IdbKeyPath, IdbQuerySource, IdbVersionChangeEvent};
use serde_wasm_bindgen::{from_value, to_value};
use subscription::{CustomSub, RelaySet};
pub use user::{AccountType, User};
use wasm_bindgen::JsValue;
use web_sys::IdbIndexParameters;

pub const CAPYBASTR_DBNAME: &str = "capybastr-db";

const CURRENT_DB_VERSION: u32 = 2;
const RELAY_SET_CF: &str = "relay-set";
const CUSTOM_SUB_CF: &str = "custom-sub";
const USER_CF: &str = "user";
const MISC_CF: &str = "misc";

// Some entries keys & values
pub const DEFAULT_RELAY_SET_KEY: &str = "default"; // This record cannot be removed

#[derive(Clone)]
pub struct CBWebDatabase {
    db: Rc<IdbDatabase>,
}

impl CBWebDatabase {
    async fn migration(&mut self) -> Result<(), CBwebDatabaseError> {
        let name: String = self.db.name();
        let old_version: u32 = self.db.version() as u32;
        tracing::info!("start migration with version {}", old_version);

        let mut db_req: OpenDbRequest = IdbDatabase::open_u32(&name, CURRENT_DB_VERSION)?;
        db_req.set_on_upgrade_needed(Some(
            move |evt: &IdbVersionChangeEvent| -> Result<(), JsValue> {
                // Apply migration 1->2
                if old_version <= 1 {
                    {
                        // Init user store
                        let mut create_store_params = IdbObjectStoreParameters::new();
                        let key_path = IdbKeyPath::str("name");
                        create_store_params.key_path(Some(&key_path));
                        let user_store = evt
                            .db()
                            .create_object_store_with_params(USER_CF, &create_store_params)?;
                        user_store.create_index_with_params(
                            "name",
                            &key_path,
                            IdbIndexParameters::new().unique(true),
                        )?;
                    }

                    {
                        // Init relay-set store
                        let mut create_store_params = IdbObjectStoreParameters::new();
                        let key_path = IdbKeyPath::str("name");
                        create_store_params.key_path(Some(&key_path));
                        let relay_set_store = evt
                            .db()
                            .create_object_store_with_params(RELAY_SET_CF, &create_store_params)?;
                        relay_set_store.create_index_with_params(
                            "name",
                            &key_path,
                            IdbIndexParameters::new().unique(true),
                        )?;
                    }

                    {
                        // Init custom-sub store
                        let mut create_store_params = IdbObjectStoreParameters::new();
                        let key_path = IdbKeyPath::str("name");
                        create_store_params.key_path(Some(&key_path));
                        let custom_sub_store = evt
                            .db()
                            .create_object_store_with_params(CUSTOM_SUB_CF, &create_store_params)?;
                        custom_sub_store.create_index_with_params(
                            "name",
                            &key_path,
                            IdbIndexParameters::new().unique(true),
                        )?;
                    }

                    {
                        // Init misc store
                        let _misc_store = evt.db().create_object_store(MISC_CF)?;
                    }
                }
                Ok(())
            },
        ));

        self.db.close();

        let mut db_req: OpenDbRequest = IdbDatabase::open_u32(&name, CURRENT_DB_VERSION)?;
        db_req.set_on_upgrade_needed(Some(
            move |evt: &IdbVersionChangeEvent| -> Result<(), JsValue> {
                // Sanity check.
                // There should be no upgrade needed since the database should have already been
                // upgraded to the latest version.
                panic!(
                    "Opening database that was not fully upgraded: \
                         DB version: {}; latest version: {CURRENT_DB_VERSION}",
                    evt.old_version()
                )
            },
        ));

        self.db = Rc::new(db_req.into_future().await?);

        Ok(())
    }

    pub async fn open<S>(name: S) -> Result<Self, CBwebDatabaseError>
    where
        S: AsRef<str>,
    {
        let mut this = Self {
            db: Rc::new(IdbDatabase::open(name.as_ref())?.into_future().await?),
        };

        this.migration().await?;
        Ok(this)
    }

    pub async fn save_relay_set(&self, relay_set: RelaySet) -> Result<(), CBwebDatabaseError> {
        let tx = self
            .db
            .transaction_on_one_with_mode(RELAY_SET_CF, IdbTransactionMode::Readwrite)?;

        let store = tx.object_store(RELAY_SET_CF)?;
        let value = to_value(&relay_set).map_err(CBwebDatabaseError::DeserializationError)?;
        store.put_val(&value)?;

        tx.await.into_result()?;
        Ok(())
    }

    pub async fn remove_relay_set(&self, name: String) -> Result<(), CBwebDatabaseError> {
        if name == DEFAULT_RELAY_SET_KEY {
            return Err(CBwebDatabaseError::InvalidOperation(
                "Cannot remove the default relay set".to_string(),
            ));
        }

        // Start a transaction for both stores
        let tx = self.db.transaction_on_multi_with_mode(
            &[RELAY_SET_CF, CUSTOM_SUB_CF],
            IdbTransactionMode::Readwrite,
        )?;

        // Remove the relay set from the relay set store
        let relay_set_store = tx.object_store(RELAY_SET_CF)?;
        relay_set_store.delete(&JsValue::from_str(&name))?;

        // Update the corresponding records in the custom_sub store
        let custom_sub_store = tx.object_store(CUSTOM_SUB_CF)?;
        let custom_subs = custom_sub_store.get_all()?.await?;

        for sub_value in custom_subs.iter() {
            let mut custom_sub: CustomSub =
                from_value(sub_value.clone()).map_err(CBwebDatabaseError::DeserializationError)?;

            if custom_sub.relay_set == name {
                custom_sub.relay_set = DEFAULT_RELAY_SET_KEY.to_string();
                let updated_sub_value =
                    to_value(&custom_sub).map_err(CBwebDatabaseError::DeserializationError)?;

                custom_sub_store.put_val(&updated_sub_value)?;
            }
        }

        // Commit the transaction
        tx.await.into_result()?;
        Ok(())
    }

    pub async fn relay_set_change(
        &self,
        old_name: String,
        new_relay_set: RelaySet,
    ) -> Result<(), CBwebDatabaseError> {
        if old_name == DEFAULT_RELAY_SET_KEY && new_relay_set.name != DEFAULT_RELAY_SET_KEY {
            return Err(CBwebDatabaseError::InvalidOperation(
                "Cannot rename the default relay set".to_string(),
            ));
        }

        // Start a transaction for both stores
        let tx = self.db.transaction_on_multi_with_mode(
            &[RELAY_SET_CF, CUSTOM_SUB_CF],
            IdbTransactionMode::Readwrite,
        )?;

        // Update the name in the relay set store
        let relay_set_store = tx.object_store(RELAY_SET_CF)?;
        let relay_set_key = JsValue::from_str(&old_name);
        if let Some(mut relay_set_value) = relay_set_store.get(&relay_set_key)?.await? {
            // Deserialize the RelaySet
            let mut relay_set: RelaySet = from_value(relay_set_value.clone())
                .map_err(CBwebDatabaseError::DeserializationError)?;

            // Update the name
            relay_set.name.clone_from(&new_relay_set.name);
            relay_set.relays.clone_from(&new_relay_set.relays);
            relay_set_value =
                to_value(&relay_set).map_err(CBwebDatabaseError::DeserializationError)?;

            // Put the updated entry
            relay_set_store.put_val(&relay_set_value)?;
        } else {
            return Err(CBwebDatabaseError::NotFound);
        }

        // Update the corresponding records in the custom_sub store
        let custom_sub_store = tx.object_store(CUSTOM_SUB_CF)?;
        let custom_subs = custom_sub_store.get_all()?.await?;

        for sub_value in custom_subs.iter() {
            let mut custom_sub: CustomSub =
                from_value(sub_value.clone()).map_err(CBwebDatabaseError::DeserializationError)?;

            if custom_sub.relay_set == old_name {
                custom_sub.relay_set.clone_from(&new_relay_set.name);
                let updated_sub_value =
                    to_value(&custom_sub).map_err(CBwebDatabaseError::DeserializationError)?;

                // Put the updated entry
                custom_sub_store.put_val(&updated_sub_value)?;
            }
        }

        // Commit the transaction
        tx.await.into_result()?;
        Ok(())
    }

    pub async fn get_relay_set(&self, name: String) -> Result<RelaySet, CBwebDatabaseError> {
        let tx = self
            .db
            .transaction_on_one_with_mode(RELAY_SET_CF, IdbTransactionMode::Readonly)?;

        let store = tx.object_store(RELAY_SET_CF)?;
        let key = JsValue::from_str(&name);
        match store.get(&key)?.await? {
            Some(value) => match from_value::<RelaySet>(value.clone()) {
                Ok(relay_set) => Ok(relay_set),
                Err(e) => {
                    tracing::error!("Error deserializing RelaySet: {:?}", e);
                    Err(CBwebDatabaseError::DeserializationError(e))
                }
            },
            None => Err(CBwebDatabaseError::NotFound),
        }
    }

    pub async fn get_all_relay_sets(&self) -> Result<Vec<RelaySet>, CBwebDatabaseError> {
        let tx = self
            .db
            .transaction_on_one_with_mode(RELAY_SET_CF, IdbTransactionMode::Readonly)?;

        let store = tx.object_store(RELAY_SET_CF)?;
        let value = store.get_all()?.await?;

        let mut relay_sets = Vec::new();

        for v in value.iter() {
            match from_value::<RelaySet>(v.clone()) {
                Ok(relay_set) => relay_sets.push(relay_set),
                Err(e) => {
                    tracing::error!("Error deserializing RelaySets: {:?}", e);
                    return Err(CBwebDatabaseError::DeserializationError(e));
                }
            }
        }

        Ok(relay_sets)
    }

    pub async fn save_custom_sub(&self, custom_sub: CustomSub) -> Result<(), CBwebDatabaseError> {
        let tx = self
            .db
            .transaction_on_one_with_mode(CUSTOM_SUB_CF, IdbTransactionMode::Readwrite)?;

        let store = tx.object_store(CUSTOM_SUB_CF)?;
        let value = to_value(&custom_sub).map_err(CBwebDatabaseError::DeserializationError)?;
        store.put_val(&value)?;

        tx.await.into_result()?;
        Ok(())
    }

    pub async fn remove_custom_sub(&self, name: String) -> Result<(), CBwebDatabaseError> {
        let tx = self
            .db
            .transaction_on_one_with_mode(CUSTOM_SUB_CF, IdbTransactionMode::Readwrite)?;

        let store = tx.object_store(CUSTOM_SUB_CF)?;
        store.delete(&JsValue::from_str(&name))?;

        tx.await.into_result()?;
        Ok(())
    }
    pub async fn update_custom_sub(
        &self,
        old_name: String,
        custom_sub: CustomSub,
    ) -> Result<(), CBwebDatabaseError> {
        tracing::info!("Update custom sub: {:?}", custom_sub);
        let old_custom_sub = self.get_custom_sub(old_name.clone()).await?;
        if old_custom_sub.name != custom_sub.name {
            // If you remove the sub first, when there is 1 sub in the sub list, the default sub will be inserted. If the default sub is modified at this time, then there is a bug here, and you will find that the modification is unsuccessful, so you need to save the modification first. sub and then remove the old sub
            self.save_custom_sub(custom_sub).await?;
            self.remove_custom_sub(old_name.clone()).await?;
        } else {
            let tx = self
                .db
                .transaction_on_one_with_mode(CUSTOM_SUB_CF, IdbTransactionMode::Readwrite)
                .map_err(|e| {
                    tracing::error!("Failed to start transaction: {:?}", e);
                    CBwebDatabaseError::Dom(e.to_string().into())
                })?;

            let store = tx.object_store(CUSTOM_SUB_CF)?;
            let value = to_value(&custom_sub).map_err(CBwebDatabaseError::DeserializationError)?;

            store.put_val(&value).map_err(|e| {
                tracing::error!("Failed to put value in store: {:?}", e);
                CBwebDatabaseError::Dom(e.to_string().into())
            })?;

            tx.await.into_result().map_err(|e| {
                tracing::error!("Transaction failed: {:?}", e);
                CBwebDatabaseError::Dom(e.to_string().into())
            })?;
        }
        Ok(())
    }
    pub async fn get_custom_sub(&self, name: String) -> Result<CustomSub, CBwebDatabaseError> {
        let tx = self
            .db
            .transaction_on_one_with_mode(CUSTOM_SUB_CF, IdbTransactionMode::Readonly)?;

        let store = tx.object_store(CUSTOM_SUB_CF)?;
        let key = JsValue::from_str(&name);
        match store.get(&key)?.await? {
            Some(value) => match from_value::<CustomSub>(value.clone()) {
                Ok(custom_sub) => Ok(custom_sub),
                Err(e) => {
                    tracing::error!("Error deserializing CustomSub: {:?}", e);
                    Err(CBwebDatabaseError::DeserializationError(e))
                }
            },
            None => Err(CBwebDatabaseError::NotFound),
        }
    }

    pub async fn get_all_subs(&self) -> Result<Vec<CustomSub>, CBwebDatabaseError> {
        let tx = self
            .db
            .transaction_on_one_with_mode(CUSTOM_SUB_CF, IdbTransactionMode::Readonly)?;

        let store = tx.object_store(CUSTOM_SUB_CF)?;
        let value = store.get_all()?.await?;

        let mut subs = Vec::new();

        for v in value.iter() {
            match from_value::<CustomSub>(v.clone()) {
                Ok(custom_sub) => subs.push(custom_sub),
                Err(e) => {
                    tracing::error!("Error deserializing CustomSub: {:?}", e);
                    return Err(CBwebDatabaseError::DeserializationError(e));
                }
            }
        }

        Ok(subs)
    }

    pub async fn save_user(&self, user: User) -> Result<(), CBwebDatabaseError> {
        let tx = self
            .db
            .transaction_on_one_with_mode(USER_CF, IdbTransactionMode::Readwrite)?;

        let store = tx.object_store(USER_CF)?;
        let value = to_value(&user).map_err(CBwebDatabaseError::DeserializationError)?;
        store.put_val(&value)?;

        tx.await.into_result()?;
        Ok(())
    }

    pub async fn remove_user(&self, name: String) -> Result<(), CBwebDatabaseError> {
        let tx = self
            .db
            .transaction_on_one_with_mode(USER_CF, IdbTransactionMode::Readwrite)?;

        let store = tx.object_store(USER_CF)?;
        store.delete(&JsValue::from_str(&name))?;

        tx.await.into_result()?;
        Ok(())
    }

    pub async fn get_user(&self, name: String) -> Result<User, CBwebDatabaseError> {
        let tx = self
            .db
            .transaction_on_one_with_mode(USER_CF, IdbTransactionMode::Readonly)?;

        let store = tx.object_store(USER_CF)?;
        let key = JsValue::from_str(&name);
        match store.get(&key)?.await? {
            Some(value) => match from_value::<User>(value.clone()) {
                Ok(user) => Ok(user),
                Err(e) => {
                    tracing::error!("Error deserializing User: {:?}", e);
                    Err(CBwebDatabaseError::DeserializationError(e))
                }
            },
            None => Err(CBwebDatabaseError::NotFound),
        }
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, CBwebDatabaseError> {
        let tx = self
            .db
            .transaction_on_one_with_mode(USER_CF, IdbTransactionMode::Readonly)?;

        let store = tx.object_store(USER_CF)?;
        let value = store.get_all()?.await?;

        let mut users = Vec::new();

        for v in value.iter() {
            match from_value::<User>(v.clone()) {
                Ok(user) => users.push(user),
                Err(e) => {
                    tracing::error!("Error deserializing User: {:?}", e);
                    return Err(CBwebDatabaseError::DeserializationError(e));
                }
            }
        }

        Ok(users)
    }

    pub async fn get_misc(&self, key: String) -> Result<Option<String>, CBwebDatabaseError> {
        let tx = self
            .db
            .transaction_on_one_with_mode(MISC_CF, IdbTransactionMode::Readonly)?;

        let store = tx.object_store(MISC_CF)?;
        let key = JsValue::from_str(&key);
        match store.get(&key)?.await? {
            Some(value) => match from_value::<String>(value.clone()) {
                Ok(s) => Ok(Some(s)),
                Err(e) => {
                    tracing::error!("Error deserializing String: {:?}", e);
                    Err(CBwebDatabaseError::DeserializationError(e))
                }
            },
            None => Ok(None),
        }
    }

    pub async fn remove_misc(&self, key: String) -> Result<(), CBwebDatabaseError> {
        let tx = self
            .db
            .transaction_on_one_with_mode(MISC_CF, IdbTransactionMode::Readwrite)?;

        let store = tx.object_store(MISC_CF)?;
        store.delete(&JsValue::from_str(&key))?;

        tx.await.into_result()?;
        Ok(())
    }

    pub async fn save_misc(&self, key: String, value: String) -> Result<(), CBwebDatabaseError> {
        let tx = self
            .db
            .transaction_on_one_with_mode(MISC_CF, IdbTransactionMode::Readwrite)?;

        let store = tx.object_store(MISC_CF)?;
        let key = to_value(&key).map_err(CBwebDatabaseError::DeserializationError)?;
        let value = to_value(&value).map_err(CBwebDatabaseError::DeserializationError)?;
        store.put_key_val(&key, &value)?;
        tx.await.into_result()?;
        Ok(())
    }
}
