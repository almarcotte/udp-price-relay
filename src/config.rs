use std::env;

pub struct Config {
    pub server_addr: String,
    pub listen_addr: String,
}

pub fn load() -> Config {
    Config {
        server_addr: env::var("SERVER_ADDR").unwrap_or_else(|_| "0.0.0.0:4000".to_string()),
        listen_addr: env::var("LISTEN_ADDR").unwrap_or_else(|_| "0.0.0.0:5000".to_string()),
    }
}