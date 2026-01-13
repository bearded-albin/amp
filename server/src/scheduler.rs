/// Background scheduler for daily updates
use crate::{config::Config, AppState};
use chrono::{Local, Timelike};
use tracing::info;

pub async fn run_scheduler(config: &Config, state: &AppState) {
    loop {
        let now = Local::now();

        // Check if it's 2 AM
        if now.hour() == 2 && now.minute() < 1 {
            info!("🔄 Running scheduled update at 2 AM");

            if let Err(e) = crate::python_bridge::pull_and_process(config, state).await {
                tracing::error!("Scheduled update failed: {}", e);
            }
        }

        // Sleep for 1 minute before next check
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}
