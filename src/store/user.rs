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
        // Deserialize into a serde_json::Value to allow inspection of the "type" field
        let value = Value::deserialize(deserializer)?;

        // Match on the "type" field to determine which variant to use for deserialization
        match value.get("type").and_then(Value::as_str) {
            Some("NoLogin") => {
                // Deserialize the entire value into a NoLogin variant
                let no_login = NoLogin::deserialize(value).map_err(serde::de::Error::custom)?;
                Ok(AccountType::NotLoggedIn(no_login))
            }
            Some("Pub") => {
                // Deserialize the entire value into a OnlyPubkey variant
                let only_pubkey =
                    OnlyPubkey::deserialize(value).map_err(serde::de::Error::custom)?;
                Ok(AccountType::Pub(only_pubkey))
            }
            Some("SecretKey") => {
                // Deserialize the entire value into a PinProtectedPrivkey variant
                let pin_protected_privkey =
                    PinProtectedPrivkey::deserialize(value).map_err(serde::de::Error::custom)?;
                Ok(AccountType::SecretKey(pin_protected_privkey))
            }
            _ => {
                // Return an error if the "type" field does not match any known variant
                Err(serde::de::Error::custom("Unknown account type"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nostr_sdk::key::Keys;
    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_account_type_deserialization() {
        let no_login_json = serde_json::json!({
            "type": "NoLogin"
        });
        let no_login: AccountType = serde_json::from_value(no_login_json).unwrap();
        assert_eq!(no_login, AccountType::NotLoggedIn(NoLogin::empty()));

        let public_key =
            String::from("npub1q0uulk2ga9dwkp8hsquzx38hc88uqggdntelgqrtkm29r3ass6fq8y9py9");
        let only_pub_json = serde_json::json!({
            "type": "Pub",
            "pk": public_key
        });
        let only_pub: AccountType = serde_json::from_value(only_pub_json).unwrap();
        assert_eq!(
            only_pub,
            AccountType::Pub(OnlyPubkey::new(
                <PublicKey as std::str::FromStr>::from_str(&public_key).unwrap()
            ))
        );

        let encrypted_sk = EncryptedSK::new(Keys::generate().secret_key().unwrap(), [1, 2, 3, 4]);
        let secret_key_json = serde_json::json!({
            "type": "SecretKey",
            "encrypted_sk": encrypted_sk
        });
        let secret_key: AccountType = serde_json::from_value(secret_key_json).unwrap();
        assert_eq!(
            secret_key,
            AccountType::SecretKey(PinProtectedPrivkey::new(encrypted_sk))
        );
    }
}
