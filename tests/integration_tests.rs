use amp_core::{CorrelationAnalyzer, CleaningEvent, GpsCoordinate};
use chrono::{Utc, Duration};

#[test]
fn test_gps_coordinate_creation() {
    let coord = GpsCoordinate::new("55.6050", "13.0038").unwrap();
    assert!(coord.is_in_malmo());
}

#[test]
fn test_high_precision_coordinates() {
    let coord = GpsCoordinate::new("55.605012345", "13.003812345").unwrap();
    assert!(coord.latitude.to_string().len() >= 11);
}

#[test]
fn test_correlation_analysis() {
    let coord = GpsCoordinate::new("55.6050", "13.0038").unwrap();
    let base_time = Utc::now();

    let mut events = Vec::new();
    for i in 0..5 {
        events.push(CleaningEvent {
            address: "Test Street".to_string(),
            coordinate: coord,
            timestamp: base_time + Duration::days(7 * i),
            is_active: true,
        });
    }

    let analyzer = CorrelationAnalyzer::new(0.75);
    let schedules = analyzer.analyze(events).unwrap();

    assert!(schedules.contains_key("Test Street"));
    let schedule = &schedules["Test Street"];
    assert!(schedule.confidence > 0.5);
}
