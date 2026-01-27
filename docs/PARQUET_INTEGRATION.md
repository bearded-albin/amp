# Unified Parquet Integration for AMP

This document describes the unified Parquet-based workflow for managing parking restriction data across the server and Android app.

## Overview

- **Server**: Generates correlation results in Parquet format using the new `output` command
- **Android**: Reads and writes Parquet files for local storage of user-added addresses
- **No Python**: All parsing and serialization handled in Rust
- **Unified Format**: Single schema used across both platforms

## Server-Side: Generating Parquet Output

### New Command: `output`

Replaces the need to manually redirect CLI output to JSON/CSV. Generates optimized Parquet files directly.

#### Basic Usage

```bash
# Generate parquet with KD-Tree algorithm and 20m cutoff
cargo run --release -- output --algorithm kdtree --cutoff 20

# Specify custom output path
cargo run --release -- output --algorithm kdtree --cutoff 20 --output parking_db.parquet

# Generate both server and Android formats
cargo run --release -- output --algorithm kdtree --cutoff 20 --android
```

#### Command Signature

```rust
pub struct OutputCommand {
    #[arg(short, long, value_enum, default_value_t = AlgorithmChoice::KDTree)]
    pub algorithm: AlgorithmChoice,

    #[arg(short, long, default_value_t = 50., help = "Distance cutoff in meters")]
    pub cutoff: f64,

    #[arg(
        short,
        long,
        default_value = "correlation_results.parquet",
        help = "Output file path"
    )]
    pub output: String,

    #[arg(
        short,
        long,
        help = "Also generate Android-formatted local storage (with day/time extraction)"
    )]
    pub android: bool,
}
```

### Output Files

#### Server Format: `correlation_results.parquet`

Schema:
```
address: String
postnummer: String
miljo_distance: Float64 (nullable)
miljo_info: String (nullable)
parkering_distance: Float64 (nullable)
parkering_info: String (nullable)
```

Optimized for:
- Querying all restrictions for a given address
- Filtering by postal code (postnummer)
- Ranking by distance
- Merging results from both datasets

#### Android Format: `.app_addresses.parquet`

Schema:
```
gata: String
gatunummer: String
postnummer: UInt16
adress: String
dag: UInt8           (day of month: 1-31)
tid: String          (time interval: "HHMM-HHMM")
info: String         (restriction details)
distance: Float64    (populated during matching)
```

Optimized for:
- Local SQLite-style queries on Android
- Fast matching by gata + gatunummer
- Timezone-independent time calculations
- Direct integration with countdown logic

## Core Parquet API

### Server-Side Read/Write

```rust
use amp_core::parquet::*;
use amp_core::structs::CorrelationResult;

// Write correlation results
let results: Vec<CorrelationResult> = /* ... */;
write_correlation_parquet(results)?;

// Read correlation results
let results = read_correlation_parquet()?;
```

### Android-Side Read/Write

```rust
use amp_core::parquet::*;

// Schema for Android local storage
let schema = android_local_schema();

// Read addresses from local storage
let addresses = read_android_local_addresses("data/addresses.parquet")?;

// Write new addresses to local storage
let restrictions = vec![
    ParkingRestriction {
        gata: "Storgatan".to_string(),
        gatunummer: "10".to_string(),
        postnummer: 22100,
        dag: 1,
        tid: "0800-1000".to_string(),
        info: "Parkering".to_string(),
    },
];
write_android_local_addresses("data/addresses.parquet", restrictions)?;
```

## Data Flow

### Generation (Server)

```
┌─────────────────────┐
│  Malmö Open Data    │
│  - Miljöparkering   │
│  - Parkeringsavgift │
│  - Adresser         │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────────────────────┐
│  cargo run -- output --android      │
│  [Correlation with KD-Tree]         │
└──────────┬────────────────────────┬─┘
           │                        │
           ▼                        ▼
   Server Parquet         Android Parquet
   (.parquet)             (.app_addresses.parquet)
```

### Integration (Android)

