use aes_gcm::{Aes256Gcm, Key, KeyInit};
use aes_gcm::aead::{Aead, AeadCore, OsRng};
use aes_gcm::aead::rand_core::RngCore;
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;
use anyhow::Result;

pub fn derive_key(password: &str, salt: &[u8]) -> [u8; 32] {
    let mut key = [0u8; 32];
    pbkdf2_hmac::<Sha256>(password.as_bytes(), salt, 100_000, &mut key);
    key
}

pub fn encrypt_data(data: &[u8], password: &str) -> Result<Vec<u8>> {

    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);

    let key_bytes = derive_key(password, &salt);
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);
    
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher.encrypt(&nonce, data)
        .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;
    
    let mut result = Vec::new();
    result.extend_from_slice(&salt);
    result.extend_from_slice(&nonce);
    result.extend_from_slice(&ciphertext);
    
    Ok(result)
}

pub fn decrypt_data(encrypted_data: &[u8], password: &str) -> Result<Vec<u8>> {
    if encrypted_data.len() < 28 { 
        return Err(anyhow::anyhow!("Invalid encrypted data length"));
    }
    
    let salt = &encrypted_data[0..16];
    let nonce = &encrypted_data[16..28];
    let ciphertext = &encrypted_data[28..];
    
    let key_bytes = derive_key(password, salt);
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);
    
    let nonce = aes_gcm::Nonce::from_slice(nonce);
    let plaintext = cipher.decrypt(nonce, ciphertext)
        .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))?;
    
    Ok(plaintext)
}