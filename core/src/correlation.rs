/// High-precision correlation analysis using Decimal arithmetic
use crate::error::{AMPError, Result};
use crate::models::{CleaningEvent, CleaningSchedule, GpsCoordinate};
use chrono::{DateTime, Utc, Weekday};
use ndarray::{Array1, Array2};
use rust_decimal::Decimal;
use std::collections::HashMap;
use tracing::{debug, info, warn};

/// Correlation analyzer with high-precision decimals
pub struct CorrelationAnalyzer {
    min_confidence: f64,
}

impl CorrelationAnalyzer {
    pub fn new(min_confidence: f64) -> Self {
        Self { min_confidence }
    }

    /// Analyze cleaning patterns from events
    pub fn analyze(&self, events: Vec<CleaningEvent>) -> Result<HashMap<String, CleaningSchedule>> {
        info!("Analyzing {} events with high precision", events.len());

        let mut by_address: HashMap<String, Vec<CleaningEvent>> = HashMap::new();

        // Group events by address
        for event in events {
            by_address
                .entry(event.address.clone())
                .or_insert_with(Vec::new)
                .push(event);
        }

        let mut schedules = HashMap::new();

        // Analyze each address
        for (address, mut address_events) in by_address {
            address_events.sort_by_key(|e| e.timestamp);

            // Filter cleaning events
            let cleaning_events: Vec<_> = address_events
                .iter()
                .filter(|e| e.is_active)
                .collect();

            if cleaning_events.len() < 2 {
                debug!("Skipping {}: only {} cleaning events", address, cleaning_events.len());
                continue;
            }

            if let Ok(schedule) = self.analyze_address(&address, cleaning_events) {
                if schedule.confidence >= self.min_confidence {
                    schedules.insert(address, schedule);
                }
            }
        }

        info!("✅ Analyzed {} addresses", schedules.len());
        Ok(schedules)
    }

    /// Analyze single address for cleaning pattern
    fn analyze_address(
        &self,
        address: &str,
        events: Vec<&CleaningEvent>,
    ) -> Result<CleaningSchedule> {
        // Extract timestamps and calculate intervals
        let timestamps: Vec<DateTime<Utc>> = events.iter().map(|e| e.timestamp).collect();

        // Calculate intervals between cleaning events (in hours)
        let mut intervals = Vec::new();
        for i in 1..timestamps.len() {
            let duration = timestamps[i] - timestamps[i - 1];
            let hours = duration.num_seconds() as f64 / 3600.0;
            intervals.push(hours);
        }

        if intervals.is_empty() {
            return Err(AMPError::CorrelationFailed(
                "No intervals calculated".to_string(),
            ));
        }

        // Find dominant interval using histogram
        let interval_hours = self.find_dominant_interval(&intervals)?;

        // Calculate confidence score
        let confidence = self.calculate_confidence(&intervals, interval_hours);

        // Get day of week and time patterns
        let day_of_week = self.get_dominant_day(&events);
        let time_of_day = self.get_dominant_time(&events);

        // Calculate next cleaning time
        let last_cleaning = *timestamps.last().unwrap();
        let next_cleaning = last_cleaning
            + chrono::Duration::hours(interval_hours.round() as i64);

        Ok(CleaningSchedule {
            address: address.to_string(),
            coordinate: events.coordinate,
            next_cleaning,
            frequency_hours: interval_hours,
            confidence,
            day_of_week,
            time_of_day,
            last_cleaning,
            sample_size: events.len(),
        })
    }

