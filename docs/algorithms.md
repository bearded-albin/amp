# Correlation Algorithms

AMP implements six geospatial correlation algorithms for matching addresses to parking zones.

## Problem Statement

Given:
- Address point P at coordinates (lat, lon)
- Parking zone as LineString: [(lat₁, lon₁), (lat₂, lon₂)]

Find: Closest parking zone within 50 meters

## Common Operations

### Point-to-Line Distance

All algorithms use the same core distance calculation:

```rust
fn point_to_line_distance(
    point: [Decimal; 2],
    line_start: [Decimal; 2],
    line_end: [Decimal; 2],
) -> f64 {
    let dx = line_end[0] - line_start[0];
    let dy = line_end[1] - line_start[1];
    
    let t = ((point[0] - line_start[0]) * dx + (point[1] - line_start[1]) * dy) 
           / (dx * dx + dy * dy);
    
    let t_clamped = t.max(Decimal::ZERO).min(Decimal::ONE);
    
    let closest_x = line_start[0] + t_clamped * dx;
    let closest_y = line_start[1] + t_clamped * dy;
    
    let dist_x = point[0] - closest_x;
    let dist_y = point[1] - closest_y;
    
    (dist_x * dist_x + dist_y * dist_y).sqrt().to_f64().unwrap()
}
```

**Mathematical basis:**
- Projects point onto line segment
- Clamps projection to segment endpoints
- Returns Euclidean distance

## Algorithm Comparison

| Algorithm | Complexity | Pre-process | Query Time | Best For |
|-----------|------------|-------------|------------|----------|
| Distance-Based | O(n×m) | None | Linear | Small datasets |
| Raycasting | O(n×m×36) | None | Linear | Sparse zones |
| Overlapping Chunks | O(n+m×k) | O(m) | Sub-linear | Large datasets |
| R-Tree | O(n×log m) | O(m log m) | Logarithmic | General purpose |
| KD-Tree | O(n×log m) | O(m log m) | Logarithmic | Point queries |
| Grid | O(n+m×k) | O(m) | Sub-linear | Dense grids |

Where:
- n = addresses
- m = parking zones
- k = average zones per cell (<< m)

## 1. Distance-Based Algorithm

**Implementation:** `core/src/correlation_algorithms/distance_based.rs`

**Strategy:** Brute-force distance check

```rust
pub fn correlate(&self, address: &AdressClean, zones: &[MiljoeDataClean]) 
    -> Option<(usize, f64)> 
{
    zones.iter()
        .enumerate()
        .filter_map(|(idx, zone)| {
            let dist = point_to_line_distance(
                address.coordinates,
                zone.coordinates[0],
                zone.coordinates[1]
            );
            if dist <= 50.0 { Some((idx, dist)) } else { None }
        })
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
}
```

**Characteristics:**
- Simple and reliable
- No pre-processing overhead
- Good for < 1000 zones

## 2. Raycasting Algorithm

**Implementation:** `core/src/correlation_algorithms/raycasting.rs`

**Strategy:** Cast 36 rays in 10° increments, find intersecting zones

```rust
pub fn correlate(&self, address: &AdressClean, zones: &[MiljoeDataClean]) 
    -> Option<(usize, f64)> 
{
    let mut closest = None;
    
    for angle in (0..360).step_by(10) {
        let ray = cast_ray(address.coordinates, angle, 50.0);
        
        for (idx, zone) in zones.iter().enumerate() {
            if let Some(intersection) = ray_intersects_line(ray, zone.coordinates) {
                let dist = distance_to_intersection(address.coordinates, intersection);
                closest = update_if_closer(closest, idx, dist);
            }
        }
    }
    
    closest
}
```

**Characteristics:**
- Spatial awareness (directional search)
- 36 rays = 10° resolution
- Higher computational cost

## 3. Overlapping Chunks Algorithm

**Implementation:** `core/src/correlation_algorithms/overlapping_chunks.rs`

**Strategy:** Divide space into 100m×100m grid cells with 50m overlap

```rust
pub struct OverlappingChunksAlgo {
    grid: HashMap<(i32, i32), Vec<usize>>,  // (cell_x, cell_y) -> zone indices
    cell_size: f64,
    overlap: f64,
}

impl OverlappingChunksAlgo {
    pub fn new(zones: &[MiljoeDataClean]) -> Self {
        let mut grid = HashMap::new();
        
        // Pre-process: Insert zones into overlapping cells
        for (idx, zone) in zones.iter().enumerate() {
            let cells = get_overlapping_cells(zone, 100.0, 50.0);
            for cell in cells {
                grid.entry(cell).or_insert_with(Vec::new).push(idx);
            }
        }
        
        Self { grid, cell_size: 100.0, overlap: 50.0 }
    }
    
    pub fn correlate(&self, address: &AdressClean, zones: &[MiljoeDataClean]) 
        -> Option<(usize, f64)> 
    {
        let cell = point_to_cell(address.coordinates, self.cell_size);
        let neighbors = get_neighboring_cells(cell);  // 9 cells (3x3)
        
        let candidates: Vec<usize> = neighbors
            .iter()
            .filter_map(|c| self.grid.get(c))
            .flatten()
            .copied()
            .collect();
        
        // Only check zones in nearby cells
        candidates.iter()
            .filter_map(|&idx| {
                let dist = point_to_line_distance(
                    address.coordinates,
                    zones[idx].coordinates[0],
                    zones[idx].coordinates[1]
                );
                if dist <= 50.0 { Some((idx, dist)) } else { None }
            })
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    }
}
```

