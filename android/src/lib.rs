use amp_core::{AppState, models::Address};

pub mod ui;
pub mod state;

pub use state::AppStateManager;

// For potential server-side rendering or library usage
pub fn create_app_state() -> AppState {
    AppState::new()
}

pub fn add_address_to_state(state: &AppState, address: Address) {
    state.add_address(address);
}

pub fn get_addresses_from_state(state: &AppState) -> Vec<Address> {
    state.get_addresses()
}
