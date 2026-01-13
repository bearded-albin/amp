/// Integration tests for AMP core
#[cfg(test)]
mod tests {
    use amp_core::correlation::CorrelationAnalyzer;
    use amp_core::models::{CleaningEvent, GpsCoordinate};
    use chrono::{Utc, Duration};

    #[test]
    fn test_end_to_end_analysis() {
        // Create sample events
        let mut events = Vec::new();
        let coord = GpsCoordinate::new("55.6050", "13.0038").unwrap();
        let base_time = Utc::now();

        for i in 0..5 {
            events.push(CleaningEvent {
                address: "Nygatan 5, Malmö".to_string(),
                coordinate: coord,
                timestamp: base_time + Duration::days(7 * i as i64),
                is_active: true,
            });
        }

        // Analyze
        let analyzer = CorrelationAnalyzer::new(0.75);
        let schedules = analyzer.analyze(events).unwrap();

        // Verify
        assert!(schedules.contains_key("Nygatan 5, Malmö"));
        let schedule = &schedules["Nygatan 5, Malmö"];
        assert!(schedule.confidence > 0.95);
        assert!((schedule.frequency_hours - 168.0).abs() < 1.0);
    }

    #[test]
    fn test_decimal_precision() {
        let coord = GpsCoordinate::new("55.605012345", "13.003812345").unwrap();

        // Verify 8+ decimal places maintained
        let lat_str = coord.latitude.to_string();
        let lon_str = coord.longitude.to_string();

        assert!(lat_str.len() >= 11); // At least 8 decimals
        assert!(lon_str.len() >= 11);
    }

    #[tokio::test]
    async fn test_api_endpoints() {
        // This would test the actual server endpoints
    }
}
