use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, KeyInit, Nonce}; // Or another encryption library
use nostr_sdk::bitcoin::hashes::sha256::Hash as Sha256Hash;
use nostr_sdk::hashes::Hash;
use nostr_sdk::key::SecretKey;
use serde::{Deserialize, Serialize};

// Define a struct to hold the encrypted secret key and the hash of the pin
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct EncryptedSK {
    encrypted_sk: Vec<u8>,
    nonce: Vec<u8>,
}

impl EncryptedSK {
    // Encrypt the secret key with the provided pin
    pub fn new(sk: &SecretKey, pin: [u8; 4]) -> Self {
        // Derive a key from the pin
        let key = Sha256Hash::hash(&pin);
        let cipher = Aes256Gcm::new_from_slice(key.as_byte_array());
        let nonce = Nonce::from_slice(b"unique nonce"); // This should be unique per encryption
        let encrypted_sk = cipher
            .unwrap()
            .encrypt(nonce, sk.to_string().as_bytes())
            .expect("encryption failure!");

        Self {
            encrypted_sk,
            nonce: nonce.to_vec(),
        }
    }

    // Method to decrypt the secret key using the pin
    pub fn decrypt(&self, pin: [u8; 4]) -> Option<SecretKey> {
        // Verify the pin
        let key = Sha256Hash::hash(&pin);
        let cipher = Aes256Gcm::new_from_slice(key.as_byte_array());
        match cipher
            .unwrap()
            .decrypt(Nonce::from_slice(&self.nonce), &*self.encrypted_sk)
        {
            Ok(decrypted) => match SecretKey::from_slice(&decrypted) {
                Ok(sk) => Some(sk),
                Err(_) => None,
            },
            Err(_) => None,
        }
    }

    // Serialize the instance to a JSON string
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize EncryptedSK")
    }

    // Deserialize from a JSON string to an instance of EncryptedSK
    pub fn from_json(json_str: &str) -> Self {
        serde_json::from_str(json_str).expect("Failed to deserialize EncryptedSK")
    }
}
