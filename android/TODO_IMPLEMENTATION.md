# TODO Implementation Guide

This document provides detailed implementation guidance for each TODO in the Android app.

## 1. GPS Location Reading

**Priority**: HIGH  
**Location**: `android/src/ui/topbar.rs:32` and `android/src/ui/mod.rs`  
**Estimated Effort**: 2-3 days

### Objective

When user clicks the GPS button, the app should:
1. Request device location (lat, lon)
2. Match location to a parking restriction from the database
3. Auto-populate the address input fields
4. Add the address to stored addresses

### Implementation Strategy

#### Step 1: Setup Android Location Permissions

Add to `android/AndroidManifest.xml`:

```xml
<uses-permission android:name="android.permission.ACCESS_FINE_LOCATION" />
<uses-permission android:name="android.permission.ACCESS_COARSE_LOCATION" />
```

#### Step 2: Create Native Wrapper

Create `android/src/android_bridge.rs`:

```rust
#[cfg(target_os = "android")]
pub fn read_device_gps_location() -> Option<(f64, f64)> {
    // TODO: Use JNI or other FFI to call Android LocationManager
    // Return (latitude, longitude) or None if location unavailable
    None
}

#[cfg(not(target_os = "android"))]
pub fn read_device_gps_location() -> Option<(f64, f64)> {
    // Mock for testing on non-Android
    None
}
```

#### Step 3: Implement Location Reverse-Lookup

Add to `matching.rs`:

```rust
pub fn find_address_by_coordinates(lat: f64, lon: f64) -> Option<StaticAddressEntry> {
    let data = get_parking_data();
    
    // Find closest address within reasonable distance (e.g., 100m)
    let mut closest: Option<(f64, StaticAddressEntry)> = None;
    let max_distance = 0.001; // ~100 meters in degrees
    
    for entry in data.values() {
        let distance = calculate_haversine(lat, lon, entry.coordinates[0], entry.coordinates[1]);
        if distance < max_distance {
            if let Some((prev_dist, _)) = &closest {
                if distance < *prev_dist {
                    closest = Some((distance, entry.clone()));
                }
            } else {
                closest = Some((distance, entry.clone()));
            }
        }
    }
    
    closest.map(|(_, entry)| entry)
}

fn calculate_haversine(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    // Implement Haversine distance formula
    // Return distance in degrees (approx)
    0.0 // TODO: implement
}
```

#### Step 4: Wire GPS Button to Handler

Update `topbar.rs`:

```rust
let handle_gps_click = move |_| {
    if let Some((lat, lon)) = read_device_gps_location() {
        if let Some(address) = find_address_by_coordinates(lat, lon) {
            // Auto-populate fields
            gata_input.set(address.gata.clone());
            gatunummer_input.set(address.gatunummer.clone());
            postnummer_input.set(address.postnummer.clone());
            
            // Optionally auto-add
            on_add_address.call((
                address.gata,
                address.gatunummer,
                address.postnummer,
            ));
        } else {
            eprintln!("No address found at this location");
        }
    } else {
        eprintln!("Could not read device location");
    }
};
```

### Testing

```bash
# Test with mock location (if using Android emulator)
# Or test manually on real device with GPS enabled
```

### Dependencies

- `jni` crate for Android FFI (if using JNI)
- Or use existing Dioxus Android support

---

## 2. Local Persistent Storage

**Priority**: HIGH  
**Location**: `android/src/ui/mod.rs` (multiple lines)  
**Estimated Effort**: 2-3 days

### Objective

Persist user-added addresses between app sessions:
1. Load addresses on app startup
2. Save after every modification
3. Handle storage failures gracefully

### Implementation Strategy

#### Option A: Android SharedPreferences (Simple)

Best for small datasets (< 1000 addresses)

```rust
// In android/src/storage.rs
pub fn read_addresses_from_device() -> Vec<StoredAddress> {
    // 1. Get SharedPreferences context
    // 2. Read "stored_addresses" JSON string
    // 3. Deserialize to Vec<StoredAddress>
    
    // Fallback to empty vec if not found
    Vec::new()
}

pub fn write_addresses_to_device(addresses: &[StoredAddress]) -> Result<(), String> {
    // 1. Serialize addresses to JSON
    // 2. Get SharedPreferences context
    // 3. Write "stored_addresses" JSON string
    // 4. Return error if write fails
    
    Ok(())
}
```

Dependencies:
```toml
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

Add to `StoredAddress`:
```rust
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoredAddress {
    // ... fields ...
}
```

#### Option B: Android Room Database (Recommended)

Best for larger datasets or complex queries

```rust
// Use existing database libraries
// Or wrap Android Room via JNI
```

#### Step 1: Create Storage Module

Create `android/src/storage.rs`:

```rust
use crate::ui::StoredAddress;