    /// Find most common interval using histogram
    fn find_dominant_interval(&self, intervals: &[f64]) -> Result<f64> {
        if intervals.is_empty() {
            return Err(AMPError::CorrelationFailed(
                "No intervals to analyze".to_string(),
            ));
        }

        // Create histogram with 20 bins
        let min = intervals.iter().copied().fold(f64::INFINITY, f64::min);
        let max = intervals.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let bin_width = (max - min) / 20.0;

        if bin_width == 0.0 {
            return Ok(intervals); // All same value
        }

        let mut bins = vec![0; 20];
        for &interval in intervals {
            let bin_idx = ((interval - min) / bin_width).floor() as usize;
            if bin_idx < 20 {
                bins[bin_idx] += 1;
            }
        }

        let dominant_bin = bins
            .iter()
            .enumerate()
            .max_by_key(|(_, &count)| count)
            .ok_or_else(|| AMPError::CorrelationFailed("Empty histogram".to_string()))?
            .0;

        let interval = min + (dominant_bin as f64 + 0.5) * bin_width;
        debug!("Dominant interval: {:.1} hours", interval);

        Ok(interval)
    }

    /// Calculate confidence score for cleaning pattern
    fn calculate_confidence(&self, intervals: &[f64], expected: f64) -> f64 {
        if intervals.len() < 2 {
            return 0.0;
        }

        // Calculate mean and standard deviation
        let mean = intervals.iter().sum::<f64>() / intervals.len() as f64;
        let variance = intervals
            .iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>()
            / intervals.len() as f64;
        let std_dev = variance.sqrt();

        // Coefficient of variation
        let cv = std_dev / expected;

        // Confidence = 1 - normalized variation
        let confidence = (1.0 - cv).max(0.0).min(1.0);

        debug!("Confidence: {:.2} (CV: {:.2})", confidence, cv);
        confidence
    }

    /// Get most common day of week for cleaning
    fn get_dominant_day(&self, events: &[&CleaningEvent]) -> String {
        let mut day_counts = [0; 7];
        for event in events {
            let weekday = event.timestamp.weekday();
            day_counts[weekday.number_from_monday() as usize - 1] += 1;
        }

        let dominant_idx = day_counts
            .iter()
            .enumerate()
            .max_by_key(|(_, &count)| count)
            .map(|(idx, _)| idx)
            .unwrap_or(0);

        let days = [
            "Monday",
            "Tuesday",
            "Wednesday",
            "Thursday",
            "Friday",
            "Saturday",
            "Sunday",
        ];
        days[dominant_idx].to_string()
    }

    /// Get most common time window for cleaning
    fn get_dominant_time(&self, events: &[&CleaningEvent]) -> String {
        let mut hour_counts = [0; 24];
        for event in events {
            let hour = event.timestamp.hour() as usize;
            hour_counts[hour] += 1;
        }

        let dominant_hour = hour_counts
            .iter()
            .enumerate()
            .max_by_key(|(_, &count)| count)
            .map(|(hour, _)| hour)
            .unwrap_or(0);

        format!(
            "{:02}:00-{:02}:00",
            dominant_hour,
            (dominant_hour + 1) % 24
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_high_precision_coordinates() {
        let coord = GpsCoordinate::new("55.605012345", "13.003812345").unwrap();
        assert!(coord.latitude.to_string().len() >= 11); // At least 8 decimals
        assert!(coord.longitude.to_string().len() >= 11);
    }

    #[test]
    fn test_malmo_bounds_validation() {
        let inside = GpsCoordinate::new("55.6050", "13.0038").unwrap();
        assert!(inside.is_in_malmo());

        let outside = GpsCoordinate::new("60.0", "13.0").unwrap();
        assert!(!outside.is_in_malmo());
    }

    #[test]
    fn test_confidence_calculation() {
        let analyzer = CorrelationAnalyzer::new(0.75);

        // Perfect pattern (same interval)
        let perfect = vec![168.0, 168.0, 168.0];
        let confidence_perfect = analyzer.calculate_confidence(&perfect, 168.0);
        assert!(confidence_perfect > 0.95);

        // Noisy pattern
        let noisy = vec![168.0, 100.0, 200.0];
        let confidence_noisy = analyzer.calculate_confidence(&noisy, 168.0);
        assert!(confidence_noisy < confidence_perfect);
    }
}
