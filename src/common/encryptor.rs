use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::str::from_utf8;
use rand::RngCore;
use rand::rngs::{ThreadRng};
use aes::Aes256;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};
use bcrypt::{hash, verify, DEFAULT_COST};

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

fn generate_new_key() -> Vec<u8> {
    let mut rng: ThreadRng = rand::thread_rng();
    let mut key: Vec<u8> = vec![0u8; 32];
    rng.fill_bytes(&mut key);

    if key.len() != 32 {
        panic!("Invalid key length. AES-256 requires a 32-byte key.");
    }

    key
}

fn get_secret_key() -> Vec<u8> {
    let file_path: &str = "./.secret/secret_key.txt";

    if let Some(parent) = Path::new(file_path).parent() {
        fs::create_dir_all(parent).unwrap();
    }

    if !Path::new(file_path).exists() {
        let mut file: File = File::create(file_path).unwrap();
        file.write_all(&generate_new_key()).unwrap();
    }

    let secret_content: Vec<u8> = fs::read(file_path).unwrap();

    secret_content
}

pub fn encrypt(original_string: &String) -> Vec<u8> {
    let key: Vec<u8> = get_secret_key();
    let iv = &key[..16];

    let cipher = Aes256Cbc::new_from_slices(&key, iv).unwrap();
    let encrypted = cipher.encrypt_vec(original_string.as_bytes());

    let mut result = iv.to_vec();
    result.extend(encrypted);
    result
}

pub fn decrypt(encrypted_bytes: &Vec<u8>) -> String {
    let key: Vec<u8> = get_secret_key();
    let iv = &encrypted_bytes[..16];
    let ciphertext = &encrypted_bytes[16..];

    let cipher = Aes256Cbc::new_from_slices(&key, iv).unwrap();
    let decrypted_bytes = cipher.decrypt_vec(ciphertext).unwrap();

    from_utf8(&decrypted_bytes).unwrap().to_string()
}

pub fn hash_password(password: &String) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

pub fn verify_password(password: &String, hashed_password: &String) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hashed_password)
}
