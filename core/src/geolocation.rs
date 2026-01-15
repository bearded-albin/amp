use crate::models::{GpsCoordinate, Address};
use crate::{AmpError, AmpResult};
use std::collections::HashMap;

/// Reverse geocoding and address management
pub struct Geocoder {
    address_cache: HashMap<(String, String), Address>,
}

impl Geocoder {
    pub fn new() -> Self {
        Self {
            address_cache: HashMap::new(),
        }
    }

    /// Validate address is in Malmö (simplified - no API calls)
    pub fn validate_malmo_address(&self, address: &str) -> AmpResult<()> {
        // Check if address ends with common Malmö postal codes
        let malmo_indicators = vec![
            "Malmö",
            "20", // 200xx postal codes
            "21", // 210xx postal codes
            "214", // etc
        ];

        if malmo_indicators.iter().any(|ind| address.contains(ind)) {
            Ok(())
        } else {
            Err(AmpError::AddressNotInMalmo)
        }
    }

    /// Mock GPS from address (would use real geocoding service)
    pub fn estimate_coordinates(&self, address: &str) -> AmpResult<GpsCoordinate> {
        // Placeholder: In production, this would call a geocoding API
        // For now, return Malmö city center if address is in Malmö
        if self.validate_malmo_address(address).is_ok() {
            Ok(GpsCoordinate::new(55.6050, 13.0038))
        } else {
            Err(AmpError::AddressNotInMalmo)
        }
    }

    /// Convert GPS to approximate address (mock)
    pub fn reverse_geocode(&self, coord: &GpsCoordinate) -> AmpResult<String> {
        if coord.is_malmo_coords() {
            Ok("Malmö, Sweden".to_string())
        } else {
            Err(AmpError::GeolocationError(
                "Coordinates outside Malmö".to_string(),
            ))
        }
    }

    pub fn clear_cache(&mut self) {
        self.address_cache.clear();
    }
}

impl Default for Geocoder {
    fn default() -> Self {
        Self::new()
    }
}
