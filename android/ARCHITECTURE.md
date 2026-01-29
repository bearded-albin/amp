# Amp Android App Architecture

## Overview

The Amp Android app is built with Dioxus 0.7.3 and manages parking restriction data through a multi-layer system:

1. **Database Layer**: Server provides `parking_db.parquet` with static correlations
2. **Local Storage**: User adds addresses, which are stored with validation state
3. **Bucketing Layer**: Valid addresses are categorized by time remaining
4. **UI Layer**: Components display addresses organized by urgency

## Data Structures

### Database Format

The server provides a parquet file containing:

```rust
pub struct ParkingRestriction {
    pub postnummer: String,      // "222 22" format
    pub adress: String,          // "Testgatan 123b"
    pub gata: String,            // "Testgatan"
    pub gatunummer: String,      // "123b"
    pub info: String,            // Long description (future tooltip)
    pub tid: String,             // "0800 - 1200" format
    pub dag: u8,                 // 1-31
}
```

### Local Storage Format

User-added addresses with validation and activity tracking:

```rust
pub struct StoredAddress {
    pub gata: String,                      // User input: street name
    pub gatunummer: String,                // User input: street number
    pub postnummer: String,                // User input: postal code
    pub valid: bool,                       // Does it match the database?
    pub active: bool,                      // Display in panels or hide?
    pub matched_entry: Option<ParkingRestriction>,  // Database entry if matched
}
```

## Component Hierarchy

```
App (mod.rs)
├── Stores: stored_addresses (Vec<StoredAddress>)
├── Computes: bucketed (HashMap<PanelBucket, Vec<StoredAddress>>)
├── Handlers: add, toggle_active, remove
│
├── TopBar
│   └── Input: gata, gatunummer, postnummer
│   └── Actions: Add, GPS (TODO)
│
├── Adresser
│   └── Displays: All stored addresses
│   └── Actions: Toggle visibility, Remove
│
└── Categories Section
    ├── Active (≤ 4 hours)
    ├── Six (4-6 hours)
    ├── Day (6h - 1 day)
    ├── Month (1-31 days)
    └── NotValid (No restriction or inactive)
```

## Data Flow

### Adding an Address

1. User enters: gata, gatunummer, postnummer in TopBar
2. TopBar calls `on_add_address` callback
3. App creates `StoredAddress` with fuzzy matching
4. `fuzzy_match_address()` looks up in database
5. If found: `valid=true`, `matched_entry=Some(...)`
6. If not found: `valid=false`, `matched_entry=None`
7. Address added to `stored_addresses` signal
8. `use_effect` recomputes `bucketed` map
9. Panelers re-render with filtered addresses

### Matching Logic

**Current**: Exact match only

```rust
fn fuzzy_match_address(
    gata: &str,
    gatunummer: &str,
    postnummer: &str,
) -> Option<StaticAddressEntry>
```

**TODO**: Implement fuzzy matching (Levenshtein distance) for:
- Case-insensitive matching
- Whitespace tolerance
- Typo correction

### Bucketing Logic

Addresses are bucketed by remaining time until restriction deadline:

- **Now** (Active): ≤ 4 hours
- **Six**: 4-6 hours
- **Day**: 6 hours - 1 day
- **Month**: 1-31 days
- **NotValid**: Invalid or no restriction

See `countdown.rs` for time calculation logic.

## Component Communication

All communication follows Dioxus 0.7.3 patterns:

1. **Props**: Components receive data via props
2. **EventHandler**: Child components communicate via `EventHandler<T>` callbacks
3. **Signals**: App maintains global state with `use_signal`
4. **Effects**: Computed state (bucketing) updated with `use_effect`

### Example: Remove Address

```rust
// App (mod.rs)
let handle_remove_address = move |index: usize| {
    let mut addrs = stored_addresses.write();
    if index < addrs.len() {
        addrs.remove(index);
    }
};

// Pass to Adresser
Adresser {
    on_remove_address: handle_remove_address,
}

// Adresser calls it
button {
    onclick: move |_| on_remove_address.call(idx),
    "×"
}
```

## TODOs - Android-Specific Features

### 1. GPS Location Reading (HIGH PRIORITY)

**File**: `android/src/ui/topbar.rs:32`

```rust
// TODO: Implement GPS location reading for Android
let handle_gps_click = move |_| {
    let location = read_device_gps_location();  // TODO
    // Match location to stored addresses
};
```

**Implementation Notes**:
- Use Android `LocationManager` or Fused Location Provider
- Return coordinates (lat, lon)
- Match against database to find address at location
- Populate input fields with matched address

### 2. Local Persistent Storage (HIGH PRIORITY)

**File**: `android/src/ui/mod.rs:121, 131, 141`