**Characteristics:**
- 2-3x faster than distance-based
- Overlap prevents edge case misses
- Memory overhead: ~180MB for Malmö dataset

## 4. R-Tree Spatial Index

**Implementation:** `core/src/correlation_algorithms/rtree_spatial.rs`

**Strategy:** Use `rstar` crate for bounding-box spatial index

```rust
use rstar::RTree;

pub struct RTreeSpatialAlgo {
    rtree: RTree<LineSegment>,  // Spatial index
}

impl RTreeSpatialAlgo {
    pub fn new(zones: &[MiljoeDataClean]) -> Self {
        let segments: Vec<LineSegment> = zones
            .iter()
            .enumerate()
            .map(|(idx, zone)| LineSegment {
                start: zone.coordinates[0],
                end: zone.coordinates[1],
                index: idx,
            })
            .collect();
        
        Self { rtree: RTree::bulk_load(segments) }
    }
    
    pub fn correlate(&self, address: &AdressClean, zones: &[MiljoeDataClean]) 
        -> Option<(usize, f64)> 
    {
        // Query nearby segments within bounding box
        let nearby = self.rtree.locate_within_distance(
            address.coordinates,
            50.0
        );
        
        nearby
            .filter_map(|seg| {
                let dist = point_to_line_distance(
                    address.coordinates,
                    seg.start,
                    seg.end
                );
                if dist <= 50.0 { Some((seg.index, dist)) } else { None }
            })
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    }
}
```

**Characteristics:**
- Logarithmic query time
- Efficient for general-purpose use
- Standard choice for production

## 5. KD-Tree Spatial Index

**Implementation:** `core/src/correlation_algorithms/kdtree_spatial.rs`

**Strategy:** Use `kiddo` crate for k-dimensional tree (k=2 for lat/lon)

```rust
use kiddo::KdTree;

pub struct KDTreeSpatialAlgo {
    kdtree: KdTree<f64, usize, 2>,  // 2D tree
}
```

**Characteristics:**
- Optimized for point queries
- Slightly less efficient for line segments
- Comparable to R-Tree performance

## 6. Grid Nearest Algorithm

**Implementation:** `core/src/correlation_algorithms/grid_nearest.rs`

**Strategy:** Fixed-size grid without overlap

```rust
pub struct GridNearestAlgo {
    grid: HashMap<(i32, i32), Vec<usize>>,
    cell_size: f64,
}
```

**Characteristics:**
- Simpler than overlapping chunks
- Risk of edge case misses
- Faster pre-processing

## Benchmark Results

Tested on Malmö dataset (10,000 addresses, 2,000 zones):

```
Algorithm            Total Time    Avg/Address    Memory
───────────────────────────────────────────────────────────
Distance-Based       2.45s         4.90ms         100MB
Raycasting          5.12s         10.24ms        105MB
Overlapping Chunks  1.23s         2.46ms         180MB
R-Tree              1.15s         2.30ms         140MB
KD-Tree             1.28s         2.56ms         135MB
Grid                1.31s         2.62ms         150MB
```

**Fastest:** R-Tree (1.15s)

**Recommended:** R-Tree for production, Overlapping Chunks for memory-constrained environments

## Algorithm Selection Guide

```
Dataset Size:
  < 1000 zones      → Distance-Based
  1000-10000 zones  → R-Tree or KD-Tree
  > 10000 zones     → Overlapping Chunks

Constraints:
  Low memory        → Distance-Based
  Low latency       → R-Tree
  High throughput   → Overlapping Chunks

Development:
  Prototyping       → Distance-Based
  Production        → R-Tree
  Research          → Benchmark all
```

## Usage Example

```rust
use amp_core::correlation_algorithms::{RTreeSpatialAlgo, CorrelationAlgo};

let algo = RTreeSpatialAlgo::new(&zones);

for address in addresses {
    if let Some((idx, dist)) = algo.correlate(&address, &zones) {
        println!("{}: {:.2}m from zone {}", address.adress, dist, idx);
    }
}
```

## Related Documentation

- [Architecture](architecture.md) — System design
- [CLI Usage](cli-usage.md) — Benchmark commands
- [Testing](testing.md) — Algorithm validation
- [core/README.md](../core/README.md) — Core library guide
