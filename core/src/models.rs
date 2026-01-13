use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct GpsCoordinate {
    pub latitude: Decimal,
    pub longitude: Decimal,
}

impl GpsCoordinate {
    pub fn new(lat: &str, lon: &str) -> Result<Self, String> {
        let latitude = Decimal::from_str(lat)
            .map_err(|_| format!("Invalid latitude: {}", lat))?;
        let longitude = Decimal::from_str(lon)
            .map_err(|_| format!("Invalid longitude: {}", lon))?;

        if latitude < Decimal::from(-90) || latitude > Decimal::from(90) {
            return Err(format!("Latitude out of range: {}", latitude));
        }
        if longitude < Decimal::from(-180) || longitude > Decimal::from(180) {
            return Err(format!("Longitude out of range: {}", longitude));
        }

        Ok(Self { latitude, longitude })
    }

    pub fn is_in_malmo(&self) -> bool {
        let lat = self.latitude;
        let lon = self.longitude;
        lat >= Decimal::from_str_exact("55.55").unwrap() &&
        lat <= Decimal::from_str_exact("55.65").unwrap() &&
        lon >= Decimal::from_str_exact("12.90").unwrap() &&
        lon <= Decimal::from_str_exact("13.10").unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleaningEvent {
    pub address: String,
    pub coordinate: GpsCoordinate,
    pub timestamp: DateTime<Utc>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleaningSchedule {
    pub address: String,
    pub coordinate: GpsCoordinate,
    pub next_cleaning: DateTime<Utc>,
    pub frequency_hours: f64,
    pub confidence: f64,
    pub day_of_week: String,
    pub time_of_day: String,
    pub last_cleaning: DateTime<Utc>,
    pub sample_size: usize,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AlertLevel {
    Info,
    Warning,
    Urgent,
    Active,
}

impl AlertLevel {
    pub fn from_hours_until(hours: i64) -> Self {
        match hours {
            ..=0 => AlertLevel::Active,
            1..=6 => AlertLevel::Urgent,
            7..=24 => AlertLevel::Warning,
            _ => AlertLevel::Info,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: DateTime<Utc>,
    pub last_update: Option<DateTime<Utc>>,
    pub data_points: usize,
    pub version: String,
}
