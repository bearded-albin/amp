use amp_core::{AppState, models::Address};
use std::sync::Arc;

/// Wrapper for Dioxus signal integration with core AppState
#[derive(Clone)]
pub struct AppStateManager {
    inner: Arc<AppState>,
}

impl AppStateManager {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(AppState::new()),
        }
    }

    pub fn add_address(&self, address: Address) {
        self.inner.add_address(address);
    }

    pub fn remove_address(&self, id: &str) {
        self.inner.remove_address(id);
    }

    pub fn get_addresses(&self) -> Vec<Address> {
        self.inner.get_addresses()
    }

    pub fn get_active_addresses(&self) -> Vec<Address> {
        self.inner.get_active_addresses()
    }

    pub fn toggle_address(&self, id: &str) {
        self.inner.toggle_address(id);
    }

    pub fn clear_all(&self) {
        self.inner.clear_all();
    }

    pub fn address_count(&self) -> usize {
        self.inner.address_count()
    }

    pub fn snapshot(&self) -> amp_core::state::AppStateSnapshot {
        self.inner.snapshot()
    }
}

impl Default for AppStateManager {
    fn default() -> Self {
        Self::new()
    }
}
