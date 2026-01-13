/// Core data models for AMP system
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

/// High-precision GPS coordinate
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GpsCoordinate {
    /// Latitude with 8+ decimal precision (±0.1m)
    pub latitude: Decimal,
    /// Longitude with 8+ decimal precision (±0.1m)
    pub longitude: Decimal,
}

impl GpsCoordinate {
    /// Create new coordinate with validation
    pub fn new(lat: &str, lon: &str) -> Result<Self, String> {
        let latitude = Decimal::from_str_exact(lat)
            .map_err(|_| format!("Invalid latitude: {}", lat))?;
        let longitude = Decimal::from_str_exact(lon)
            .map_err(|_| format!("Invalid longitude: {}", lon))?;

        // Validate ranges
        if latitude < Decimal::from(-90) || latitude > Decimal::from(90) {
            return Err(format!("Latitude out of range: {}", latitude));
        }
        if longitude < Decimal::from(-180) || longitude > Decimal::from(180) {
            return Err(format!("Longitude out of range: {}", longitude));
        }

        Ok(Self { latitude, longitude })
    }

    /// Check if within Malmö city bounds
    pub fn is_in_malmo(&self) -> bool {
        let lat = self.latitude;
        let lon = self.longitude;

        lat >= Decimal::from_str_exact("55.55").unwrap() &&
            lat <= Decimal::from_str_exact("55.65").unwrap() &&
            lon >= Decimal::from_str_exact("12.90").unwrap() &&
            lon <= Decimal::from_str_exact("13.10").unwrap()
    }

    /// Calculate distance to another point (Haversine formula)
    pub fn distance_to(&self, other: &GpsCoordinate) -> f64 {
        use std::f64::consts::PI;

        let lat1_rad = self.latitude.to_f64().unwrap() * PI / 180.0;
        let lat2_rad = other.latitude.to_f64().unwrap() * PI / 180.0;
        let delta_lat = (other.latitude.to_f64().unwrap() - self.latitude.to_f64().unwrap()) * PI / 180.0;
        let delta_lon = (other.longitude.to_f64().unwrap() - self.longitude.to_f64().unwrap()) * PI / 180.0;

        let a = (delta_lat / 2.0).sin().powi(2)
            + lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        6371000.0 * c // Earth radius in meters
    }
}

/// Cleaning event from source data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleaningEvent {
    pub address: String,
    pub coordinate: GpsCoordinate,
    pub timestamp: DateTime<Utc>,
    pub is_active: bool,
}

/// Analyzed cleaning schedule for an address
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleaningSchedule {
    pub address: String,
    pub coordinate: GpsCoordinate,
    pub next_cleaning: DateTime<Utc>,
    pub frequency_hours: f64,
    pub confidence: f64, // 0.0-1.0
    pub day_of_week: String,
    pub time_of_day: String,
    pub last_cleaning: DateTime<Utc>,
    pub sample_size: usize,
}

/// Alert notification levels
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AlertLevel {
    Info,           // >24 hours
    Warning,        // 6-24 hours
    Urgent,         // <6 hours
    Active,         // Cleaning happening now
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

/// Address check response
#[derive(Debug, Serialize, Deserialize)]
pub struct AddressCheckResponse {
    pub found: bool,
    pub address: String,
    pub next_cleaning: Option<DateTime<Utc>>,
    pub hours_until: Option<i64>,
    pub alert_level: Option<AlertLevel>,
    pub frequency: Option<String>,
    pub confidence: Option<f64>,
}

/// Health check response
#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: DateTime<Utc>,
    pub last_update: Option<DateTime<Utc>>,
    pub data_points: usize,
    pub version: String,
}

use std::str::FromStr;

impl FromStr for GpsCoordinate {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return Err("Expected format: 'lat,lon'".to_string());
        }
        Self::new(parts.trim(), parts.trim())
    }
}
