use std::env;

#[derive(Clone)]
pub struct Config {
    pub host: String,
    pub port: String,
    pub data_dir: String,
}

impl Config {
    // The `from_env()` function reads environment variables to create a new `Config` instance.
    // If an environment variable is not found, it falls back to a default value.
    pub fn from_env() -> Self {
        Self {
            // Reads the "HOST" environment variable or defaults to "127.0.0.1" if not found.
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            // Reads the "PORT" environment variable or defaults to "8080" if not found.
            port: env::var("PORT").unwrap_or_else(|_| "8080".to_string()),
            // Reads the "DATA_DIR" environment variable or defaults to "data" if not found.
            data_dir: env::var("DATA_DIR").unwrap_or_else(|_| "data".to_string()),
        }
    }
}
