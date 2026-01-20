# Correlation Tests - Quick Reference

## TL;DR - Run Tests Now

```bash
# Run all correlation tests
cargo test --lib correlation_tests

# Run with verbose output to see all test names
cargo test --lib correlation_tests -- --list

# Run a specific test
cargo test --lib correlation_tests::tests::test_exact_match_distance_zero

# Run and show output (println, assertions, etc)
cargo test --lib correlation_tests -- --nocapture

# Run in release mode (faster)
cargo test --lib correlation_tests --release
```

## What Gets Tested

| # | Test Name | What It Validates | Status |
|---|-----------|------------------|--------|
| 1 | Precision | 7+ decimals preserved | ✅ Essential |
| 2 | Exact Match | Distance = 0 for identical coords | ✅ Critical |
| 3 | Within Threshold | dist < 0.001 is relevant | ✅ Critical |
| 4 | Outside Threshold | dist > 0.001 is rejected | ✅ Critical |
| 5 | Closest Selection | Picks closest zone of many | ✅ Critical |
| 6 | Output Structure | Correct AdressInfo generated | ✅ Critical |
| 7 | Batch Processing | 3 addresses matched correctly | ✅ Critical |
| 8 | Degenerate Geometry | Handles point zones | ⚠️ Edge case |
| 9 | Threshold Calibration | Tests various threshold values | ℹ️ Informational |
| 10 | Real Malmö Data | Works with production coordinates | ✅ Critical |
| 11 | Precision Loss | No rounding errors | ✅ Critical |
| 12 | Batch Performance | 100 addresses × 50 zones | ℹ️ Performance |

## Key Numbers to Know

### Distance to Degrees (approximate)
- `0.0001` = 10 meters
- `0.001` = 100 meters ← **Your threshold**
- `0.01` = 1 km
- `0.1` = 10 km

### Decimal Places
- Minimum: 7 decimal places
- Tested: 13-14 decimal places
- Preserved from API via `Decimal::from_f64_retain()`

## Test Data

### Coordinates Used
**Malmö, Sweden (sample values):**
```
Lilla Torg:              13.1945945, 55.5932645
Västra Varvsgatan 41:  13.2004523, 55.6043210
```

## Expected Output - All Pass

```
running 12 tests
test correlation_tests::tests::test_batch_performance_many_records ... ok
test correlation_tests::tests::test_correlation_output_structure ... ok
test correlation_tests::tests::test_decimal_precision_preserved ... ok
test correlation_tests::tests::test_degenerate_line_segment ... ok
test correlation_tests::tests::test_exact_match_distance_zero ... ok
test correlation_tests::tests::test_multiple_addresses_correlation ... ok
test correlation_tests::tests::test_multiple_lines_closest_selected ... ok
test correlation_tests::tests::test_no_precision_loss_in_calculations ... ok
test correlation_tests::tests::test_outside_threshold ... ok
test correlation_tests::tests::test_real_world_malmo_coordinates ... ok
test correlation_tests::tests::test_threshold_calibration_values ... ok
test correlation_tests::tests::test_within_threshold ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured
```

## If a Test Fails

### Problem: "sqrt not found"
**Solution:** Add feature to `Cargo.toml`:
```toml
rust_decimal = { version = "1.34", features = ["sqrt"] }
```

### Problem: Test expects different distance
**Solution:** Check if threshold needs adjustment.
Edit `correlation.rs` line with threshold:
```rust
let threshold = Decimal::from_str_exact("0.001").unwrap_or_default();
//                                      ^^^^^^^ change this value
```

Common values:
- `"0.0005"` = ~50m (very strict)
- `"0.001"` = ~100m (current)
- `"0.002"` = ~200m (relaxed)
- `"0.005"` = ~500m (very relaxed)

## How to Adjust Threshold

**If many valid parking zones are REJECTED:**
```
0.001 → 0.002  (looser)
```

**If many WRONG parking zones are selected:**
```
0.001 → 0.0005  (stricter)
```

Then run tests again:
```bash
cargo test --lib correlation_tests
```

## Integration Testing

To run full test suite including this:
```bash
cargo test
```

## Understanding Test Names

```
test correlation_tests::tests::test_exact_match_distance_zero ... ok
    \______ ________________/\_______ _______/\____ __________/
           |                         |              |
           |                         |              +-- Test result
           |                         +-- Test function name
           +-- Test module path
```

## View Specific Test Details

Each test file has detailed comments explaining:
- What's being tested
- Why it matters
- Expected results
- Real-world implications

See `core/src/correlation_tests.rs` for full details.

Or read `core/CORRELATION_TESTS.md` for comprehensive documentation.

## Performance Targets

- All tests should complete in <2 seconds
- 100 addresses × 50 zones in <200ms (release build)
- No memory leaks or panics

## CI/CD Integration

Add to GitHub Actions:
```yaml
- name: Run correlation tests
  run: cargo test --lib correlation_tests
  
- name: Run with release optimizations
  run: cargo test --lib correlation_tests --release
```

## Debugging a Specific Scenario

Run single test with output:
```bash
cargo test --lib correlation_tests::tests::test_multiple_addresses_correlation -- --nocapture
```

This shows:
- Test execution
- Any println! output
- Assertion failures with details
- Final pass/fail result

## Files Created

1. **core/src/correlation_tests.rs** - 12 comprehensive tests
2. **core/src/lib.rs** - Module declaration (updated)
3. **core/CORRELATION_TESTS.md** - Full test documentation
4. **core/TEST_QUICK_REFERENCE.md** - This file

## Next Steps

1. ✅ Run all tests:
   ```bash
   cargo test --lib correlation_tests
   ```

2. ✅ Verify all 12 pass

3. ✅ Check real data accuracy (Test #10)

4. ✅ Adjust threshold if needed (Test #9 helps)

5. ✅ Integrate into CI/CD

## Questions?

Refer to:
- Full details: `core/CORRELATION_TESTS.md`
- Test source: `core/src/correlation_tests.rs`
- Implementation: `core/src/correlation.rs`

Each test includes inline documentation explaining the scenario and expectations.
