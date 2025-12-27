// src/config.rs

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn new() -> Self {
        let host = std::env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let raw_port = std::env::var("APP_PORT").unwrap_or_else(|_| "8000".to_string());
        let port = raw_port
            .parse::<u16>()
            .expect("Unable to get an application port.");

        Self { host, port }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
