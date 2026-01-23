# Testing

AMP uses a multi-layered testing strategy to ensure correlation accuracy and performance.

## Test Structure

```
core/
├── src/
│   ├── correlation_tests.rs       # Integration tests
│   └── correlation_algorithms/
│       ├── distance_based.rs      # Unit tests inline
│       ├── raycasting.rs
│       └── ...
└── tests/
    └── benchmark_tests.rs        # Performance tests
```

## Unit Tests

Each algorithm module includes tests for core functionality.

**Example:** `distance_based.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;
    
    #[test]
    fn test_point_to_line_distance() {
        let point = [
            Decimal::from_str("55.6050").unwrap(),
            Decimal::from_str("13.0024").unwrap()
        ];
        
        let line_start = [
            Decimal::from_str("55.6040").unwrap(),
            Decimal::from_str("13.0020").unwrap()
        ];
        
        let line_end = [
            Decimal::from_str("55.6060").unwrap(),
            Decimal::from_str("13.0030").unwrap()
        ];
        
        let dist = point_to_line_distance(point, line_start, line_end);
        
        // Should be perpendicular distance
        assert!(dist < 50.0);
        assert!(dist > 0.0);
    }
    
    #[test]
    fn test_correlate_finds_closest() {
        let address = AdressClean {
            coordinates: [
                Decimal::from_str("55.6050").unwrap(),
                Decimal::from_str("13.0024").unwrap()
            ],
            adress: "Test Street 1".to_string(),
            gata: "Test".to_string(),
            gatunummer: "1".to_string(),
            postnummer: "211 22".to_string(),
        };
        
        let zones = vec![
            MiljoeDataClean {
                coordinates: [
                    [Decimal::from_str("55.6040").unwrap(), Decimal::from_str("13.0020").unwrap()],
                    [Decimal::from_str("55.6060").unwrap(), Decimal::from_str("13.0030").unwrap()]
                ],
                info: "Zone 1".to_string(),
                tid: "06:00-18:00".to_string(),
                dag: 31,
            },
            MiljoeDataClean {
                coordinates: [
                    [Decimal::from_str("55.7000").unwrap(), Decimal::from_str("13.1000").unwrap()],
                    [Decimal::from_str("55.7010").unwrap(), Decimal::from_str("13.1010").unwrap()]
                ],
                info: "Zone 2 (far)".to_string(),
                tid: "08:00-16:00".to_string(),
                dag: 31,
            },
        ];
        
        let algo = DistanceBasedAlgo;
        let result = algo.correlate(&address, &zones);
        
        assert!(result.is_some());
        let (idx, dist) = result.unwrap();
        assert_eq!(idx, 0);  // Should match Zone 1 (closer)
        assert!(dist < 50.0);
    }
}
```

**Run:**
```bash
cargo test --lib distance_based
```

## Integration Tests

**Module:** `core/src/correlation_tests.rs`

Tests full correlation pipeline with real data structures.

```rust
#[cfg(test)]
mod correlation_tests {
    use super::*;
    
    fn create_test_data() -> (Vec<AdressClean>, Vec<MiljoeDataClean>) {
        let addresses = vec![
            AdressClean {
                coordinates: [
                    Decimal::from_str("55.6050").unwrap(),
                    Decimal::from_str("13.0024").unwrap()
                ],
                adress: "Stortorget 1".to_string(),
                gata: "Stortorget".to_string(),
                gatunummer: "1".to_string(),
                postnummer: "211 22".to_string(),
            },
            // ... more addresses
        ];
        
        let zones = vec![
            MiljoeDataClean {
                coordinates: [
                    [Decimal::from_str("55.6045").unwrap(), Decimal::from_str("13.0020").unwrap()],
                    [Decimal::from_str("55.6055").unwrap(), Decimal::from_str("13.0028").unwrap()]
                ],
                info: "Miljözon Stortorget".to_string(),
                tid: "06:00-18:00".to_string(),
                dag: 31,
            },
            // ... more zones
        ];
        
        (addresses, zones)
    }
    
    #[test]
    fn test_all_algorithms_agree() {
        let (addresses, zones) = create_test_data();
        
        let algos: Vec<Box<dyn CorrelationAlgo>> = vec![
            Box::new(DistanceBasedAlgo),
            Box::new(RTreeSpatialAlgo::new(&zones)),
            Box::new(OverlappingChunksAlgo::new(&zones)),
        ];
        
        for address in &addresses {
            let results: Vec<Option<(usize, f64)>> = algos
                .iter()
                .map(|algo| algo.correlate(address, &zones))
                .collect();
            
            // All algorithms should find same zone (or all fail)
            if let Some((idx, _)) = results[0] {
                for result in &results[1..] {
                    assert!(result.is_some());
                    assert_eq!(result.unwrap().0, idx);
                }
            }
        }
    }
    
    #[test]
    fn test_threshold_enforcement() {
        let (addresses, zones) = create_test_data();
        let algo = RTreeSpatialAlgo::new(&zones);
        
        for address in &addresses {
            if let Some((_, dist)) = algo.correlate(address, &zones) {
                assert!(dist <= 50.0, "Distance {} exceeds threshold", dist);
            }
        }
    }
}
```

