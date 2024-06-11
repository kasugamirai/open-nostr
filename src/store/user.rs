use nostr_sdk::key::PublicKey;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::account::EncryptedSK;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub inner: AccountType,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AccountType {
    NotLoggedIn(NoLogin),
    Pub(OnlyPubkey),
    SecretKey(PinProtectedPrivkey),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NoLogin {
    pub r#type: String,
}

impl NoLogin {
    pub fn empty() -> Self {
        Self {
            r#type: "NoLogin".to_string(),
        }
    }
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct OnlyPubkey {
    pub r#type: String,
    pub pk: PublicKey,
}

impl OnlyPubkey {
    pub fn new(pk: PublicKey) -> Self {
        Self {
            r#type: "Pub".to_string(),
            pk,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PinProtectedPrivkey {
    pub r#type: String,
    pub encrypted_sk: EncryptedSK,
}

impl PinProtectedPrivkey {
    pub fn new(encrypted_sk: EncryptedSK) -> Self {
        Self {
            r#type: "SecretKey".to_string(),
            encrypted_sk,
        }
    }
}

impl Serialize for AccountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            AccountType::NotLoggedIn(nl) => nl.serialize(serializer),
            AccountType::Pub(pk) => pk.serialize(serializer),
            AccountType::SecretKey(pin_key) => pin_key.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for AccountType {
    fn deserialize<D>(deserializer: D) -> Result<AccountType, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        match value.get("type").and_then(Value::as_str) {
            Some("NoLogin") => Ok(AccountType::NotLoggedIn(NoLogin::empty())),
            Some("Pub") => Ok(AccountType::Pub(OnlyPubkey {
                r#type: "Pub".to_string(),
                pk: PublicKey::deserialize(value).unwrap(),
            })),
            Some("SecretKey") => Ok(AccountType::SecretKey(PinProtectedPrivkey {
                r#type: "SecretKey".to_string(),
                encrypted_sk: EncryptedSK::deserialize(value).unwrap(),
            })),
            _ => Err(serde::de::Error::custom("Invalid value")),
        }
    }
}
