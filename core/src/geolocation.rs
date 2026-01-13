use crate::error::Result;
use crate::models::GpsCoordinate;
use std::collections::HashMap;
use parking_lot::RwLock;
use std::sync::Arc;

pub struct GeolocationService {
    cache: Arc<RwLock<HashMap<String, String>>>,
}

impl GeolocationService {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn get_address_from_gps(
        &self,
        coord: &GpsCoordinate,
        _language: &str,
    ) -> Result<String> {
        let cache_key = format!("{:.8},{:.8}", coord.latitude, coord.longitude);

        {
            let cache = self.cache.read();
            if let Some(cached) = cache.get(&cache_key) {
                return Ok(cached.clone());
            }
        }

        // Simulate geocoding
        let address = format!("Street {}, Malmö", coord.latitude);

        {
            let mut cache = self.cache.write();
            cache.insert(cache_key, address.clone());
        }

        Ok(address)
    }
}

impl Default for GeolocationService {
    fn default() -> Self {
        Self::new()
    }
}