```rust
// TODO: Read addresses from Android persistent storage on app start
use_effect(move || {
    let addrs = read_addresses_from_device();
    stored_addresses.set(addrs);
});

// TODO: Write addresses to Android persistent storage
let handle_add_address = move |gata: String, gatunummer: String, postnummer: String| {
    let new_addr = StoredAddress::new(gata, gatunummer, postnummer);
    let mut addrs = stored_addresses.write();
    addrs.push(new_addr);
    write_addresses_to_device(&addrs);  // TODO
};
```

**Implementation Notes**:
- Use Android SharedPreferences or Room database
- Serialize/deserialize `Vec<StoredAddress>` as JSON or Parquet
- Load on app startup
- Write on every modification

### 3. Android Notifications (MEDIUM PRIORITY)

**File**: `android/src/ui/mod.rs:146`

```rust
// TODO: Implement Android notifications
let handle_send_notification = move |title: &str, body: &str| {
    send_android_notification(title, body);
};
```

**Implementation Notes**:
- Use Android NotificationCompat
- Send alerts when address enters "Active" bucket (≤ 4 hours)
- Include countdown in notification
- Add action to navigate to address in maps

### 4. Fuzzy Address Matching (MEDIUM PRIORITY)

**File**: `android/src/ui/mod.rs:35-57`

```rust
fn fuzzy_match_address(
    gata: &str,
    gatunummer: &str,
    postnummer: &str,
) -> Option<StaticAddressEntry> {
    // TODO: Implement Levenshtein distance or similar
    // Allow typos, extra whitespace, case differences
}
```

**Implementation Notes**:
- Consider dependency: `strsim` crate for Levenshtein
- Set similarity threshold (e.g., 90%)
- Search gata and gatunummer separately from database
- Fall back to exact match if fuzzy fails

### 5. Database Tooltip Expansion (LOW PRIORITY)

**File**: `server/src/data.rs` (future expansion)

Current database has `info` field. Future expansion will add:
- Separate `tooltip` field with detailed restriction info
- UI will display on long-press or hover

## Dioxus 0.7.3 Best Practices Used

### Signal Usage

✅ **Correct**: Signal at App level, derived in components

```rust
let mut stored_addresses = use_signal::<Vec<StoredAddress>>(Vec::new);
let mut bucketed = use_signal::<HashMap<...>>(HashMap::new());
```

✅ **Correct**: Clone signals for closures

```rust
let handle_add_address = {
    let mut addresses = addresses.to_owned();
    move |data| { addresses.write().push(data); }
};
```

❌ **Avoid**: Capturing signals directly in closures without cloning

### Component Props

✅ **Correct**: Pass data via props, use EventHandler for callbacks

```rust
#[component]
fn MyComponent(
    data: Vec<Item>,
    on_action: EventHandler<String>,
) -> Element {
    rsx! {
        button {
            onclick: move |_| on_action.call("value".to_string()),
        }
    }
}
```

❌ **Avoid**: Passing signals as props across components

### Effects

✅ **Correct**: Use `use_effect` for derived state

```rust
use_effect(move || {
    let computed = compute_value(&stored_addresses.read());
    bucketed.set(computed);
});
```

### External Functions

✅ **Correct**: Extract non-component logic to pure functions

```rust
pub fn fuzzy_match_address(gata: &str, gatunummer: &str, postnummer: &str) -> Option<...> {
    // Pure function, no Dioxus dependencies
}
```

These can be called from components and tested independently.

## File Structure

```
android/src/
├── main.rs                 # App entry point
├── ui/
│   ├── mod.rs             # App component, StoredAddress, bucketing
│   ├── topbar.rs          # User input, GPS button
│   ├── adresser.rs        # Display stored addresses
│   └── paneler.rs         # Category panels (Active, Six, Day, Month, NotValid)
├── countdown.rs           # Time calculations, bucketing logic
├── matching.rs            # Database lookup
└── static_data.rs         # Data loading
```

## Testing

Existing tests in `countdown.rs` and `matching.rs` verify:
- Time parsing and formatting
- Bucket categorization
- Input validation

TODO: Add integration tests for:
- Address matching workflow
- Bucketing pipeline
- Component rendering with various address states

## Performance Considerations

1. **Database Loading**: Uses `OnceLock` for lazy-load on first access
2. **Bucketing**: Recomputed with `use_effect` on every address change
3. **Component Rendering**: Dioxus handles differential updates

**Future**: If > 10,000 addresses, consider:
- Incremental bucketing updates
- Virtualized list rendering in panels
- Database indexing by postal code

## Server Component Interaction

The server creates the parking database file:

```bash
cd server
cargo run --release -- output --android
```

This produces `.app_addresses.parquet` which should be copied to:

```
android/assets/parking_db.parquet
```

See `static_data.rs` for loading logic.
