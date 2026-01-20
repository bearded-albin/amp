# AMP Core Library

Core geospatial correlation library for matching addresses to environmental parking zones in Malmö, Sweden.

**Documentation Hub:** See [docs/](../docs/) folder for comprehensive architecture guides.

## Quick Start

```rust
use amp_core::api::api;
use amp_core::correlation::correlation;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Fetch data from ArcGIS services
    let (addresses, zones) = api().await?;
    
    // Correlate addresses to parking zones
    let results = correlation(addresses, zones);
    
    // Filter for relevant matches
    let matched: Vec<_> = results.iter().filter(|r| r.relevant).collect();
    println!("Found {} matching addresses", matched.len());
    
    Ok(())
}
```

## Modules

### `api` - ArcGIS Integration

**File:** `src/api.rs`

**Purpose:** Fetch and transform geospatial data from ArcGIS Feature Services.

**Key Functions:**
- `api()` - Main entry point, returns addresses and parking zones
- `ArcGISClient::fetch_all_features()` - Handles pagination
- `ArcGISClient::to_adress_clean()` - Transforms addresses
- `ArcGISClient::to_miljoe_clean()` - Transforms parking zones

**Documentation:** [docs/API_ARCHITECTURE.md](../docs/API_ARCHITECTURE.md)

**Reference Keys:**
- [REF-API-001] through [REF-API-014] - See API architecture document

### `correlation` - Geographic Matching

**File:** `src/correlation.rs`

**Purpose:** Match addresses to parking zones using point-to-line distance calculations.

**Key Functions:**
- `correlation()` - Main correlation pipeline
- `find_closest_lines()` - Parallel distance finding
- `distance_point_to_line_squared()` - Distance calculation

**Algorithm:**
- Perpendicular distance from point to line segment
- Distance threshold: 0.001 degrees (≈111 meters)
- Parallel processing with Rayon

**Documentation:** [docs/CORRELATION_ALGORITHM.md](../docs/CORRELATION_ALGORITHM.md)

**Reference Keys:**
- [REF-CORR-001] through [REF-CORR-015] - See algorithm document

### `structs` - Data Types

**File:** `src/structs.rs`

**Types:**
- `AdressClean` - Cleaned address with coordinates
- `MiljoeDataClean` - Parking zone with time/day restrictions
- `AdressInfo` - Correlation result with matching zone info

### `parquet` - Data Serialization

**File:** `src/parquet.rs`

**Purpose:** Save/load correlation results to Parquet format.

**Functions:**
- `save_to_parquet()` - Write results to file
- `load_from_parquet()` - Read results from file

### `error` - Error Types

**File:** `src/error.rs`

**Custom error handling for library operations.**

## Testing

The library includes 12 comprehensive tests with pass/not token system:

**File:** `src/correlation_tests.rs`

**Tests Cover:**
1. Decimal precision preservation
2. Exact coordinate matches
3. Threshold acceptance/rejection
4. Multiple zone selection
5. Batch processing
6. Edge cases (degenerate segments)
7. Real-world Malmö coordinates

**Documentation:** [docs/TEST_STRATEGY.md](../docs/TEST_STRATEGY.md)

**Run All Tests:**
```bash
cargo test --release -p amp_core
```

**Reference Keys:**
- [REF-TEST-001] through [REF-TEST-020] - See test strategy document

## Data Flow

```
┌─ ArcGIS Services ─┐
│ Addresses         │
│ Parking Zones     │
└───────────────────┘
          │
          │ api()
          │ [REF-API-013]
          │
          │
    ┌─────▼─────┐
    │           │
    │  Raw Data │
    │           │
    └─────│─────┘
          │
          │ correlation()
          │ [REF-CORR-002]
          │
    ┌─────▼────────┐
    │              │
    │  Results     │
    │  - relevant  │
    │  - info      │
    │  - tid       │
    │  - dag       │
    └──────────────┘
```

## Key Concepts

### Precision Handling

Coordinates use `Decimal` type to maintain 7+ decimal places (±0.111 meters precision). This prevents floating-point accumulation errors in distance calculations.

**Reference:** [REF-CORR-005]

### Distance Threshold

**0.001 degrees ≈ 111 meters**

Covers neighborhood-level precision while avoiding false matches with distant zones.

**Reference:** [REF-CORR-006]

