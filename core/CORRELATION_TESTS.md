# Correlation Tests Documentation

Comprehensive test suite for validating address-to-parking-zone correlation with Decimal precision.

## Running Tests

### Run all tests
```bash
cargo test --lib correlation_tests
```

### Run specific test
```bash
cargo test --lib correlation_tests::tests::test_exact_match_distance_zero
```

### Run with verbose output
```bash
cargo test --lib correlation_tests -- --nocapture
```

### Run tests and show test names
```bash
cargo test --lib correlation_tests -- --list
```

## Test Suite Overview

### ✅ TEST 1: Decimal Precision Preservation
**File:** `test_decimal_precision_preserved`

**Purpose:** Verify that Decimal coordinates maintain at least 7 decimal places throughout the system.

**What it tests:**
- Coordinate storage maintains full precision
- No rounding errors from f64 conversion
- At least 7 decimal places are preserved

**Why it matters:** GPS coordinates need high precision to accurately identify locations in urban environments.

**Example values:**
```
X: 13.1881234567890 (13+ decimal places)
Y: 55.6048765432109 (13+ decimal places)
```

---

### ✅ TEST 2: Exact Match - Distance Zero
**File:** `test_exact_match_distance_zero`

**Purpose:** Validate that when an address is directly on a parking zone, distance = 0.

**What it tests:**
- Perfect coordinate alignment returns zero distance
- Degenerate line segment (identical endpoints) handled correctly
- Correlation is marked as `relevant: true`

**Why it matters:** Ensures baseline functionality when coordinates perfectly match.

**Scenario:**
```
Address:  (13.1881234, 55.6048765)
ParkZone: [(13.1881234, 55.6048765), (13.1881234, 55.6048765)]
Result:   Distance = 0 ✓
```

---

### ✅ TEST 3: Within Threshold - Valid Match
**File:** `test_within_threshold`

**Purpose:** Verify that addresses close to parking zones (distance < 0.001) are correctly marked as relevant.

**What it tests:**
- Distance calculation works correctly
- Threshold comparison (< 0.001) functions properly
- Relevant flag is set correctly

**Why it matters:** Confirms the threshold of 0.001 (approximately 100 meters at equator) correctly identifies valid parking zone associations.

**Threshold reference:**
- 0.0001 ≈ 10 meters
- 0.001 ≈ 100 meters ← **Current threshold**
- 0.01 ≈ 1000 meters (1 km)
- 0.1 ≈ 10 km

---

### ✅ TEST 4: Outside Threshold - Rejection
**File:** `test_outside_threshold`

**Purpose:** Ensure addresses far from parking zones (distance > 0.001) are marked as not relevant.

**What it tests:**
- Distance calculations exceed threshold
- Relevant flag is set to `false`
- No false positive matches

**Why it matters:** Prevents incorrect zone assignments for distant addresses.

**Scenario:**
```
Address:      (13.200000, 55.600000)
ParkZone:     [(13.100000, 55.500000), (13.100100, 55.500100)]
Distance:     > 0.001
Result:       relevant: false ✓
```

---

### ✅ TEST 5: Multiple Lines - Closest Selection
**File:** `test_multiple_lines_closest_selected`

**Purpose:** When multiple parking zones exist, verify the algorithm selects the closest one.

**What it tests:**
- Parallel comparison of distances to multiple zones
- Correct index returned for closest zone
- No selection errors with multiple options

**Why it matters:** Real-world data has overlapping parking zones; must choose the most relevant.

**Scenario:**
```
Zone 1: Distance = 5.0 km  (index 0)
Zone 2: Distance = 0.0005  (index 1) ← Selected ✓
Zone 3: Distance = 3.0 km  (index 2)
```

---

### ✅ TEST 6: Correlation Output Structure
**File:** `test_correlation_output_structure`

**Purpose:** Validate the final AdressInfo struct contains correct linked data.

**What it tests:**
- All address fields copied correctly
- All parking zone fields linked correctly
- Relevant flag set appropriately
- No data loss or corruption

**Why it matters:** End-to-end validation that the full correlation pipeline works.

