use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use crate::{Error, Result};
use aes_gcm::aead::Aead;
use aes_gcm::KeyInit;
use aes_gcm::{Aes256Gcm, Key, Nonce};
use base64::{engine::general_purpose, Engine};
use pbkdf2::pbkdf2_hmac;
use sha2::{Digest, Sha256};

pub type CredentialStore = HashMap<String, Vec<Credential>>;

const PBKDF2_ITERATIONS: u32 = 100_000; // Adjust for desired security vs speed
const KEY_LEN: usize = 32; // 256-bit AES key length

pub fn hash_sha256(plaintext: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(plaintext.as_bytes()); // safer than into_bytes()
    let result = hasher.finalize();
    hex::encode(result)
}

// Derive encryption key from master password + salt
fn pbkdf2(master_password: &str, salt: &[u8]) -> [u8; KEY_LEN] {
    let mut key = [0u8; KEY_LEN];
    pbkdf2_hmac::<Sha256>(
        master_password.as_bytes(),
        salt,
        PBKDF2_ITERATIONS,
        &mut key,
    );
    key
}

// Encrypt data with key, returns (nonce, ciphertext)
fn aes256_gcm_encrypt(key_bytes: &[u8; KEY_LEN], plaintext: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {
    let key = Key::<Aes256Gcm>::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(key);

    // Generate random nonce (12 bytes)
    let mut nonce_bytes = [0u8; 12];
    rand::fill(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, plaintext)?;

    Ok((nonce_bytes.to_vec(), ciphertext))
}

// Decrypt ciphertext with key and nonce
fn aes256_gcm_decrypt(key_bytes: &[u8], nonce_bytes: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>> {
    let key = Key::<Aes256Gcm>::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(key);

    let nonce = Nonce::from_slice(nonce_bytes);

    let plaintext = cipher.decrypt(nonce, ciphertext)?;

    Ok(plaintext)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credential {
    pub username: String,
    pub ciphertext: String,
    pub salt: String,
    pub nonce: String,
}

impl Credential {
    pub fn new(username: &str, password: &str, master: &str) -> Result<Credential> {
        // Generate random salt
        let mut salt = [0u8; 16];
        rand::fill(&mut salt);

        let key = pbkdf2(master, &salt);
        let (nonce, ciphertext) = aes256_gcm_encrypt(&key, password.as_bytes())?;

        Ok(Credential {
            username: hash_sha256(username),
            ciphertext: general_purpose::STANDARD.encode(ciphertext),
            salt: general_purpose::STANDARD.encode(salt),
            nonce: general_purpose::STANDARD.encode(nonce),
        })
    }

    pub fn password_string(&self, master: &str) -> Result<String> {
        let salt = general_purpose::STANDARD.decode(&self.salt)?;
        let nonce = general_purpose::STANDARD.decode(&self.nonce)?;
        let ciphertext = general_purpose::STANDARD.decode(&self.ciphertext)?;

        // Derive key again from master password and salt
        let key = pbkdf2(master, &salt);

        let decrypted_bytes = aes256_gcm_decrypt(&key, &nonce, &ciphertext)?;

        let decrypted_data = String::from_utf8(decrypted_bytes)?;
        Ok(decrypted_data)
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_encryption() -> anyhow::Result<()> {
        let master_password = "supersecret";

        // Generate random salt
        let mut salt = [0u8; 16];
        rand::fill(&mut salt);

        let key = pbkdf2(master_password, &salt);

        // Data to encrypt: username and password concatenated or serialized
        let password = "password123";
        println!("Original data: {}", password);

        // Encrypt
        let (nonce, ciphertext) = aes256_gcm_encrypt(&key, password.as_bytes())?;

        // decrypt

        // Derive key again from master password and salt
        let key2 = pbkdf2(master_password, &salt);

        let decrypted_bytes = aes256_gcm_decrypt(&key2, &nonce, &ciphertext)?;
        let decrypted_data = String::from_utf8(decrypted_bytes)?;

        println!("Decrypted data: {}", decrypted_data);

        Ok(())
    }
}
