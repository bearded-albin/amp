mod api;
mod config;
mod scheduler;
mod python_bridge;

use axum::{
    routing::get,
    Router,
    extract::State,
};
use std::sync::Arc;
use tracing_subscriber;
use parking_lot::RwLock;
use std::collections::HashMap;

use amp_core::models::CleaningSchedule;

pub struct AppState {
    schedules: Arc<RwLock<HashMap<String, CleaningSchedule>>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            schedules: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn data_points(&self) -> usize {
        self.schedules.read().len()
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = config::Config::from_env().expect("Failed to load config");
    let state = Arc::new(AppState::new());

    let app = Router::new()
        .route("/health", get(api::handlers::health))
        .route("/api/cleaning-schedule", get(api::handlers::get_schedule))
        .route("/api/address-check", get(api::handlers::check_address))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.bind_addr, config.port))
        .await
        .expect("Failed to bind");

    tracing::info!("🚀 Server listening on {}:{}", config.bind_addr, config.port);

    axum::serve(listener, app)
        .await
        .expect("Server failed");
}

mod api {
    pub mod handlers;
}