**Output verification:**
```rust
AdressInfo {
    relevant: true,              // Correct threshold check
    postnummer: 202,            // From address
    adress: "Storgatan 15",     // From address
    gata: "Storgatan",          // From address
    gatunummer: "15",           // From address
    info: "Parking Zone A",      // From zone
    tid: "08:00-18:00",         // From zone
    dag: 1,                      // From zone
}
```

---

### ✅ TEST 7: Batch Processing
**File:** `test_multiple_addresses_correlation`

**Purpose:** Verify correct operation with multiple addresses and parking zones.

**What it tests:**
- Batch processing without cross-contamination
- Each address paired with closest zone
- Correct relevant/not-relevant distribution
- Parallel processing correctness

**Why it matters:** Real deployment processes thousands of records; must be correct at scale.

**Scenario:**
```
3 Addresses
2 Parking Zones

Result:
✓ Address 1 → Zone A (relevant)
✓ Address 2 → Zone B (relevant)
✓ Address 3 → No zone (not relevant, too far)
```

---

### ✅ TEST 8: Degenerate Line Segment
**File:** `test_degenerate_line_segment`

**Purpose:** Handle edge case where parking zone has identical start/end coordinates.

**What it tests:**
- Degenerate line (point) handling
- Distance calculation to point geometry
- No crashes or NaN values

**Why it matters:** Real data might have malformed or point-based zones.

**Scenario:**
```
ParkZone Start: (13.1880000, 55.6040000)
ParkZone End:   (13.1880000, 55.6040000) ← Same point
Result:         Treated as point, distance calculated
```

---

### ✅ TEST 9: Threshold Calibration
**File:** `test_threshold_calibration_values`

**Purpose:** Test distance calculations against multiple threshold values.

**What it tests:**
- 0.0001 (10m) - Too small, fails
- 0.001 (100m) - Current threshold, close
- 0.01 (1km) - Larger threshold, passes
- 0.1 (10km) - Very large threshold, passes

**Why it matters:** Helps calibrate the optimal threshold for your specific use case.

**Usage:**
If many valid parking zones are being rejected (marked not-relevant), increase threshold.
If too many false positives occur, decrease threshold.

---

### ✅ TEST 10: Real-World Malmö Coordinates
**File:** `test_real_world_malmo_coordinates`

**Purpose:** Test with actual Malmö addresses and realistic coordinate precision.

**What it tests:**
- Real coordinate values from API
- Realistic address formatting
- End-to-end pipeline with real data

**Why it matters:** Validates system works with production data.

**Real coordinates tested:**
```
Lilla Torg 1:              (13.1945945, 55.5932645)
Västra Varvsgatan 41:      (13.2004523, 55.6043210)

Corresponding parking zones:
Lilla Torg Miljözon:       [(13.1940000, 55.5930000), (13.1950000, 55.5935000)]
Västra Varvsgatan Miljözon: [(13.2000000, 55.6040000), (13.2010000, 55.6045000)]
```

---

### ✅ TEST 11: Precision Loss Detection
**File:** `test_no_precision_loss_in_calculations`

**Purpose:** Ensure no precision is lost during distance calculations.

**What it tests:**
- Ultra-precise coordinates (13+ decimal places)
- Very small distances computed accurately
- Decimal arithmetic maintains precision

**Why it matters:** Rust Decimal type guarantees precision; this validates the guarantee.

**Coordinates tested:**
```
X: 13.18812345678901  (14 decimal places)
Y: 55.60487654321098  (14 decimal places)
```

---

### ✅ TEST 12: Batch Performance
**File:** `test_batch_performance_many_records`

**Purpose:** Verify performance and correctness with 100+ records.

**What it tests:**
- Processing 100 addresses against 50 zones
- No performance degradation
- Parallel processing correct
- Memory handling

**Why it matters:** Production will process thousands of records; must be efficient.

**Performance notes:**
- Uses `rayon` for parallel processing
- Should complete in <1 second for 100×50 matrix
- Linear scaling with address count

---

## Threshold Adjustment Guide

If you need to adjust the closeness threshold from 0.001:

### In `correlation.rs`, find:
```rust
let threshold = Decimal::from_str_exact("0.001").unwrap_or_default();
if dist < &threshold {
```

