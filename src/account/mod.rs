use std::str::FromStr;

use aes_gcm::aead::{generic_array::GenericArray, Aead, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Nonce};
use nostr_sdk::bitcoin::hashes::sha256::Hash as Sha256Hash;
use nostr_sdk::hashes::Hash;
use nostr_sdk::key::SecretKey;
use nostr_sdk::secp256k1::rand::Rng;
use serde::{Deserialize, Serialize};
use tracing::error;
use tracing::info;
use wasm_bindgen_test::console_log;

// Define a struct to hold the encrypted secret key and the hash of the pin
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct EncryptedSK {
    encrypted_sk: Vec<u8>,
    nonce: Vec<u8>,
}

impl EncryptedSK {
    // Encrypt the secret key with the provided pin
    pub fn new(sk: &SecretKey, pin: [u8; 4]) -> Self {
        let key = Sha256Hash::hash(&pin);
        let cipher = Aes256Gcm::new(GenericArray::from_slice(key.as_byte_array()));
        let nonce_array = OsRng.gen::<[u8; 12]>();
        let nonce = Nonce::from_slice(&nonce_array);

        let encrypted_sk = match cipher.encrypt(nonce, sk.display_secret().to_string().as_bytes()) {
            Ok(encrypted_data) => {
                info!("Encryption successful");
                encrypted_data
            }
            Err(e) => {
                error!("Encryption failure: {:?}", e);
                Vec::new()
            }
        };

        Self {
            encrypted_sk,
            nonce: nonce_array.to_vec(),
        }
    }

    // Method to decrypt the secret key using the pin
    pub fn decrypt(&self, pin: [u8; 4]) -> Option<SecretKey> {
        // Verify the pin
        let key = Sha256Hash::hash(&pin);
        let cipher = Aes256Gcm::new(GenericArray::from_slice(key.as_byte_array()));
        console_log!("Derived Key: {:?}", key.as_byte_array());

        console_log!("Nonce: {:?}", self.nonce);
        console_log!("Encrypted SK: {:?}", self.encrypted_sk);

        match cipher.decrypt(Nonce::from_slice(&self.nonce), &*self.encrypted_sk) {
            Ok(decrypted) => {
                console_log!("Decrypted Bytes: {:?}", decrypted);
                // Convert decrypted bytes back to string and then to SecretKey
                let decrypted_str = String::from_utf8(decrypted).ok()?;
                SecretKey::from_str(&decrypted_str).ok()
            }
            Err(err) => {
                console_log!("Decryption Error: {:?}", err);
                None
            }
        }
    }

    // Serialize the instance to a JSON string
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "".to_string())
    }

    // Deserialize from a JSON string to an instance of EncryptedSK
    pub fn from_json(json_str: &str) -> Self {
        serde_json::from_str(json_str).unwrap_or_else(|_| EncryptedSK {
            encrypted_sk: Vec::new(),
            nonce: Vec::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nostr_sdk::key::Keys;
    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_encryption_decryption() {
        let pin = [1, 2, 3, 4];
        let keys = Keys::generate();
        let secret_key = keys.secret_key().unwrap();

        let encrypted_sk = EncryptedSK::new(secret_key, pin);
        let decrypted_sk = encrypted_sk.decrypt(pin).expect("Failed to decrypt");

        assert_eq!(
            secret_key.display_secret().to_string(),
            decrypted_sk.display_secret().to_string()
        );
    }

    #[wasm_bindgen_test]
    fn test_serialization_deserialization() {
        let pin = [1, 2, 3, 4];
        let keys = Keys::generate();
        let secret_key = keys.secret_key().unwrap();

        let encrypted_sk = EncryptedSK::new(secret_key, pin);
        let json_str = encrypted_sk.to_json();
        let deserialized_encrypted_sk = EncryptedSK::from_json(&json_str);

        assert_eq!(encrypted_sk, deserialized_encrypted_sk);
    }

    #[wasm_bindgen_test]
    fn test_decryption_with_wrong_pin() {
        let pin = [1, 2, 3, 4];
        let wrong_pin = [4, 3, 2, 1];
        let keys = Keys::generate();
        let secret_key = keys.secret_key().unwrap();

        let encrypted_sk = EncryptedSK::new(secret_key, pin);
        let decrypted_sk = encrypted_sk.decrypt(wrong_pin);

        assert!(decrypted_sk.is_none());
    }
}
