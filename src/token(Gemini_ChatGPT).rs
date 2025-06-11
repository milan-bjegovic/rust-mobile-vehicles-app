use std::path::PathBuf;
use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce};
use rand::rngs::OsRng;
//use rand_core::RngCore;
use rand::TryRngCore;
use std::fs;
use base64::{Engine as _, engine::general_purpose};


const KEY_PATH: &str = "/data/data/com.example.blogapp/enc_key";

fn get_storage_path() -> PathBuf {
    let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("session_token.enc");
    path
}

fn generate_or_load_key() -> [u8; 32] {
    if let Ok(key) = std::fs::read(KEY_PATH) {
        return key.try_into().expect("Invalid key size");
    }

    let mut key = [0u8; 32];
    let mut rng = OsRng;
    rng.try_fill_bytes(&mut key); // Ensure the RngCore trait is in scope
    std::fs::write(KEY_PATH, &key).expect("Failed to save encryption key");

    key
}

fn encrypt_and_save_token(token: &str) -> std::io::Result<()> {
    let key_bytes = generate_or_load_key();
    let key = Key::<aes_gcm::Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    let mut nonce_bytes = [0u8; 12];
    let mut rng = OsRng; // Create an instance
    let _ = rng.try_fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, token.as_bytes()).expect("Encryption failed");

    let encrypted_data = format!(
        "{}:{}",
        general_purpose::STANDARD.encode(nonce_bytes),
        general_purpose::STANDARD.encode(ciphertext)
    );
    fs::write(get_storage_path(), encrypted_data)?;

    Ok(())
}

fn load_and_decrypt_token() -> std::io::Result<Option<String>> {
    let path = get_storage_path();
    if !path.exists() {
        return Ok(None);
    }

    let data = fs::read_to_string(path)?;
    let parts: Vec<&str> = data.split(':').collect();
    if parts.len() != 2 {
        return Ok(None);
    }

    let nonce_bytes = general_purpose::STANDARD.decode(parts[0]).expect("Invalid nonce");
    let ciphertext = general_purpose::STANDARD.decode(parts[1]).expect("Invalid ciphertext");

    let key_bytes = generate_or_load_key();
    let key = Key::<aes_gcm::Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(&nonce_bytes);

    match cipher.decrypt(nonce, ciphertext.as_ref()) {
        Ok(decrypted) => Ok(Some(String::from_utf8(decrypted).expect("Invalid UTF-8"))),
        Err(_) => Ok(None),
    }
}

fn delete_token() -> std::io::Result<()> {
    let path = get_storage_path();
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}