use aes::Aes256;
use block_modes::BlockMode;
use block_modes::Cbc;
use block_padding::Pkcs7;
use rand::Rng;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

const KEY_SIZE: usize = 32;  // AES256 requires a 32-byte key
const IV_SIZE: usize = 16;   // AES CBC block size

// Generate a random key and IV
fn generate_key_iv() -> (Vec<u8>, Vec<u8>) {
    let mut rng = rand::thread_rng();  // You can still use this for random number generation.
    let key: Vec<u8> = (0..KEY_SIZE).map(|_| rng.gen()).collect();
    let iv: Vec<u8> = (0..IV_SIZE).map(|_| rng.gen()).collect();
    (key, iv)
}

// Encrypt the session token
fn encrypt_token(token: &str, key: &[u8], iv: &[u8]) -> Vec<u8> {
    let cipher = Aes256Cbc::new_from_slices(key, iv).unwrap();
    let encrypted_token = cipher.encrypt_vec(token.as_bytes());
    encrypted_token
}

// Decrypt the session token
fn decrypt_token(encrypted_token: &[u8], key: &[u8], iv: &[u8]) -> String {
    let cipher = Aes256Cbc::new_from_slices(key, iv).unwrap();
    let decrypted_token = cipher.decrypt_vec(encrypted_token).unwrap();
    String::from_utf8(decrypted_token).unwrap()
}


// Save the new token securely to the file
pub fn update_token_in_file(token: &str) {
    let file_path = "/data/data/com.example.VehiclesApp/session_token.enc"; 
    let (key, iv) = generate_key_iv();
    let encrypted_token = encrypt_token(token, &key, &iv);

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true) // Overwrite the existing file content
        .open(file_path)
        .unwrap();

    // Write the new key, IV, and encrypted token to the file
    file.write_all(&key).unwrap();         // Write the key
    file.write_all(&iv).unwrap();          // Write the IV
    file.write_all(&encrypted_token).unwrap(); // Write the encrypted token
}

// Load and decrypt the token from the file
pub fn load_token_from_file() -> Option<String> {
    let file_path = "/data/data/com.example.VehiclesApp/session_token.enc"; 
    let mut file = File::open(file_path).ok()?;
    let mut key = vec![0; KEY_SIZE];
    let mut iv = vec![0; IV_SIZE];
    
    file.read_exact(&mut key).unwrap();
    file.read_exact(&mut iv).unwrap();
    
    let mut encrypted_token = Vec::new();
    file.read_to_end(&mut encrypted_token).unwrap();

    // Some(decrypt_token(&encrypted_token, &key, &iv))

    match decrypt_token(&encrypted_token, &key, &iv).into() {
        Some(decrypted_str) => {
            // Extract session code (UUID-like part after "Session code: ")
            decrypted_str.split("Session code: ").nth(1)
                .map(|session_code| session_code.trim_matches('"').trim().to_string())
                .or(Some(decrypted_str)) // Fallback if "Session code: " is not found
        }
        None => None, // Return None if decryption fails
    }    
}

pub fn create_empty_token_file() {

    let file_path = "/data/data/com.example.VehiclesApp/session_token.enc"; 

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_path)
        .unwrap();

    let empty_data: Vec<u8> = vec![0; 32]; // Example: 32-byte empty data
    file.write_all(&empty_data).unwrap(); // Write key placeholder (32 bytes for AES256 key)
    file.write_all(&empty_data).unwrap(); // Write IV placeholder (16 bytes for AES block size)
    file.write_all(&empty_data).unwrap(); // Write encrypted token placeholder (empty data)
}
