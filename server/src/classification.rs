use crate::ui::{AlgorithmChoice, App};
use amp_core::structs::{AdressClean, CorrelationResult, MiljoeDataClean};

pub fn run_test_mode_legacy(
    algorithm: AlgorithmChoice,
    cutoff: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    // Placeholder: call into the existing run_test_mode implementation if needed.
    // For now, keep behaviour by delegating to a minimal wrapper or leaving as TODO.
    println!(
        "[TUI] Test mode not yet wired: you selected {:?} with cutoff {:.1}m",
        algorithm, cutoff
    );
    Ok(())
}

pub fn run_benchmark_legacy(cutoff: f64) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "[TUI] Benchmark not yet wired: cutoff {:.1}m. Existing CLI benchmark can be re-used here.",
        cutoff
    );
    Ok(())
}

pub fn run_check_updates_legacy() -> Result<(), Box<dyn std::error::Error>> {
    println!("[TUI] Updates check not yet wired. Existing CLI update logic can be re-used.");
    Ok(())
}
