use crate::models::StoredAddress;
use parking_lot::RwLock;
use std::sync::Arc;
use std::collections::HashMap;

pub struct AppState {
    stored_addresses: Arc<RwLock<HashMap<String, StoredAddress>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            stored_addresses: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn add_address(&self, address: StoredAddress) {
        let mut stored = self.stored_addresses.write();
        stored.insert(address.address.clone(), address);
    }

    pub fn get_addresses(&self) -> Vec<StoredAddress> {
        let stored = self.stored_addresses.read();
        stored.values().cloned().collect()
    }

    pub fn remove_address(&self, address: &str) {
        let mut stored = self.stored_addresses.write();
        stored.remove(address);
    }

    pub fn clear_all(&self) {
        let mut stored = self.stored_addresses.write();
        stored.clear();
    }

    pub fn count(&self) -> usize {
        self.stored_addresses.read().len()
    }
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        Self {
            stored_addresses: Arc::clone(&self.stored_addresses),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
