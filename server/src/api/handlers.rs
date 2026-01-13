/// API endpoint handlers
use axum::{
    extract::{Query, State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use amp_core::models::{AddressCheckResponse, HealthResponse, AlertLevel, GpsCoordinate};
use crate::AppState;
use chrono::Utc;

/// Get health status
pub async fn health(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let last_update = *state.last_update.read();
    let data_points = state.schedules.read().len();

    let response = HealthResponse {
        status: "healthy".to_string(),
        timestamp: Utc::now(),
        last_update,
        data_points,
        version: "1.0.0".to_string(),
    };

    (StatusCode::OK, Json(response))
}

#[derive(Deserialize)]
pub struct ScheduleQuery {
    addresses: Option<String>,
    format: Option<String>,
}

/// Get cleaning schedule
pub async fn get_schedule(
    State(state): State<Arc<AppState>>,
    Query(q): Query<ScheduleQuery>,
) -> impl IntoResponse {
    let schedules = state.schedules.read();

    if schedules.is_empty() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({ "error": "No schedule data available" })),
        )
            .into_response();
    }

    let filtered = if let Some(addresses) = q.addresses {
        let addr_list: Vec<_> = addresses.split(',').map(|s| s.trim()).collect();
        schedules
            .iter()
            .filter(|(k, _)| addr_list.contains(&k.as_str()))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect::<std::collections::HashMap<_, _>>()
    } else {
        schedules.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    };

    let format_type = q.format.as_deref().unwrap_or("minimal");
    let data = if format_type == "minimal" {
        filtered
            .iter()
            .map(|(addr, schedule)| {
                (
                    addr.clone(),
                    serde_json::json!({
                        "next_cleaning": schedule.next_cleaning,
                        "frequency": schedule.frequency_hours
                    }),
                )
            })
            .collect::<serde_json::Value>()
    } else {
        serde_json::to_value(&filtered).unwrap()
    };

    let checksum = calculate_checksum(&data);

    (
        StatusCode::OK,
        Json(serde_json::json!({
            "data": data,
            "checksum": checksum,
            "timestamp": Utc::now()
        })),
    )
        .into_response()
}

#[derive(Deserialize)]
pub struct AddressCheckRequest {
    address: String,
    latitude: Option<String>,
    longitude: Option<String>,
}

/// Check if address is in schedule
pub async fn check_address(
    State(state): State<Arc<AppState>>,
    Json(req): Json<AddressCheckRequest>,
) -> impl IntoResponse {
    let schedules = state.schedules.read();

    match schedules.get(&req.address) {
        Some(schedule) => {
            let now = Utc::now();
            let hours_until = (schedule.next_cleaning - now).num_hours();
            let alert_level = AlertLevel::from_hours_until(hours_until);

            (
                StatusCode::OK,
                Json(AddressCheckResponse {
                    found: true,
                    address: req.address,
                    next_cleaning: Some(schedule.next_cleaning),
                    hours_until: Some(hours_until),
                    alert_level: Some(alert_level),
                    frequency: Some(format!("Every {:.1} hours", schedule.frequency_hours)),
                    confidence: Some(schedule.confidence),
                }),
            )
                .into_response()
        }
        None => (
            StatusCode::NOT_FOUND,
            Json(AddressCheckResponse {
                found: false,
                address: req.address,
                next_cleaning: None,
                hours_until: None,
                alert_level: None,
                frequency: None,
                confidence: None,
            }),
        )
            .into_response(),
    }
}

/// Force immediate update
pub async fn force_update(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    // Trigger update logic
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "status": "update_triggered",
            "timestamp": Utc::now()
        })),
    )
}

fn calculate_checksum(data: &serde_json::Value) -> String {
    use sha2::{Sha256, Digest};
    let json_str = serde_json::to_string(data).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(json_str.as_bytes());
    format!("{:x}", hasher.finalize())
}