/// Load stored addresses from persistent storage
/// 
/// Returns empty vec if storage unavailable or no data stored
pub fn read_addresses_from_device() -> Vec<StoredAddress> {
    #[cfg(target_os = "android")]
    {
        // TODO: Implement Android SharedPreferences read
        load_from_shared_preferences()
    }
    
    #[cfg(not(target_os = "android"))]
    {
        // Mock for testing
        Vec::new()
    }
}

/// Write stored addresses to persistent storage
/// 
/// Returns Ok if successful, Err with message if failed
pub fn write_addresses_to_device(addresses: &[StoredAddress]) -> Result<(), String> {
    #[cfg(target_os = "android")]
    {
        // TODO: Implement Android SharedPreferences write
        save_to_shared_preferences(addresses)
    }
    
    #[cfg(not(target_os = "android"))]
    {
        // Mock for testing
        Ok(())
    }
}

#[cfg(target_os = "android")]
fn load_from_shared_preferences() -> Vec<StoredAddress> {
    // TODO: Implement
    Vec::new()
}

#[cfg(target_os = "android")]
fn save_to_shared_preferences(addresses: &[StoredAddress]) -> Result<(), String> {
    // TODO: Implement
    Ok(())
}
```

#### Step 2: Wire to App Component

Update `android/src/ui/mod.rs`:

```rust
use crate::storage::{read_addresses_from_device, write_addresses_to_device};

#[component]
pub fn App() -> Element {
    let mut stored_addresses = use_signal::<Vec<StoredAddress>>(Vec::new);
    
    // Load addresses on app startup
    use_effect(move || {
        let addrs = read_addresses_from_device();
        stored_addresses.set(addrs);
    });
    
    // Save after adding
    let handle_add_address = move |gata: String, gatunummer: String, postnummer: String| {
        let new_addr = StoredAddress::new(gata, gatunummer, postnummer);
        let mut addrs = stored_addresses.write();
        addrs.push(new_addr);
        
        // Persist
        if let Err(e) = write_addresses_to_device(&addrs) {
            eprintln!("Failed to persist addresses: {}", e);
        }
    };
    
    // Save after removing
    let handle_remove_address = move |index: usize| {
        let mut addrs = stored_addresses.write();
        if index < addrs.len() {
            addrs.remove(index);
            
            // Persist
            if let Err(e) = write_addresses_to_device(&addrs) {
                eprintln!("Failed to persist addresses: {}", e);
            }
        }
    };
    
    // Similar for toggle_active
    
    // ... rest of component ...
}
```

#### Step 3: Update main.rs to import storage

```rust
pub mod storage;
pub mod ui;
```

### JSON Format Example

Stored in SharedPreferences as:

```json
[
  {
    "gata": "Storgatan",
    "gatunummer": "10",
    "postnummer": "22100",
    "valid": true,
    "active": true,
    "matched_entry": { ... }
  },
  { ... }
]
```

### Testing

```bash
# On Android emulator:
adb shell
run-as com.example.amp
cat /data/data/com.example.amp/shared_prefs/amp_prefs.xml
```

### Error Handling

- Silently fall back to empty list on read failure
- Log errors on write failure (critical)
- Consider: retry logic, backup storage, user notification

---

## 3. Android Notifications

**Priority**: MEDIUM  
**Location**: `android/src/ui/mod.rs:146`  
**Estimated Effort**: 1-2 days

### Objective

Notify user when an address enters the "Active" bucket (≤ 4 hours remaining)

### Implementation Strategy

#### Step 1: Create notification module

Create `android/src/notifications.rs`:

```rust
pub fn send_android_notification(title: &str, body: &str) -> Result<(), String> {
    #[cfg(target_os = "android")]
    {
        // TODO: Use Android NotificationManager API
        // Create NotificationCompat.Builder
        // Set title and body
        // Show notification
        Ok(())
    }
    
    #[cfg(not(target_os = "android"))]
    {
        eprintln!("NOTIFICATION: {} - {}", title, body);
        Ok(())
    }
}

pub fn cancel_notification(notification_id: i32) -> Result<(), String> {
    // TODO: Cancel notification by ID
    Ok(())
}
```

#### Step 2: Create notification service

The app should:
1. Run a background task checking bucket changes every minute
2. When address enters "Active" bucket, send notification
3. Include countdown in notification body

```rust
use crate::notifications::send_android_notification;
use crate::countdown::format_countdown;

