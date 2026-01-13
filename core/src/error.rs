//! AMP Core Error Types
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AMPError {
    #[error("Data fetch failed: {0}")]
    DataFetch(String),

    #[error("Correlation analysis failed: {0}")]
    CorrelationFailed(String),

    #[error("API error: {status} - {message}")]
    ApiError { status: u16, message: String },

    #[error("Notification delivery failed: {0}")]
    NotificationFailed(String),

    #[error("Geolocation error: {0}")]
    GeolocationFailed(String),

    #[error("Invalid coordinate: {0}")]
    InvalidCoordinate(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::error::Error),

    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Decimal error: {0}")]
    DecimalError(String),

    #[error("Time error: {0}")]
    TimeError(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, AMPError>;

#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub base_delay_ms: u64,
    pub max_delay_ms: u64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_delay_ms: 500,
            max_delay_ms: 30000,
        }
    }
}

pub async fn retry_async<F, Fut, T>(f: F, config: &RetryConfig) -> Result<T>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T>>,
{
    let mut attempt = 0;
    loop {
        match f().await {
            Ok(value) => return Ok(value),
            Err(e) if attempt < config.max_attempts - 1 => {
                attempt += 1;
                let delay_ms = (config.base_delay_ms * 2_u64.pow(attempt - 1))
                    .min(config.max_delay_ms);
                tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
            }
            Err(e) => return Err(e),
        }
    }
}
