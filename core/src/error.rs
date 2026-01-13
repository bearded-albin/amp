use thiserror::Error;

#[derive(Error, Debug)]
pub enum AMPError {
    #[error("Correlation analysis failed: {0}")]
    CorrelationFailed(String),

    #[error("Geolocation error: {0}")]
    GeolocationFailed(String),

    #[error("Invalid coordinate: {0}")]
    InvalidCoordinate(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::error::Error),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, AMPError>;
