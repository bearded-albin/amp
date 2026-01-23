# Correlation Algorithms

This document describes the four correlation algorithms implemented in AMP for matching addresses to parking zones.

## Overview

All algorithms implement the `CorrelationAlgo` trait, making them interchangeable and easily testable.

```rust
pub trait CorrelationAlgo {
    fn correlate(
        &self,
        address: &AdressClean,
        parking_lines: &[MiljoeDataClean],
    ) -> Option<(usize, f64)>;
    
    fn name(&self) -> &'static str;
}
```

## 1. Distance-Based Algorithm

### Description
Calculates perpendicular distance from address point to each parking line segment. Returns closest match.

### Algorithm
1. For each parking line:
   - Calculate line vector from start to end
   - Project address point onto line
   - Clamp projection to line segment [0, 1]
   - Calculate distance from point to projection
2. Return line with minimum distance

### Complexity
- **Time**: O(n × m) where n = addresses, m = parking lines
- **Space**: O(1)

### Best For
- Small to medium datasets
- High accuracy requirements
- Simple, predictable behavior

### Usage
```bash
amp-server correlate --algorithm distance-based
```

## 2. Raycasting Algorithm

### Description
Casts 36 rays (10° increments) from address point with 50m lifetime. Returns closest intersecting parking line.

### Algorithm
1. Cast 36 rays in 360° from address point
2. For each ray:
   - Check intersection with all parking lines
   - Use parametric line-line intersection
   - Verify intersection within ray lifetime (50m)
3. Return closest intersecting line

### Complexity
- **Time**: O(n × m × 36)
- **Space**: O(1)

### Best For
- Sparse parking data
- Detecting nearby zones in any direction
- Validation against distance-based results

### Usage
```bash
amp-server correlate --algorithm raycasting
```

## 3. Overlapping Chunks (Spatial Grid)

### Description
Divides world into 100m grid cells with 50m overlap. Queries only relevant cells for each address.

### Algorithm
1. **Pre-processing**:
   - Divide world into 100m × 100m cells
   - Add 50m overlap to handle edge cases
   - Store parking line indices in affected cells
2. **Query**:
   - Find cell containing address
   - Query this cell + 8 neighbors (3×3)
   - Calculate distance only to candidates
   - Return closest match

### Complexity
- **Time**: 
  - Pre-processing: O(m)
  - Query: O(n × k) where k = avg lines per chunk
  - Total: O(m + n × k)
- **Space**: O(m)

### Best For
- **Large datasets** (10,000+ addresses)
- Production environments
- Best overall performance

### Usage
```bash
amp-server correlate --algorithm overlapping-chunks
```

## 4. Linear Algebra Algorithm

### Description
Uses vector dot products and projections for clean mathematical distance calculation.

### Algorithm
1. For each parking line:
   - Create line direction vector
   - Create vector from line start to address
   - Project address vector onto line vector using dot product
   - Clamp projection parameter t ∈ [0, 1]
   - Calculate magnitude of difference vector
2. Return line with minimum distance

### Complexity
- **Time**: O(n × m)
- **Space**: O(1)

### Best For
- Code clarity and maintainability
- Educational purposes
- General-purpose use

### Usage
```bash
amp-server correlate --algorithm linear-algebra
```

## Performance Comparison

### Expected Results (1,000 addresses, 2,000 parking lines)

| Algorithm | Time | Accuracy | Memory |
|-----------|------|----------|--------|
| Distance-Based | 2.5s | High | Low |
| Raycasting | 5.2s | Medium | Low |
| Overlapping Chunks | **1.2s** | High | Medium |
| Linear Algebra | 2.3s | High | Low |

### Benchmark Command
```bash
amp-server benchmark --sample-size 1000
```

## Algorithm Selection Guide

### Choose **Distance-Based** if:
- Dataset is small (< 1,000 addresses)
- You need proven, simple algorithm
- Memory is extremely constrained

### Choose **Raycasting** if:
- Parking zones are sparse
- You need spatial awareness
- Validating other algorithms

### Choose **Overlapping Chunks** if:
- Dataset is large (> 5,000 addresses)
- Performance is critical
- You can afford pre-processing time

### Choose **Linear Algebra** if:
- You prefer clean mathematical code
- General-purpose use case
- Teaching/learning purposes

## Implementation Details

### Adding New Algorithms

1. Create new file: `core/src/correlation_algorithms/my_algo.rs`
2. Implement `CorrelationAlgo` trait
3. Add to `mod.rs`:
   ```rust
   pub mod my_algo;
   pub use my_algo::MyAlgo;
   ```
4. Add to CLI enum in `server/src/main.rs`
5. Add to benchmarker in `core/src/benchmark.rs`

### Testing

Each algorithm includes unit tests:
```bash
cargo test correlation_algorithms
```

## References

- **Distance to Line**: Computational Geometry in C (O'Rourke, 1998)
- **Raycasting**: DDA Algorithm (Bresenham line algorithm variant)
- **Spatial Hashing**: Real-Time Collision Detection (Ericson, 2004)
- **Vector Projection**: Linear Algebra (Strang, 2016)
