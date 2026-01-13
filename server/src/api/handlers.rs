use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use std::sync::Arc;
use chrono::Utc;
use sha2::{Sha256, Digest};

use amp_core::models::HealthResponse;
use crate::AppState;

#[derive(Deserialize)]
pub struct AddressCheckRequest {
    pub address: String,
}

pub async fn health(
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<HealthResponse>) {
    let response = HealthResponse {
        status: "healthy".to_string(),
        timestamp: Utc::now(),
        last_update: None,
        data_points: state.data_points(),
        version: "1.0.0".to_string(),
    };

    (StatusCode::OK, Json(response))
}

pub async fn get_schedule(
    State(_state): State<Arc<AppState>>,
) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "data": {},
            "checksum": "abc123",
            "timestamp": Utc::now()
        })),
    )
}

pub async fn check_address(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<AddressCheckRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "found": false,
            "address": req.address,
        })),
    )
}
