pub mod error;
pub mod subscription;
pub mod user;

pub use error::CBwebDatabaseError;
use indexed_db_futures::idb_object_store::IdbObjectStoreParameters;
use indexed_db_futures::request::{IdbOpenDbRequestLike, OpenDbRequest};
use indexed_db_futures::web_sys::IdbTransactionMode;
use indexed_db_futures::{IdbDatabase, IdbKeyPath, IdbQuerySource, IdbVersionChangeEvent};
use serde_wasm_bindgen::{from_value, to_value};
use std::future::IntoFuture;
use std::sync::Arc;
use subscription::{CustomSub, RelaySet, SubNames};
pub use user::{AccountType, User};
use wasm_bindgen::JsValue;
use web_sys::IdbIndexParameters;

const CURRENT_DB_VERSION: u32 = 2;
const RELAY_SET_CF: &str = "relay-set";
const SUB_NAME_LIST: &str = "sub-name-list";
const CUSTOM_SUB_CF: &str = "custom-sub";
const USER_CF: &str = "user";

#[derive(Clone)]
pub struct CBWebDatabase {
    db: Arc<IdbDatabase>,
}

impl CBWebDatabase {
    async fn migration(&mut self) -> Result<(), CBwebDatabaseError> {
        let name: String = self.db.name();
        let old_version: u32 = self.db.version() as u32;
        tracing::info!("start migration with version {}", old_version);

        let mut db_req: OpenDbRequest = IdbDatabase::open_u32(&name, CURRENT_DB_VERSION)?;
        db_req.set_on_upgrade_needed(Some(
            move |evt: &IdbVersionChangeEvent| -> Result<(), JsValue> {
                //apply migration 1->2
                if old_version == 1 {
                    {
                        //init user store
                        let mut create_store_params = IdbObjectStoreParameters::new();
                        let key_path = IdbKeyPath::str("name");
                        create_store_params.key_path(Some(&key_path));
                        let user_store = evt
                            .db()
                            .create_object_store_with_params(USER_CF, &create_store_params)
                            .unwrap();
                        user_store
                            .create_index_with_params(
                                "name",
                                &key_path,
                                IdbIndexParameters::new().unique(true),
                            )
                            .unwrap();

                        //insert default user
                        let value = to_value(&User {
                            name: "NOT_LOGGED_IN".to_string(),
                            inner: AccountType::NotLoggedIn,
                        })
                        .unwrap();
                        user_store.add_val(&value).unwrap();
                    }

                    {
                        //init relay-set store
                        let mut create_store_params = IdbObjectStoreParameters::new();
                        let key_path = IdbKeyPath::str("name");
                        create_store_params.key_path(Some(&key_path));
                        let relay_set_store = evt
                            .db()
                            .create_object_store_with_params(RELAY_SET_CF, &create_store_params)
                            .unwrap();
                        relay_set_store
                            .create_index_with_params(
                                "name",
                                &key_path,
                                IdbIndexParameters::new().unique(true),
                            )
                            .unwrap();
                    }

                    {
                        //init sub-name-list names store
                        let mut create_store_params = IdbObjectStoreParameters::new();
                        let key_path = IdbKeyPath::str("name");
                        create_store_params.key_path(Some(&key_path));
                        let relay_set_store = evt
                            .db()
                            .create_object_store_with_params(SUB_NAME_LIST, &create_store_params)
                            .unwrap();
                        relay_set_store
                            .create_index_with_params(
                                "name",
                                &key_path,
                                IdbIndexParameters::new().unique(true),
                            )
                            .unwrap();
                    }

                    {
                        //init custom-sub store
                        let mut create_store_params = IdbObjectStoreParameters::new();
                        let key_path = IdbKeyPath::str("name");
                        create_store_params.key_path(Some(&key_path));
                        let custom_sub_store = evt
                            .db()
                            .create_object_store_with_params(CUSTOM_SUB_CF, &create_store_params)
                            .unwrap();
                        custom_sub_store
                            .create_index_with_params(
                                "name",
                                &key_path,
                                IdbIndexParameters::new().unique(true),
                            )
                            .unwrap();
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

        self.db = Arc::new(db_req.into_future().await?);

        Ok(())
    }

    pub async fn open<S>(name: S) -> Result<Self, CBwebDatabaseError>
    where
        S: AsRef<str>,
    {
        let mut this = Self {
            db: Arc::new(IdbDatabase::open(name.as_ref())?.into_future().await?),
        };

        this.migration().await?;
        Ok(this)
    }

    pub async fn save_relay_set(&self, relay_set: RelaySet) -> Result<(), CBwebDatabaseError> {
        let tx = self
            .db
            .transaction_on_one_with_mode(RELAY_SET_CF, IdbTransactionMode::Readwrite)?;

        let store = tx.object_store(RELAY_SET_CF)?;
        store.delete(
            &to_value(&relay_set.name.clone()).map_err(CBwebDatabaseError::DeserializationError)?,
        )?;
        let value = to_value(&relay_set).map_err(CBwebDatabaseError::DeserializationError)?;
        store.add_val(&value)?;

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

    pub async fn save_sub_name_list(&self, names: SubNames) -> Result<(), CBwebDatabaseError> {
        let tx = self
            .db
            .transaction_on_one_with_mode(SUB_NAME_LIST, IdbTransactionMode::Readwrite)?;

        let store = tx.object_store(SUB_NAME_LIST)?;
        store.delete(
            &to_value(&names.name.clone()).map_err(CBwebDatabaseError::DeserializationError)?,
        )?;
        let value = to_value(&names).map_err(CBwebDatabaseError::DeserializationError)?;
        store.add_val(&value)?;

        tx.await.into_result()?;
        Ok(())
    }

    pub async fn get_sub_name_list(&self) -> Result<SubNames, CBwebDatabaseError> {
        let tx = self
            .db
            .transaction_on_one_with_mode(SUB_NAME_LIST, IdbTransactionMode::Readonly)?;

        let store = tx.object_store(SUB_NAME_LIST)?;
        let key = JsValue::from_str("SUBSCRIPTION_LIST");
        match store.get(&key)?.await? {
            Some(value) => match from_value::<SubNames>(value.clone()) {
                Ok(name_list) => Ok(name_list),
                Err(e) => {
                    tracing::error!("Error deserializing Vec<String>: {:?}", e);
                    Err(CBwebDatabaseError::DeserializationError(e))
                }
            },
            None => Err(CBwebDatabaseError::NotFound),
        }
    }

    pub async fn save_custom_sub(&self, custom_sub: CustomSub) -> Result<(), CBwebDatabaseError> {
        let tx = self
            .db
            .transaction_on_one_with_mode(CUSTOM_SUB_CF, IdbTransactionMode::Readwrite)?;

        let store = tx.object_store(CUSTOM_SUB_CF)?;
        store.delete(
            &to_value(&custom_sub.name.clone())
                .map_err(CBwebDatabaseError::DeserializationError)?,
        )?;
        let value = to_value(&custom_sub).map_err(CBwebDatabaseError::DeserializationError)?;
        store.add_val(&value)?;

        tx.await.into_result()?;
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

        loop {
            let v = value.shift();
            if !v.is_object(){
                break;
            }
            match from_value::<CustomSub>(v) {
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
        store.delete(
            &to_value(&user.name.clone()).map_err(CBwebDatabaseError::DeserializationError)?,
        )?;
        let value = to_value(&user).map_err(CBwebDatabaseError::DeserializationError)?;
        store.add_val(&value)?;

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
}
