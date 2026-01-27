# Android App Integration Guide

## Overview

This document explains how the Android app integrates with the pre-computed address correlations from the server-side processing (`cargo run --release correlate -c 20 -a kdtree`).

## Architecture

### Component Structure

```
android/src/
├── main.rs                 # App entry point
├── static_data.rs          # Embedded address correlations (generated from server)
├── matching.rs             # Address matching logic
├── countdown.rs            # Parking deadline countdown calculations
└── ui/
    ├── mod.rs              # Main App component
    ├── adresser.rs         # Address input form with validation
    ├── paneler.rs          # Category panels for deadline buckets
    └── topbar.rs           # Header component
```

## How It Works

### 1. Static Data Generation (Pre-packaging)

Before building the APK, run on the development machine:

```bash
# Generate correlations with kdtree algorithm and 20 clusters
cargo run --release correlate -c 20 -a kdtree > address_output.json
```

This command:
- Reads parking zone data from the attached JSON files
- Uses the kdtree algorithm to find nearest parking restrictions for each address
- Outputs structured data with address + time info

### 2. Embedding Static Data

The output from step 1 is parsed and embedded in `android/src/static_data.rs`:

```rust
pub fn get_static_addresses() -> HashMap<String, StaticAddressEntry> {
    // All correlations are hardcoded at compile time
    // No runtime calculation needed in the app
}
```

**Advantages:**
- ✅ No runtime overhead
- ✅ No network calls needed
- ✅ Instant address validation
- ✅ Deterministic behavior

**Update process:**
1. Re-run correlation on server
2. Update `static_data.rs` with new entries
3. Rebuild APK

### 3. Address Validation Flow

When user clicks "Add Address" button:

```
User Input (Gata, Gatunummer, Postnummer)
         ↓
    Validate Input (non-empty)
         ↓
    Look up in static correlations
         ↓
    ┌─────────────────┬──────────────────┐
    ↓ Valid          ↓ Invalid
   Show✓ message     Show ✗ message
   Add to addresses  Keep form visible
         ↓
   Route to category panel based on deadline
```

### 4. Countdown Categorization

Once added, each address is categorized by time remaining:

| Category | Time Remaining | Priority |
|----------|----------------|----------|
| **Nu** (Now) | ≤ 4 hours | 🔴 Urgent |
| **Om mindre än 6h** | 4-6 hours | 🟠 High |
| **Inom 24h** | 6h-24h | 🟡 Medium |
| **Inom 1 månad** | 24h-31d | 🟢 Low |
| **Ingen städning här** | Invalid/No data | ⚪ None |

Time calculations use the `dag` (day of month) and `tid` (time interval) from the parking data:
- Example: `dag=15, tid="0800-1000"` means restrictions are every 15th of the month from 08:00-10:00
- Countdown calculates time until next occurrence (current month or next month)

## Implementation Details

### Address Matching

**File:** `android/src/matching.rs`

```rust
pub fn match_address(gata: &str, gatunummer: &str, postnummer: &str) -> MatchResult
```

- Normalizes input (trim whitespace)
- Creates lookup key: `"Storgatan 10-22100"`
- Searches HashMap in constant time O(1)
- Returns `Valid(entry)` or `Invalid`

### Countdown Logic

**File:** `android/src/countdown.rs`

Key functions:

```rust
// Parse time string "0800-1000" → (08:00, 10:00)
pub fn parse_tid_interval(tid: &str) -> Option<(NaiveTime, NaiveTime)>

// Calculate time until next deadline
pub fn remaining_duration(dag: u8, tid: &str) -> Option<Duration>

// Format for display "5d 02h 30m"
pub fn format_countdown(dag: u8, tid: &str) -> Option<String>

// Categorize by bucket
pub fn bucket_for(dag: u8, tid: &str) -> TimeBucket
```

Handles:
- ✅ Month wraparound (e.g., 15th of current month already passed → next month)
- ✅ Invalid dates (e.g., 31st of February)
- ✅ Time zone local to device
- ✅ Proper duration arithmetic

### UI Components

#### `Adresser` (Input Form)

```rust
pub fn Adresser(on_add_valid_address: EventHandler<StaticAddressEntry>) -> Element
```

