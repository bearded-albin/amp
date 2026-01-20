# Threshold Calibration and Analysis Guide

## Current Threshold: 0.001 (≈100 meters)

This document helps you understand and calibrate the distance threshold for valid parking zone matches.

## Distance to Real-World Conversion

### At Malmö Latitude (55.6°N)

| Decimal Distance | Latitude (km) | Longitude (km) | Approximate |
|-----------------|---------------|-----------------|-------------|
| 0.00001 | 0.001 | 0.0008 | 1 meter |
| 0.0001 | 0.011 | 0.008 | 8-11 meters |
| 0.0005 | 0.056 | 0.040 | 40-56 meters |
| **0.001** | **0.111** | **0.080** | **80-110 meters** |
| 0.002 | 0.222 | 0.160 | 160-220 meters |
| 0.005 | 0.556 | 0.399 | 400-560 meters |
| 0.01 | 1.11 | 0.799 | 800m - 1.1 km |
| 0.02 | 2.22 | 1.598 | 1.6 - 2.2 km |

**Current threshold (0.001):** ~80-110 meters from center of parking zone

## Choosing the Right Threshold

### TEST 9: Threshold Calibration Values

This test evaluates your distance against multiple thresholds:

```rust
let test_thresholds = [
    ("0.0001", false),  // 10 meters - too small
    ("0.001", false),   // 100 meters - close
    ("0.01", true),     // 1000 meters - should pass
    ("0.1", true),      // 10km - definitely passes
];
```

When you run the test, it will show:
- ✓ PASS: Distance is within threshold
- ✗ FAIL: Distance exceeds threshold

## Calibration Strategies

### Strategy 1: Start Loose, Get Tight

1. **Start with 0.01 (1 km)** - Ensure basic functionality
   ```rust
   let threshold = Decimal::from_str_exact("0.01").unwrap_or_default();
   ```
   - Run tests
   - Check: Are real parking zones being linked?
   - Expected: Most tests pass, few rejections

2. **Tighten to 0.005 (500 m)**
   ```rust
   let threshold = Decimal::from_str_exact("0.005").unwrap_or_default();
   ```
   - Run tests
   - Check: Still getting valid matches?
   - Expected: Most tests still pass

3. **Further tighten to 0.002 (200 m)**
   ```rust
   let threshold = Decimal::from_str_exact("0.002").unwrap_or_default();
   ```
   - Run tests
   - Check: Real addresses still matching?
   - Expected: Critical tests pass, edge cases may fail

4. **Final threshold 0.001 (100 m)** ← Current
   ```rust
   let threshold = Decimal::from_str_exact("0.001").unwrap_or_default();
   ```
   - Run tests
   - Check: All critical tests pass?
   - Expected: Only very close parking zones match

### Strategy 2: Data-Driven Analysis

1. **Run with loose threshold:**
   ```bash
   # Change to 0.01
   cargo test --lib correlation_tests -- --nocapture
   ```

2. **Analyze results:**
   - Count "relevant: true" vs "relevant: false"
   - Are the matches geographically sensible?
   - Any obvious false positives?

3. **Gradually tighten** until you find the sweet spot

## Problem Diagnosis

### Symptom 1: Too Many False Positives
**Indicator:** Random parking zones far away are being linked

**Example:**
```
Address: Storgatan 1 (13.1881234, 55.6048765)
Matched to: Zone 5 km away
```

**Diagnosis:** Threshold too loose

**Solution:** Decrease threshold
```rust
// Current
let threshold = Decimal::from_str_exact("0.001").unwrap_or_default();

// Try this
let threshold = Decimal::from_str_exact("0.0005").unwrap_or_default();  // 50 meters
```

**Test to verify:**
```bash
cargo test --lib correlation_tests::tests::test_outside_threshold
```

### Symptom 2: Too Many False Negatives  
**Indicator:** Addresses are marked not-relevant when they clearly should be

**Example:**
```
Address: Storgatan 1
ParkZone: Exactly 150 meters away
Result: Not relevant (should be relevant!)
```

**Diagnosis:** Threshold too tight

**Solution:** Increase threshold
```rust
// Current
let threshold = Decimal::from_str_exact("0.001").unwrap_or_default();

// Try this
let threshold = Decimal::from_str_exact("0.002").unwrap_or_default();  // 200 meters
```

**Test to verify:**
```bash
cargo test --lib correlation_tests::tests::test_within_threshold
```

### Symptom 3: Precision Errors
**Indicator:** Same coordinate sometimes matches, sometimes doesn't

**Diagnosis:** Decimal precision issue or rounding error

**Solution:** Verify Decimal usage in api.rs
```rust
// Check this is used:
let x = Decimal::from_f64_retain(coords[0]).unwrap_or_default();

// NOT this:
let x = Decimal::from_f64(coords[0]).unwrap_or_default();  // WRONG
```

**Test to verify:**
```bash
cargo test --lib correlation_tests::tests::test_no_precision_loss_in_calculations
cargo test --lib correlation_tests::tests::test_decimal_precision_preserved
```

## Real-World Calibration

If you have actual production data:

1. **Run correlation with current threshold:**
   ```bash
   cargo test --lib correlation_tests --release
   ```

2. **Examine results manually:**
   - Are parking zones sensibly close to addresses?
   - Any obvious mismatches?
   - Patterns in false positives/negatives?

3. **Sample analysis:**
   ```rust
   // After running correlation on real data
   let relevant_count = results.iter().filter(|r| r.relevant).count();
   let not_relevant_count = results.len() - relevant_count;
   
   println!("Relevant: {} ({:.1}%)", 
       relevant_count,
       (relevant_count as f64 / results.len() as f64) * 100.0
   );
   ```

4. **Expected distribution:**
   - **30-50% relevant:** Addresses in parking zones
   - **50-70% not-relevant:** Addresses outside zones
   - If ratio is off, adjust threshold

## Test-Specific Calibration

### TEST 9: Use for Threshold Testing

Run just this test to see threshold performance:
```bash
cargo test --lib correlation_tests::tests::test_threshold_calibration_values -- --nocapture
```

Output shows how your calculated distance compares:
```
Test thresholds:
  0.0001 (10m):  FAIL - too small
  0.001 (100m):  FAIL - close
  0.01 (1km):    PASS - large
  0.1 (10km):    PASS - very large
```

If distance is 0.0015:
- Would fail at 0.001 ← Too tight!
- Would pass at 0.002 ← Good zone
- Would pass at 0.01 ← Definitely OK

**Action:** If this test fails at your current threshold, increase it.

## Threshold Decision Matrix

Use this to decide threshold:

```
┌─────────────────────────────────────────┐
│ How close must parking zone be?         │
├──────────────────┬──────────────────────┤
│ VERY CLOSE       │ ≤ 0.0003 (30m)      │
│ (Same street)    │                      │
├──────────────────┼──────────────────────┤
│ CLOSE            │ ≤ 0.001 (100m)      │
│ (Nearby block)   │  ← CURRENT           │
├──────────────────┼──────────────────────┤
│ NEARBY           │ ≤ 0.002 (200m)      │
│ (Walking dist.)  │                      │
├──────────────────┼──────────────────────┤
│ NEIGHBORHOOD     │ ≤ 0.005 (500m)      │
│ (Area-wide)      │                      │
├──────────────────┼──────────────────────┤
│ CITY SCALE       │ ≤ 0.01 (1km)        │
│ (District)       │                      │
└──────────────────┴──────────────────────┘
```

Current threshold (0.001) = CLOSE (nearby block)

## Adjusting in Code

### Location: `core/src/correlation.rs`

Find this line:
```rust
let threshold = Decimal::from_str_exact("0.001").unwrap_or_default();
```

Change "0.001" to your chosen value:

```rust
// Example: Make stricter
let threshold = Decimal::from_str_exact("0.0005").unwrap_or_default();

// Example: Make looser  
let threshold = Decimal::from_str_exact("0.002").unwrap_or_default();
```

Then rebuild and test:
```bash
cargo clean  # Optional but recommended
cargo test --lib correlation_tests
```

## Monitoring in Production

Add logging to monitor threshold performance:

```rust
// In correlation() function
let threshold = Decimal::from_str_exact("0.001").unwrap_or_default();

let mut within_threshold = 0;
let mut outside_threshold = 0;

for (i, res) in results.iter().enumerate() {
    match res {
        Some((line_index, dist)) => {
            if dist < &threshold {
                within_threshold += 1;
            } else {
                outside_threshold += 1;
            }
        }
        None => {}
    }
}

println!("Threshold analysis:");
println!("  Within threshold: {}", within_threshold);
println!("  Outside threshold: {}", outside_threshold);
println!("  Ratio: {:.1}%", 
    (within_threshold as f64 / (within_threshold + outside_threshold) as f64) * 100.0
);
```

## Documentation After Threshold Change

When you adjust the threshold, update:

1. **README.md** - Document the choice
2. **correlation.rs** - Add comment explaining why
3. **THRESHOLD_ANALYSIS.md** - Note the change and results

Example:
```rust
// CHANGED 2026-01-20: Increased from 0.001 to 0.002
// Reason: Real data showed 85% false negatives at 0.001
// Testing confirmed 0.002 provides 5% false positives, acceptable tradeoff
let threshold = Decimal::from_str_exact("0.002").unwrap_or_default();
```

## Summary

✅ **Start**: 0.001 (100 meters) - Current, balanced choice

✅ **Monitor**: Run tests after any data changes

✅ **Analyze**: Test 9 helps understand distance relationships  

✅ **Adjust**: Only if you see systematic false positives/negatives

✅ **Document**: Note any threshold changes with justification

✅ **Test**: Always run full test suite after adjustment

## Quick Adjustment Checklist

- [ ] Identified the problem (too many true/false positives/negatives)
- [ ] Calculated new threshold value
- [ ] Updated `correlation.rs` line with threshold
- [ ] Ran `cargo test --lib correlation_tests`
- [ ] All critical tests (1-7, 10-11) pass
- [ ] Test 9 shows expected threshold behavior
- [ ] Documented the change with comment in code
- [ ] Verified with real production data sample