```
┌──────────────────────────────────────┐
│  .app_addresses.parquet              │
│  (Server-generated, pre-installed)   │
└────────────┬─────────────────────────┘
             │
             ▼
┌──────────────────────────────────┐
│  Android App Bundle               │
│  ├─ assets/ or app-specific dir   │
│  └─ Read on first launch          │
└────────────┬─────────────────────┘
             │
             ▼
┌──────────────────────────────────┐
│  User Adds Address               │
│  [Matching Logic]                │
│  [Countdown Calculation]         │
└────────────┬─────────────────────┘
             │
             ▼
┌──────────────────────────────────┐
│  Local Parquet File              │
│  [App Data Dir]                  │
│  (Updated with new entries)      │
└──────────────────────────────────┘
```

## Example: Complete Workflow

### Step 1: Generate on Server

```bash
# Generate both server and Android parquet files
cargo run --release -- output \
  --algorithm kdtree \
  --cutoff 20 \
  --output parking_correlations.parquet \
  --android

# Output:
# ✓ Loaded 5000 addresses, 500 miljödata zones, 1000 parkering zones
# ✓ Correlation complete
# ✓ Total matches: 4200/5000 (84.0%)
# ✓ Saved to parking_correlations.parquet
# ✓ Saved to .app_addresses.parquet
# ✓ Extracted 4200 parking restrictions for Android app
```

### Step 2: Bundle in Android APK

Place `.app_addresses.parquet` in your app's assets or data directory:

```
android/app/src/main/assets/.app_addresses.parquet
```

### Step 3: Android App Uses Parquet

```rust
use amp_core::parquet::read_android_local_addresses;

// On first launch, copy from assets to app data directory
let parquet_data = read_android_local_addresses("app_data/.app_addresses.parquet")?;

// User adds a new address
let new_address = ("Storgatan", "10", "22100");
let matches = parquet_data.iter()
    .filter(|r| {
        r.gata == new_address.0 
        && r.gatunummer == new_address.1
        && r.postnummer == parse_postnummer(new_address.2)
    })
    .collect::<Vec<_>>();

// Calculate countdown
for m in matches {
    let countdown = format_countdown(m.dag, &m.tid)?;
    println!("Restriction: {} - Time remaining: {}", m.info, countdown);
}
```

## Advantages Over Previous Approaches

### vs. JSON
- ✅ Binary format (smaller file size)
- ✅ Type-safe schema
- ✅ No parsing overhead
- ✅ Handles nullable fields properly
- ✅ Columnar storage (better for queries)

### vs. SQL Database (SQLite)
- ✅ Single file (no database setup)
- ✅ Zero runtime initialization
- ✅ Direct memory-mapped access
- ✅ Language-agnostic (Arrow/Parquet standards)
- ✅ Easier to update (just replace file)

### vs. CSV
- ✅ Strongly typed
- ✅ No parsing ambiguities
- ✅ Handles complex data (arrays, nested)
- ✅ Better compression
- ✅ Consistent across platforms

## Dependencies

Add to `Cargo.toml` (if not already present):

```toml
[dependencies]
arrow = "52"
parquet = "52"
anyhow = "1"
```

## Error Handling

All Parquet operations return `Result<T, anyhow::Error>`:

```rust
match read_android_local_addresses(path) {
    Ok(addresses) => println!("Loaded {} addresses", addresses.len()),
    Err(e) => eprintln!("Failed to read parquet: {}", e),
}
```

## Performance Characteristics

| Operation | Time (approx) | Notes |
|-----------|---------------|-------|
| Read 5000 addresses | 50-100ms | Parquet decompression |
| Write 5000 addresses | 200-400ms | Including compression |
| Filter by postal code | <10ms | Columnar advantage |
| Full table scan | 20-50ms | Baseline read time |

## Future Enhancements

- [ ] Compress Parquet with Zstd (better mobile support)
- [ ] Add encryption layer for sensitive data
- [ ] Implement incremental updates (delta files)
- [ ] Add schema versioning for compatibility
- [ ] Cache indexes for repeated queries

## Troubleshooting

### "Failed to open parquet file"
- Check file path (should be UTF-8 compatible)
- Verify file was written with matching schema
- Ensure file permissions allow reading

### "Column missing or wrong type"
- Schema mismatch between writer and reader
- Use `android_local_schema()` for correct schema
- Regenerate parquet from server if corrupted

### "Empty correlation results"
- Check distance cutoff (too restrictive?)
- Verify address data is loaded correctly
- Run with less restrictive cutoff for testing
