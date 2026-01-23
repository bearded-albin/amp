# Architecture

AMP is organized as a Rust workspace with four modules sharing a common core library.

## System Overview

```
┌────────────────────────────────────────────┐
│           Malmö Open Data (ArcGIS)            │
│   Miljöparkering │ Parkeringsavgifter │ Adresser │
└────────────────┬───────────────┬────────────┘
                 │               │
                 │   HTTP/JSON  │
                 │               │
                 v               v
           ┌─────────────────────────────┐
           │       amp_core Library       │
           │   ─────────────────────  │
           │   • API integration          │
           │   • Data structures          │
           │   • 6 algorithms             │
           │   • Benchmarking             │
           │   • Checksum verification    │
           └───────┬──────────────────────┘
                  │
      ┌───────────┼───────────┐
      │            │            │
      v            v            v
┌─────────┐  ┌─────────┐  ┌─────────┐
│ Server  │  │ Android │  │   iOS   │
│   CLI   │  │   App   │  │   App   │
└─────────┘  └─────────┘  └─────────┘
```

## Core Library (`core/`)

**Purpose:** Geospatial correlation engine

**Modules:**
- `api.rs` — Fetch data from ArcGIS Feature Services
- `structs.rs` — Data types (`AdressClean`, `MiljoeDataClean`, `CorrelationResult`)
- `correlation_algorithms/` — Six algorithm implementations
- `benchmark.rs` — Performance testing framework
- `checksum.rs` — SHA256 data verification
- `parquet.rs` — Columnar storage for results

**Key Types:**
```rust
pub struct AdressClean {
    pub coordinates: [Decimal; 2],  // High-precision lat/lon
    pub adress: String,
}

pub struct MiljoeDataClean {
    pub coordinates: [[Decimal; 2]; 2],  // Line segment
    pub info: String,                     // Zone restrictions
}
```

See: [algorithms.md](algorithms.md), [core/README.md](../core/README.md)

## Server (`server/`)

**Purpose:** Command-line interface for correlation and benchmarking

**Commands:**
```bash
correlate --algorithm <name>  # Run correlation
benchmark --sample-size <n>   # Performance testing  
check-updates                 # Verify data changes
```

**Implementation:** Uses `clap` for CLI, `indicatif` for progress bars, `rayon` for parallelism.

See: [cli-usage.md](cli-usage.md), [server/README.md](../server/README.md)

## Mobile Apps (`android/`, `ios/`)

**Purpose:** Native apps for checking parking restrictions offline

**Framework:** Dioxus (Rust-to-native UI)

**Features:**
- Address search
- Current location detection
- Zone restriction display
- Offline operation (embedded data)

**Build:**
```bash
dx build --android --release  # Android
dx build --ios --release      # iOS
```

See: [android/README.md](../android/README.md), [ios/README.md](../ios/README.md)

## Data Flow

### 1. Data Acquisition

```
ArcGIS API → Reqwest HTTP → GeoJSON → Rust Structs
```

- Three datasets: Miljöparkering, Parkeringsavgifter, Adresser
- Automatic pagination for large datasets
- Graceful error handling for missing fields

See: [api-integration.md](api-integration.md)

### 2. Correlation Processing

```
Addresses + Zones → Algorithm → (Index, Distance) → CorrelationResult
```

- Parallel processing with Rayon
- Distance threshold: 50 meters
- Returns closest parking zone per address

See: [algorithms.md](algorithms.md)

### 3. Result Storage

```
CorrelationResult[] → Apache Parquet → Disk
```

- Columnar format for efficient storage
- Used by mobile apps for offline access

## Design Decisions

### High-Precision Coordinates

**Problem:** Floating-point errors in distance calculations

**Solution:** `rust_decimal::Decimal` for all coordinate math

```rust
// Maintains precision in repeated calculations
let lat: Decimal = Decimal::from_str("55.605")?;
let lon: Decimal = Decimal::from_str("13.002")?;
```

### Parallel Processing

**Problem:** 100K addresses × 2K zones = 200M distance calculations

**Solution:** Rayon data-parallelism

```rust
addresses.par_iter()  // Automatic CPU core utilization
    .map(|addr| algo.correlate(addr, zones))
    .collect()
```

**Performance:** 3-4x speedup on quad-core systems

### Dual Dataset Support

**Problem:** Parking restrictions split across two government datasets

**Solution:** Correlate with both, merge results

```rust
pub struct CorrelationResult {
    pub miljo_match: Option<(f64, String)>,
    pub parkering_match: Option<(f64, String)>,
}
```

## Testing Strategy

- Unit tests per algorithm (`correlation_algorithms/*_test.rs`)
- Integration tests (`core/src/correlation_tests.rs`)
- Benchmark comparisons
- Real-world data validation (1000+ address-zone pairs)

See: [testing.md](testing.md)

## Performance Characteristics

| Component | Complexity | Optimization |
|-----------|------------|-------------|
| Distance-Based | O(n×m) | Rayon parallelism |
| R-Tree | O(n×log m) | Spatial indexing |
| KD-Tree | O(n×log m) | Spatial indexing |
| Grid | O(n+m×k) | Spatial hashing |
| API Fetch | O(m) | Async/await, pagination |

Where:
- n = number of addresses
- m = number of parking zones  
- k = average zones per grid cell

See: [algorithms.md](algorithms.md) for detailed complexity analysis

## Related Documentation

- [Algorithms](algorithms.md) — Correlation algorithms explained
- [CLI Usage](cli-usage.md) — Command-line guide
- [API Integration](api-integration.md) — Data fetching details
- [Testing](testing.md) — Test strategy
