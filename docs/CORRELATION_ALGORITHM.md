# Correlation Algorithm Documentation

## Overview

The correlation module matches addresses to environmental parking zones based on geographic proximity. This document covers the algorithm, distance calculations, and threshold mechanisms.

**Reference:** [REF-CORR-001]

## Core Algorithm: `correlation()`

### Function Signature

```rust
pub fn correlation(points: Vec<AdressClean>, lines: Vec<MiljoeDataClean>) -> Vec<AdressInfo>
```

### Algorithm Flow

1. **Distance Calculation:** Find closest parking zone for each address
2. **Threshold Filtering:** Mark as relevant if distance < 0.001 degrees
3. **Result Construction:** Build `AdressInfo` struct with metadata

**Reference:** [REF-CORR-002]

## Distance Calculation: Point-to-Line Segment

### Function: `distance_point_to_line_squared()`

Computes the perpendicular distance from a point to a line segment using vector projection.

**Parameters:**
- `point`: [Decimal; 2] - Address coordinates
- `line_seg_start`: [Decimal; 2] - Parking zone start point
- `line_seg_end`: [Decimal; 2] - Parking zone end point

**Reference:** [REF-CORR-003]

### Mathematical Steps

1. **Vector Setup:**
   - AB = line end - line start
   - AP = point - line start

2. **Degenerate Check:**
   - If |AB|² = 0 (both endpoints identical), distance = |AP|

3. **Projection Parameter:**
   - t = (AP · AB) / |AB|² (unclamped)
   - t_clamped = clamp(t, 0.0, 1.0)
   - Ensures closest point lies on segment, not line extension

4. **Closest Point:**
   - closest = A + t_clamped * AB

5. **Distance:**
   - distance² = |point - closest|²
   - Return sqrt(distance²)

**Reference:** [REF-CORR-004]

### Precision Handling

**Why Decimal → f64 → Decimal?**
- Mathematical operations (sqrt, projections) require floating-point
- `Decimal::from_f64_retain()` preserves coordinate precision
- Maintains 7+ decimal places throughout calculation
- Avoids cumulative precision loss in iterative calculations

**Reference:** [REF-CORR-005]

## Distance Threshold: 0.001 Degrees

### Threshold Justification

| Decimal Degrees | Approximate Distance | Use Case |
|---|---|---|
| 0.00001 | 1.11 meters | Fine-grained precision |
| 0.0001 | 11.1 meters | Building-level precision |
| 0.001 | 111 meters | **SELECTED** Neighborhood/Zone level |
| 0.01 | 1.11 km | District level |
| 0.1 | 11.1 km | City level |

**Selection Rationale:**
- Captures parking zones within walking distance (~2-3 blocks)
- Reduces false positives (unrelated zones)
- Accommodates address geocoding imprecision
- Aligns with municipal district sizes

**Reference:** [REF-CORR-006]

### Threshold Application

In `correlation()` function:
```
if distance < 0.001 degrees:
    relevant = true
    copy zone metadata (info, tid, dag)
else:
    relevant = false
    use empty defaults
```

**Reference:** [REF-CORR-007]

## Result Structure: `AdressInfo`

### Fields

| Field | Type | Source | Meaning |
|---|---|---|---|
| relevant | bool | Threshold check | Zone matches this address |
| postnummer | u16 | Original address | Postal code |
| adress | String | Original address | Full address |
| gata | String | Original address | Street name |
| gatunummer | String | Original address | Street number |
| info | String | Parking zone OR default | Zone description |
| tid | String | Parking zone OR default | Time restrictions (HH:MM-HH:MM) |
| dag | u8 | Parking zone OR default | Day of week (1-7) |

**Reference:** [REF-CORR-008]

## Parallel Processing: `find_closest_lines()`

### Function Signature

```rust
pub fn find_closest_lines(
    points: &[AdressClean],
    lines: &[MiljoeDataClean],
) -> Vec<Option<(usize, Decimal)>>
```

### Implementation

- **Rayon Parallelization:** `par_iter()` on points
- **Per-Point Operation:** Iterate through all lines, find minimum distance
- **Return Value:** `(line_index, distance)` or `None` if no lines

**Reference:** [REF-CORR-009]

### Complexity

- **Time:** O(P × L) where P = points, L = lines (parallelized)
- **Space:** O(P) for results
- **Practical:** Process 100k addresses + 50k zones in seconds

**Reference:** [REF-CORR-010]

## Edge Cases

### Degenerate Line Segments

**Problem:** Parking zone with both endpoints identical

**Handling:**
- Detected by |AB|² = 0
- Distance = distance to point (not segment)
- No crashes or NaN values

**Reference:** [REF-CORR-011]

### No Lines Available

**Handling:**
- `find_closest_lines()` returns `None` for that point
- `correlation()` catches with pattern match
- Prints debug message, doesn't crash

**Reference:** [REF-CORR-012]

### Multiple Addresses, No Correlation

**Outcome:**
- `relevant = false`
- All metadata fields use `Default::default()`
- Address still present in output (100% coverage)

**Reference:** [REF-CORR-013]

## Test Coverage

The correlation_tests module provides 12 comprehensive tests covering:
- Precision preservation (7+ decimals)
- Exact matches (distance = 0)
- Threshold boundaries
- Rejection of far addresses
- Multiple line selection
- Batch processing (100+ addresses)
- Degenerate segments
- Real-world Malmö coordinates

**Reference:** [REF-CORR-014]

## Pass/Not Token System

Each test includes explicit pass/fail criteria:
- `assert!()` for boolean conditions
- `assert_eq!()` for exact value matching
- `assert!(..., "message")` with human-readable failure descriptions
- All 12 tests must pass for algorithm validity

**Reference:** [REF-CORR-015]
