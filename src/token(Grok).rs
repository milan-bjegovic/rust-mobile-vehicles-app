use std::path::PathBuf;
use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce};
use rand::RngCore;
use std::fs;
use base64::engine::general_purpose::STANDARD as Base64Engine;
use base64::Engine;

// Use a dynamic key path based on the platform
pub fn get_key_path() -> PathBuf {
    #[cfg(target_os = "android")]
    return PathBuf::from("/data/data/com.example.blogapp/enc_key");

    #[cfg(not(target_os = "android"))]
    {
        let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("com.example.blogapp");
        path.push("enc_key");
        path
    }
}

pub fn get_storage_path() -> PathBuf {
    let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("session_token.enc");
    path
}

fn generate_or_load_key() -> [u8; 32] {
    let key_path = get_key_path();
    if let Ok(key) = fs::read(&key_path) {
        return key.try_into().expect("Invalid key size");
    }

    let mut key = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut key);

    // Ensure the parent directory exists
    if let Some(parent) = key_path.parent() {
        fs::create_dir_all(parent).expect("Failed to create key directory");
    }
    fs::write(&key_path, &key).expect("Failed to save encryption key");

    key
}

pub fn encrypt_and_save_token(token: &str) -> std::io::Result<()> {
    let binding = generate_or_load_key();
    let key = Key::<Aes256Gcm>::from_slice(&binding);
    let cipher = Aes256Gcm::new(key);

    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, token.as_bytes()).expect("Encryption failed");

    let encrypted_data = format!(
        "{}:{}",
        Base64Engine.encode(nonce_bytes),
        Base64Engine.encode(&ciphertext)
    );

    // Ensure the storage directory exists
    let storage_path = get_storage_path();
    if let Some(parent) = storage_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(storage_path, encrypted_data)?;

    Ok(())
}

pub fn load_and_decrypt_token() -> std::io::Result<Option<String>> {
    let path = get_storage_path();
    if !path.exists() {
        return Ok(None);
    }

    let data = fs::read_to_string(path)?;
    let parts: Vec<&str> = data.split(':').collect();
    if parts.len() != 2 {
        return Ok(None);
    }

    let nonce_bytes = Base64Engine.decode(parts[0]).expect("Invalid nonce");
    let ciphertext = Base64Engine.decode(parts[1]).expect("Invalid ciphertext");

    let binding = generate_or_load_key();
    let key = Key::<Aes256Gcm>::from_slice(&binding);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(&nonce_bytes);

    match cipher.decrypt(nonce, ciphertext.as_ref()) {
        Ok(decrypted) => {
            let decrypted_str = String::from_utf8(decrypted).expect("Invalid UTF-8");
            // Extract the session code (UUID-like part after "Session code: ")
            if let Some(session_code) = decrypted_str.split("Session code: ").nth(1) {
                // Trim any trailing quotes or whitespace
                let session_code = session_code.trim_matches('"').trim();
                Ok(Some(session_code.to_string()))
            } else {
                Ok(Some(decrypted_str)) // Fallback to full string if parsing fails
            }
        }
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