use thiserror::Error;

#[derive(Error, Debug)]
pub enum AmpError {
    #[error("Invalid GPS coordinates: {0}")]
    InvalidGps(String),

    #[error("Geolocation error: {0}")]
    GeolocationError(String),

    #[error("Address not in Malmö")]
    AddressNotInMalmo,

    #[error("Correlation error: {0}")]
    CorrelationError(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("State error: {0}")]
    StateError(String),

    #[error("JNI error: {0}")]
    JniError(String),

    #[error("Data error: {0}")]
    DataError(String),
}

pub type AmpResult<T> = Result<T, AmpError>;