// In App component or separate service
pub async fn monitor_address_changes(addresses: Signal<Vec<StoredAddress>>) {
    loop {
        // TODO: Check every minute (or configurable interval)
        let current_addresses = addresses.read();
        
        for addr in current_addresses.iter() {
            if let Some(entry) = &addr.matched_entry {
                let bucket = bucket_for(entry.dag, &entry.tid);
                
                // If newly entered Active bucket
                if bucket == TimeBucket::Now {
                    let countdown = format_countdown(entry.dag, &entry.tid)
                        .unwrap_or_else(|| "...".to_string());
                    
                    let title = format!("{} står nu att stå under parkering", entry.gata);
                    let body = format!("Tid kvar: {}", countdown);
                    
                    let _ = send_android_notification(&title, &body);
                }
            }
        }
        
        // Sleep for 60 seconds
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}
```

#### Step 3: Wire to App lifecycle

```rust
use crate::notifications::send_android_notification;

#[component]
pub fn App() -> Element {
    let stored_addresses = use_signal::<Vec<StoredAddress>>(Vec::new);
    
    // Spawn notification monitor on app start
    use_effect(move || {
        // TODO: Spawn async task to monitor addresses
        // Call send_android_notification when needed
    });
    
    // ...
}
```

### Notification Action (Future)

Allow notification to open maps to address:

```rust
fn send_notification_with_action(
    title: &str,
    body: &str,
    address: &str,
) -> Result<(), String> {
    // TODO: Add PendingIntent to open maps
    // Intent: "geo:0,0?q=<address>"
    Ok(())
}
```

### Dependencies

```toml
tokio = { version = "1.0", features = ["time"] }
```

---

## 4. Fuzzy Address Matching

**Priority**: MEDIUM  
**Location**: `android/src/ui/mod.rs:35-57`  
**Estimated Effort**: 1 day

### Objective

Allow users to add addresses with typos or whitespace differences

### Implementation Strategy

#### Step 1: Add strsim dependency

```toml
strsim = "0.11"
```

#### Step 2: Implement fuzzy matching

Update `fuzzy_match_address` in `android/src/ui/mod.rs`:

```rust
fn fuzzy_match_address(
    gata: &str,
    gatunummer: &str,
    postnummer: &str,
) -> Option<StaticAddressEntry> {
    // Try exact match first
    if let crate::matching::MatchResult::Valid(entry) = 
        match_address(gata, gatunummer, postnummer) {
        return Some(entry);
    }
    
    // Try fuzzy match
    let data = crate::matching::get_parking_data();
    let mut candidates: Vec<_> = data
        .values()
        .map(|entry| {
            let gata_sim = strsim::jaro_winkler(&gata.to_lowercase(), 
                                                 &entry.gata.to_lowercase());
            let gnum_sim = strsim::jaro_winkler(&gatunummer.to_lowercase(), 
                                                &entry.gatunummer.to_lowercase());
            let postal_sim = strsim::jaro_winkler(postnummer, &entry.postnummer);
            
            // Combined score
            let score = (gata_sim + gnum_sim + postal_sim) / 3.0;
            (score, entry.clone())
        })
        .filter(|(score, _)| *score > 0.85) // 85% similarity threshold
        .collect();
    
    // Return best match
    candidates.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    candidates.first().map(|(_, entry)| entry.clone())
}
```

#### Step 3: Test fuzzy matching

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fuzzy_matching_typo() {
        // "Storgatan" vs "Storgata" (missing 'n')
        let result = fuzzy_match_address("Storgata", "10", "22100");
        // Should find the match
        assert!(result.is_some());
    }
    
    #[test]
    fn test_fuzzy_matching_case() {
        // "storgatan" (lowercase) vs "Storgatan"
        let result = fuzzy_match_address("storgatan", "10", "22100");
        assert!(result.is_some());
    }
}
```

### Alternative: Substring Matching

If Jaro-Winkler is too permissive:

```rust
fn fuzzy_match_address(...) -> Option<...> {
    // Substring matching as fallback
    let data = get_parking_data();
    
    for entry in data.values() {
        let gata_match = entry.gata.to_lowercase().contains(&gata.to_lowercase());
        let gnum_match = entry.gatunummer.to_lowercase().contains(&gatunummer.to_lowercase());
        let postal_match = entry.postnummer.contains(postnummer);
        
        if gata_match && gnum_match && postal_match {
            return Some(entry.clone());
        }
    }
    
    None
}
```

---

## Priority Roadmap

```
Week 1:
- [ ] Local Persistent Storage (HIGH)

Week 2:
- [ ] GPS Location Reading (HIGH)

Week 3:
- [ ] Fuzzy Address Matching (MEDIUM)
- [ ] Android Notifications (MEDIUM)

Week 4:
- [ ] Testing & Polish
- [ ] Database Tooltip Expansion (LOW)
```

## Build & Test Commands

```bash
# Test on Android emulator
cd android
cargo build --target aarch64-linux-android

# Test on device
adb install -r target/aarch64-linux-android/debug/app.apk
adb logcat | grep "Amp"

# Test storage
adb shell
run-as com.example.amp
cat /data/data/com.example.amp/shared_prefs/*
```

## Debugging Tips

1. Use `eprintln!()` for logging (visible in logcat)
2. Add debug buttons in UI for testing
3. Use Android emulator for controlled testing
4. Test with multiple addresses to check performance