**Run:**
```bash
cargo test --test correlation_tests
```

## Benchmark Tests

Performance validation using real datasets.

**Example:** `core/tests/benchmark_tests.rs`

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use amp_core::correlation_algorithms::*;

fn benchmark_algorithms(c: &mut Criterion) {
    let (addresses, zones) = load_test_data();  // 1000 addresses, 200 zones
    
    c.bench_function("distance_based", |b| {
        let algo = DistanceBasedAlgo;
        b.iter(|| {
            for addr in &addresses {
                black_box(algo.correlate(addr, &zones));
            }
        });
    });
    
    c.bench_function("rtree", |b| {
        let algo = RTreeSpatialAlgo::new(&zones);
        b.iter(|| {
            for addr in &addresses {
                black_box(algo.correlate(addr, &zones));
            }
        });
    });
}

criterion_group!(benches, benchmark_algorithms);
criterion_main!(benches);
```

**Run:**
```bash
cargo bench
```

## Real-World Validation

Manual verification against known address-zone pairs from Malmö city records.

**Process:**
1. Fetch live data from ArcGIS API
2. Run correlation with all algorithms
3. Compare results to manual inspection
4. Verify threshold compliance (all matches ≤50m)

**Command:**
```bash
amp-server correlate --algorithm rtree > results.txt
# Inspect "10 Addresses with Largest Distances" section
# All should be ≤50.0m
```

## Test Coverage

**Unit Tests:**
- Point-to-line distance calculation
- Edge cases (point before/after line segment)
- Coordinate precision (Decimal vs f64)

**Integration Tests:**
- Algorithm consistency (all find same zone)
- Threshold enforcement
- Dual dataset correlation
- Missing data handling

**Performance Tests:**
- Algorithm comparison (relative speed)
- Memory usage
- Scalability (1K, 10K, 100K addresses)

**Validation Tests:**
- Real Malmö data (100K+ addresses)
- Known address-zone pairs
- Threshold verification

## Continuous Integration

**GitHub Actions:** `.github/workflows/correlation-tests.yml`

```yaml
name: Correlation Algorithm Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Run tests
        run: cargo test --release
      
      - name: Run benchmarks
        run: cargo bench --no-fail-fast
      
      - name: Validate threshold
        run: |
          cargo run --release -p amp_server -- correlate --algorithm rtree > output.txt
          grep "Threshold verification: All matches" output.txt
```

## Running All Tests

```bash
# Unit tests
cargo test --lib

# Integration tests
cargo test --test '*'

# Benchmarks
cargo bench

# Full suite
cargo test --all && cargo bench

# With coverage (requires tarpaulin)
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

## Test Data

**Location:** Test data embedded in source files (no external files).

**Structure:**
- 10-100 addresses per test
- 5-50 zones per test
- Known distances for validation

**Generation:**
```rust
fn create_test_address(lat: &str, lon: &str, name: &str) -> AdressClean {
    AdressClean {
        coordinates: [
            Decimal::from_str(lat).unwrap(),
            Decimal::from_str(lon).unwrap()
        ],
        adress: name.to_string(),
        gata: name.split(' ').next().unwrap().to_string(),
        gatunummer: "1".to_string(),
        postnummer: "211 22".to_string(),
    }
}
```

## Known Issues

**Floating-point precision:**
- Use `Decimal` for coordinates
- Convert to `f64` only for final distance

**Edge cases:**
- Address exactly on zone boundary → distance = 0.0
- Multiple zones at same distance → returns first found

## Related Documentation

- [Algorithms](algorithms.md) — Algorithm details
- [Architecture](architecture.md) — System design
- [core/README.md](../core/README.md) — Core library guide