### Parallelization

Rayon `par_iter()` processes multiple addresses simultaneously. No file I/O - data stays in memory.

**Reference:** [REF-CORR-009]

## Performance

- **100 addresses + 50 zones:** <1 second
- **Memory:** Linear with data size
- **Optimization:** Parallel processing, early exit on closest match

**Reference:** [REF-TEST-017]

## Dependencies

- `rust-decimal` - Precise coordinate handling
- `rayon` - Parallel iteration
- `reqwest` - Async HTTP client
- `tokio` - Async runtime
- `serde` - Serialization
- `geojson` - GeoJSON parsing
- `parquet` - Columnar storage

## Error Handling

- Missing API fields: Records skipped gracefully
- Network errors: Propagated to caller
- Degenerate geometries: Handled without panics
- NaN values: Prevented by Decimal type

**Reference:** [REF-API-010]

## Integration Points

### With Android App

The Android module queries correlation results and displays relevant parking restrictions.

**See:** [android/](../android/) module documentation

### With iOS App

The iOS module integrates similar correlation functionality with native APIs.

**See:** [ios/](../ios/) module documentation

### With Server

The server module exposes correlation results via REST API.

**See:** [server/](../server/) module documentation

## Reference Key Index

### API References
- API Architecture: [docs/API_ARCHITECTURE.md](../docs/API_ARCHITECTURE.md)
  - [REF-API-001] Overview
  - [REF-API-002] ArcGISClient structure
  - [REF-API-003] Pagination strategy
  - [REF-API-004] GeoJSON point extraction
  - [REF-API-005] GeoJSON polygon extraction
  - [REF-API-006] Address transformation
  - [REF-API-007] Parking zone transformation
  - [REF-API-008] Malmö addresses source
  - [REF-API-009] Environmental parking source
  - [REF-API-010] Error handling
  - [REF-API-011] Pagination performance
  - [REF-API-012] Async/await runtime
  - [REF-API-013] Integration entry point
  - [REF-API-014] Field mapping flexibility

### Correlation References
- Correlation Algorithm: [docs/CORRELATION_ALGORITHM.md](../docs/CORRELATION_ALGORITHM.md)
  - [REF-CORR-001] Overview
  - [REF-CORR-002] Algorithm flow
  - [REF-CORR-003] Point-to-line function
  - [REF-CORR-004] Mathematical steps
  - [REF-CORR-005] Precision handling
  - [REF-CORR-006] Threshold justification
  - [REF-CORR-007] Threshold application
  - [REF-CORR-008] Result structure
  - [REF-CORR-009] Parallel processing
  - [REF-CORR-010] Complexity analysis
  - [REF-CORR-011] Degenerate segments
  - [REF-CORR-012] No lines available
  - [REF-CORR-013] No correlation edge case
  - [REF-CORR-014] Test coverage
  - [REF-CORR-015] Pass/not token system

### Test References
- Test Strategy: [docs/TEST_STRATEGY.md](../docs/TEST_STRATEGY.md)
  - [REF-TEST-001] Overview
  - [REF-TEST-002] Pass/not token definition
  - [REF-TEST-003] Boolean assertions
  - [REF-TEST-004] Equality assertions
  - [REF-TEST-005] Inequality assertions
  - [REF-TEST-006] Test 1: Precision
  - [REF-TEST-007] Test 2: Exact match
  - [REF-TEST-008] Test 3: Within threshold
  - [REF-TEST-009] Test 4: Outside threshold
  - [REF-TEST-010] Test 5: Multiple lines
  - [REF-TEST-011] Test 6: Output structure
  - [REF-TEST-012] Test 7: Batch processing
  - [REF-TEST-013] Test 8: Degenerate segment
  - [REF-TEST-014] Test 9: Threshold calibration
  - [REF-TEST-015] Test 10: Real-world coords
  - [REF-TEST-016] Test 11: Precision loss
  - [REF-TEST-017] Test 12: Performance
  - [REF-TEST-018] Running tests
  - [REF-TEST-019] Results interpretation
  - [REF-TEST-020] Best practices

## Contributing

When modifying code:
1. Replace inline comments with reference keys pointing to docs
2. Update docs/ files with detailed explanations
3. Add pass/not tokens for all tests
4. Reference existing keys when relevant

## License

See [LICENSE](../LICENSE) file.
