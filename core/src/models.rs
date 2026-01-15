use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// High-precision GPS coordinate (8+ decimal places = ±0.1m accuracy)
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct GpsCoordinate {
    pub latitude: Decimal,
    pub longitude: Decimal,
}

impl GpsCoordinate {
    pub fn new(lat: f64, lon: f64) -> Self {
        Self {
            latitude: Decimal::from_str(&format!("{:.8}", lat))
                .unwrap_or_else(|_| Decimal::from_f64_retain(lat).unwrap_or(Decimal::ZERO)),
            longitude: Decimal::from_str(&format!("{:.8}", lon))
                .unwrap_or_else(|_| Decimal::from_f64_retain(lon).unwrap_or(Decimal::ZERO)),
        }
    }

    /// Check if coordinates are within Malmö city bounds
    pub fn is_malmo_coords(&self) -> bool {
        let lat = self.latitude.to_f64().unwrap_or(0.0);
        let lon = self.longitude.to_f64().unwrap_or(0.0);

        // Malmö city bounds (approximately)
        lat >= 55.55 && lat <= 55.65 && lon >= 12.95 && lon <= 13.05
    }

    pub fn distance_to(&self, other: &GpsCoordinate) -> Decimal {
        let lat1 = self.latitude.to_f64().unwrap_or(0.0);
        let lon1 = self.longitude.to_f64().unwrap_or(0.0);
        let lat2 = other.latitude.to_f64().unwrap_or(0.0);
        let lon2 = other.longitude.to_f64().unwrap_or(0.0);

        let dx = lat2 - lat1;
        let dy = lon2 - lon1;
        let dist = (dx * dx + dy * dy).sqrt();

        Decimal::from_f64_retain(dist).unwrap_or(Decimal::ZERO)
    }
}

/// Address stored by user
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Address {
    pub id: String,
    pub name: String,
    pub street: String,
    pub coordinates: Option<String>, // JSON serialized (lat, lon) to avoid float issues
    pub active: bool,
}

impl Address {
    pub fn get_coordinates(&self) -> Option<(f64, f64)> {
        self.coordinates.as_ref().and_then(|coords_str| {
            if let Ok(val) = serde_json::from_str::<[f64; 2]>(coords_str) {
                Some((val[0], val[1]))
            } else {
                None
            }
        })
    }

    pub fn set_coordinates(&mut self, lat: f64, lon: f64) {
        self.coordinates = serde_json::to_string(&[lat, lon]).ok();
    }
}

/// Malmö street cleaning schedule
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CleaningSchedule {
    pub street_name: String,
    pub next_cleaning: String, // ISO 8601 date
    pub days_until: i32,
    pub area_code: String,
}

/// Alert severity levels
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum AlertLevel {
    Cleaning,     // Cleaning happening now
    SixHours,     // 6 hours until cleaning
    TwentyFours,  // 24 hours until cleaning
    None,         // No upcoming cleaning
}

impl AlertLevel {
    pub fn priority(&self) -> u32 {
        match self {
            AlertLevel::Cleaning => 3,
            AlertLevel::SixHours => 2,
            AlertLevel::TwentyFours => 1,
            AlertLevel::None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_malmo_bounds() {
        let malmo = GpsCoordinate::new(55.6050, 13.0038);
        assert!(malmo.is_malmo_coords());

        let stockholm = GpsCoordinate::new(59.3293, 18.0686);
        assert!(!stockholm.is_malmo_coords());
    }

    #[test]
    fn test_high_precision() {
        let coord = GpsCoordinate::new(55.60501234, 13.00381234);
        assert_eq!(
            coord.latitude.to_string(),
            "55.60501234"
        );
    }
}
