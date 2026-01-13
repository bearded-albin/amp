/// AMP Server - Rust-native Linux binary
mod api;
mod config;
mod python_bridge;
mod scheduler;

use axum::{extract::State, response::IntoResponse, routing::{get, post}, Router};
use parking_lot::RwLock;
use std::sync::Arc;
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing::{error, info};

use amp_core::{
    models::{CleaningSchedule, HealthResponse},
    error::Result,
};
use std::collections::HashMap;
use chrono::Utc;

/// Shared server state
pub struct AppState {
    schedules: Arc<RwLock<HashMap<String, CleaningSchedule>>>,
    last_update: Arc<RwLock<Option<chrono::DateTime<Utc>>>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("amp_server=debug".parse()?)
                .add_directive("tower_http=debug".parse()?),
        )
        .init();

    info!("🚀 Starting AMP Server...");

    // Load configuration
    let config = config::Config::from_env()?;
    info!("📋 Configuration loaded: {:?}", config);

    // Initialize state
    let state = AppState {
        schedules: Arc::new(RwLock::new(HashMap::new())),
        last_update: Arc::new(RwLock::new(None)),
    };

    // Initial data load via Python bridge
    if let Err(e) = python_bridge::pull_initial_data(&config, &state).await {
        error!("⚠️  Initial data pull failed: {}", e);
    }

    // Start scheduler for daily updates
    let scheduler_state = state.clone();
    let scheduler_config = config.clone();
    tokio::spawn(async move {
        scheduler::run_scheduler(&scheduler_config, &scheduler_state).await;
    });

    // Build router
    let app = Router::new()
        .route("/health", get(api::handlers::health))
        .route("/api/cleaning-schedule", get(api::handlers::get_schedule))
        .route("/api/address-check", post(api::handlers::check_address))
        .route("/api/force-update", post(api::handlers::force_update))
        .layer(TraceLayer::new_for_http())
        .with_state(Arc::new(state));

    // Start server
    let listener = tokio::net::TcpListener::bind(&config.bind_addr).await?;
    info!("🎯 Server listening on {}", config.bind_addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    info!("👋 Server shutdown");
    Ok(())
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        Self {
            schedules: Arc::clone(&self.schedules),
            last_update: Arc::clone(&self.last_update),
        }
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install CTRL+C signal handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Received CTRL+C signal");
        },
        _ = terminate => {
            info!("Received SIGTERM signal");
        },
    }
}
