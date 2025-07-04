use aes_gcm::{Aes256Gcm, KeyInit, aead::{Aead, Payload}};

pub fn encrypt_data(data: &str, passkey: &str) -> anyhow::Result<Vec<u8>> {
    let key = derive_key(passkey)?;
    let cipher = Aes256Gcm::new_from_slice(&key)?;
    let nonce = rand::thread_rng().gen::<[u8; 12]>();
    Ok(cipher.encrypt(&nonce.into(), Payload::from(data.as_bytes()))?)
}