- Three input fields: Gata, Gatunummer, Postnummer
- Validates input and matches against static data
- Shows success/error feedback
- Clears form on successful add
- Calls parent handler with validated entry

#### `Active`, `Six`, `Day`, `Month`, `NotValid` (Panels)

Each panel displays addresses in its time bucket:

```rust
pub fn Active(addresses: Vec<StaticAddressEntry>, on_remove: EventHandler<String>) -> Element
```

- Filters addresses by time bucket
- Displays countdown for each address
- Provides remove button
- Shows empty state when no addresses

## Data Flow Example

```
1. User types:
   Gata: "Storgatan"
   Gatunummer: "10"
   Postnummer: "22100"

2. Matching checks static_data.rs:
   Key = "Storgatan 10-22100"
   ✓ Found!
   Returns: StaticAddressEntry {
       adress: "Storgatan 10",
       dag: 15,
       tid: "0800-1000",
       ...
   }

3. Countdown calculates:
   Current: 2026-01-27 14:30
   Deadline: 2026-02-15 10:00  (next occurrence)
   Remaining: 18 days, 19 hours, 30 minutes

4. Categorization:
   bucket_for(15, "0800-1000") = TimeBucket::Within1Month

5. UI adds to "Inom 1 månad" panel
   Display: "Storgatan 10" → "18d 19h 30m"
```

## Building for APK

### Prerequisites

1. **Generate correlations:**
   ```bash
   cargo run --release correlate -c 20 -a kdtree > correlations.json
   ```

2. **Parse JSON and update `android/src/static_data.rs`**
   - Use a script or manual process
   - Populate the `entries` vector in `get_static_addresses()`

3. **Verify compilation:**
   ```bash
   cargo build --release --target aarch64-linux-android
   ```

### Include Dependencies

Ensure `Cargo.toml` has required crates:

```toml
[dependencies]
chrono = "0.4"
dioxus = "0.7"

[dev-dependencies]
# For testing countdown logic
chrono = { version = "0.4", features = ["naive"] }
```

## Testing

### Unit Tests

Run with:
```bash
cargo test --lib
```

Tests cover:
- Time interval parsing
- Duration calculations
- Month wraparound
- Countdown formatting
- Address validation

### Integration Testing

1. Add test addresses to `static_data.rs`
2. Test the "Add Address" flow
3. Verify countdown displays correctly
4. Test category assignment
5. Test remove functionality

### Test Data

Use the provided JSON files to create test addresses:
- `parkeringsavgifter.json` - Standard parking zones
- `miljoparkeringar.json` - Environmental zones

## Troubleshooting

### Address not found
- ✅ Verify spelling matches static data exactly
- ✅ Check postal code format (should be 5 digits)
- ✅ Ensure address exists in correlation output

### Countdown shows "..."
- ✅ Check `tid` format is "HHMM-HHMM" (e.g., "0800-1000")
- ✅ Ensure `dag` is 1-31
- ✅ Verify date/time calculations don't have issues

### Address categorized as "Invalid"
- ✅ `remaining_duration()` returned None
- ✅ Check correlation data for the address
- ✅ Verify `dag` and `tid` are correct

## Performance

- **Address lookup:** O(1) HashMap access
- **Countdown calculation:** O(1) arithmetic
- **Categorization:** O(n) filter (n = number of addresses)
- **Memory:** Static data is compiled into binary

## Future Enhancements

1. **Persistent storage:** Save user's addresses to device
2. **Location-based:** Auto-populate addresses from GPS
3. **Notifications:** Alert before deadline
4. **Data updates:** Check for correlation updates over network
5. **Search/autocomplete:** Help users find addresses
6. **Historical data:** Track parking history

## Files Modified

- `android/src/main.rs` - Added module imports
- `android/src/static_data.rs` - NEW: Address correlations
- `android/src/matching.rs` - NEW: Matching logic
- `android/src/countdown.rs` - NEW: Countdown calculations
- `android/src/ui/mod.rs` - Refactored App component
- `android/src/ui/adresser.rs` - Updated with validation
- `android/src/ui/paneler.rs` - Updated with countdown display

## References

- Chrono documentation: https://docs.rs/chrono/latest/chrono/
- Dioxus: https://dioxuslabs.com/
- Parking data: See attached JSON files
