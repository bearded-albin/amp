pub mod error;
pub mod models;
pub mod correlation;
pub mod geolocation;
pub mod state;

pub use error::{AmpError, AmpResult};
pub use models::{GpsCoordinate, CleaningSchedule, Address, AlertLevel};
pub use correlation::CorrelationAnalyzer;
pub use geolocation::Geocoder;
pub use state::AppState;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_precision() {
        let coord = GpsCoordinate::new(55.6050, 13.0038);
        assert!(coord.is_malmo_coords());
    }
}
