/// Reverse geocoding: GPS coordinates to address
use crate::error::Result;
use crate::models::GpsCoordinate;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, error};

#[derive(Deserialize)]
struct NominatimResponse {
    address: HashMap<String, String>,
    #[serde(rename = "display_name")]
    display_name: String,
}

/// Reverse geocoding service using Nominatim
pub struct GeolocationService {
    client: Client,
    cache: std::sync::Arc<parking_lot::RwLock<HashMap<String, String>>>,
}

impl GeolocationService {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            cache: std::sync::Arc::new(parking_lot::RwLock::new(HashMap::new())),
        }
    }

    /// Convert GPS coordinates to address
    pub async fn get_address_from_gps(
        &self,
        coord: &GpsCoordinate,
        language: &str,
    ) -> Result<String> {
        let cache_key = format!("{:.8},{:.8}", coord.latitude, coord.longitude);

        // Check cache
        {
            let cache = self.cache.read();
            if let Some(cached) = cache.get(&cache_key) {
                return Ok(cached.clone());
            }
        }

        // Query Nominatim
        let url = format!(
            "https://nominatim.openstreetmap.org/reverse?format=json&lat={}&lon={}&language={}",
            coord.latitude, coord.longitude, language
        );

        let response = self
            .client
            .get(&url)
            .header("User-Agent", "AMP-App/1.0")
            .send()
            .await?;

        let nominatim_resp: NominatimResponse = response.json().await?;

        // Build address from components
        let mut address_parts = Vec::new();

        if let Some(num) = nominatim_resp.address.get("house_number") {
            address_parts.push(num.clone());
        }
        if let Some(road) = nominatim_resp.address.get("road") {
            address_parts.push(road.clone());
        }
        if let Some(postal) = nominatim_resp.address.get("postcode") {
            address_parts.push(postal.clone());
        }
        if let Some(city) = nominatim_resp.address.get("city") {
            address_parts.push(city.clone());
        }

        let full_address = address_parts.join(", ");

        // Cache result
        {
            let mut cache = self.cache.write();
            cache.insert(cache_key, full_address.clone());
        }

        debug!("✅ Geocoded: {} -> {}", cache_key, full_address);
        Ok(full_address)
    }

    /// Get address details from string
    pub async fn get_address_details(&self, address: &str) -> Result<GpsCoordinate> {
        let url = format!(
            "https://nominatim.openstreetmap.org/search?q={}&format=json&limit=1",
            urlencoding::encode(address)
        );

        let response = self
            .client
            .get(&url)
            .header("User-Agent", "AMP-App/1.0")
            .send()
            .await?;

        #[derive(Deserialize)]
        struct SearchResult {
            lat: String,
            lon: String,
        }

        let results: Vec<SearchResult> = response.json().await?;

        if let Some(result) = results.first() {
            GpsCoordinate::new(&result.lat, &result.lon)
                .map_err(|e| crate::error::AMPError::GeolocationFailed(e))
        } else {
            Err(crate::error::AMPError::GeolocationFailed(
                "No results found".to_string(),
            ))
        }
    }
}

impl Default for GeolocationService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_valid_coordinates() {
        let geo = GeolocationService::new();
        let coord = GpsCoordinate::new("55.6050", "13.0038").unwrap();

        // This would hit the real API
        // let address = geo.get_address_from_gps(&coord, "en").await;
        // assert!(address.is_ok());
    }
}