### Change "0.001" to:
- **"0.0005"** → ~50 meters (very strict)
- **"0.001"** → ~100 meters (current)
- **"0.002"** → ~200 meters (relaxed)
- **"0.005"** → ~500 meters (very relaxed)
- **"0.01"** → ~1 km (very loose)

### Then re-run tests:
```bash
cargo test --lib correlation_tests -- --nocapture
```

Tests will show which values pass/fail with the new threshold.

---

## Interpreting Test Output

### Successful run:
```bash
running 12 tests
test correlation_tests::tests::test_batch_performance_many_records ... ok
test correlation_tests::tests::test_correlation_output_structure ... ok
test correlation_tests::tests::test_decimal_precision_preserved ... ok
...
test result: ok. 12 passed; 0 failed; 0 ignored
```

### Failed test example:
```bash
test correlation_tests::tests::test_within_threshold ... FAILED

failures:

---- correlation_tests::tests::test_within_threshold stdout ----
thread 'correlation_tests::tests::test_within_threshold' panicked at 'Distance 0.0015 should be less than threshold 0.001'
```

**Action:** The distance (0.0015) exceeds your threshold (0.001). Either:
1. Increase threshold to 0.002
2. Verify coordinate data is accurate
3. Check calculation logic

---

## Adding New Tests

Template for adding a new test:

```rust
#[test]
fn test_your_scenario_name() {
    // Setup test data
    let point = AdressClean {
        coordinates: [decimal("13.1881234"), decimal("55.6048765")],
        // ... fields ...
    };

    let line = MiljoeDataClean {
        coordinates: [
            [decimal("13.1880000"), decimal("55.6040000")],
            [decimal("13.1890000"), decimal("55.6050000")],
        ],
        // ... fields ...
    };

    // Run function
    let results = find_closest_lines(&[point], &[line]);

    // Verify expectations
    assert!(results[0].is_some(), "Should have a result");
    assert_eq!(results[0].unwrap().0, 0, "Should match first line");
}
```

---

## Continuous Integration

Run tests in CI/CD pipeline:

```yaml
# .github/workflows/test.yml
- name: Run correlation tests
  run: cargo test --lib correlation_tests
```

---

## Performance Metrics

Current performance expectations (100 addresses × 50 zones):
- **Serial processing:** ~500ms
- **Parallel processing:** ~50-100ms
- **Speedup:** ~5-10x

Use `--release` for benchmarking:
```bash
cargo test --lib correlation_tests --release -- --nocapture
```

---

## Troubleshooting

### "sqrt" feature not found
```
error: method `sqrt` not found for struct `rust_decimal::Decimal`
```
**Fix:** Add to `Cargo.toml`:
```toml
rust_decimal = { version = "1.34", features = ["sqrt"] }
```

### Tests panic with precision errors
**Fix:** Verify Decimal::from_f64_retain() is used in api.rs:
```rust
let x = Decimal::from_f64_retain(coords[0]).unwrap_or_default();
```

### False negatives (zones not linked)
- Increase threshold (change "0.001" to larger value)
- Verify coordinate precision in API data
- Check for coordinate system mismatches

### False positives (wrong zones linked)
- Decrease threshold (change "0.001" to smaller value)
- Verify parking zone geometry is correct
- Check for duplicate/overlapping zones

---

## Summary

| Test | Purpose | Threshold Relevant | Critical |
|------|---------|-------------------|----------|
| 1 | Precision | N/A | ✓ Yes |
| 2 | Exact match | 0.0 | ✓ Yes |
| 3 | Within threshold | < 0.001 | ✓ Yes |
| 4 | Outside threshold | > 0.001 | ✓ Yes |
| 5 | Closest selection | Any | ✓ Yes |
| 6 | Output structure | < 0.001 | ✓ Yes |
| 7 | Batch processing | < 0.001 | ✓ Yes |
| 8 | Degenerate geometry | Any | - No |
| 9 | Calibration | Various | - No |
| 10 | Real-world data | < 0.001 | ✓ Yes |
| 11 | Precision loss | Near 0 | ✓ Yes |
| 12 | Performance | < 0.001 | - No |

**Critical tests** must pass before deployment.
