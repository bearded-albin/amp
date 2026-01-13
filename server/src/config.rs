/// Configuration management
use amp_core::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub bind_addr: String,
    pub port: u16,
    pub malmo_api_base: String,
    pub data_dir: String,
    pub cache_file: String,
    pub api_key: Option<String>,
    pub log_level: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        Ok(Self {
            bind_addr: std::env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "5000".to_string())
                .parse()
                .unwrap_or(5000),
            malmo_api_base: std::env::var("MALMO_API_BASE")
                .unwrap_or_else(|_| "https://opendata.malmö.se".to_string()),
            data_dir: std::env::var("DATA_DIR").unwrap_or_else(|_| "./data".to_string()),
            cache_file: std::env::var("CACHE_FILE")
                .unwrap_or_else(|_| "./data/schedule_cache.json".to_string()),
            api_key: std::env::var("API_KEY").ok(),
            log_level: std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
        })
    }
}
