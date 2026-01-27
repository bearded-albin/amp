//! AMP Server - Address-Parking Correlation TUI
//! Ratatui-based interactive interface for correlation, testing, benchmarking and update checks.

mod classification;
mod tui;
mod ui;
mod app;

use crate::app::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Run the Ratatui-based TUI. All functionality is accessed from inside the UI.
    let mut app = App::new()?;
    app.run()?;
    Ok(())
}
