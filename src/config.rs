// src/config.rs

#[derive(Debug, Clone)]
pub struct Config {
    pub address: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            address: std::env::var("APP_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string()),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
