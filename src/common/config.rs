use std::env;
use dotenv::dotenv;
use log::{info, warn};

pub fn init_env() {
    if dotenv().is_err() {
        warn!("Failed to load .env file. Proceeding with system environment variables.");
    } else {
        info!(".env file loaded successfully.");
    }
}

pub fn get_config(key: &str, default_value: &str) -> String {
    init_env();
    info!("Get config for {}", key);
    env::var(key).unwrap_or_else(|err| {
        warn!("Failed to read environment variable '{}': {}. Using default value '{}'.", key, err, default_value);
        default_value.to_string()
    })
}
