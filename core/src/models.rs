use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use std::str::FromStr;
use rust_decimal::prelude::ToPrimitive;

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

    pub fn distance_to(&self, other: &GpsCoordinate) -> f64 {
        use std::f64::consts::PI;
        let lat1 = self.latitude.to_f64().unwrap_or(0.0) * PI / 180.0;
        let lat2 = other.latitude.to_f64().unwrap_or(0.0) * PI / 180.0;
        let delta_lat = (other.latitude.to_f64().unwrap_or(0.0) - self.latitude.to_f64().unwrap_or(0.0)) * PI / 180.0;
        let delta_lon = (other.longitude.to_f64().unwrap_or(0.0) - self.longitude.to_f64().unwrap_or(0.0)) * PI / 180.0;

        let a = (delta_lat / 2.0).sin().powi(2)
            + lat1.cos() * lat2.cos() * (delta_lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
        6371000.0 * c
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
#[derive(Clone)]
pub struct StoredAddress {
    pub address: String,
    pub coordinate: GpsCoordinate,
    pub added_at: DateTime<Utc>,
}
