//! R-tree spatial indexing algorithm
//! Uses rstar crate for O(log n) nearest-neighbor queries
//! Best performance for large datasets (1000+ parking zones)

use crate::structs::{AdressClean, MiljoeDataClean};
use crate::correlation_algorithms::CorrelationAlgo;
use rust_decimal::prelude::ToPrimitive;
use rstar::{RTree, AABB, PointDistance};

pub struct RTreeSpatialAlgo {
    rtree: RTree<IndexedLineSegment>,
}

#[derive(Debug, Clone)]
struct IndexedLineSegment {
    index: usize,
    start: [f64; 2],
    end: [f64; 2],
}

impl rstar::RTreeObject for IndexedLineSegment {
    type Envelope = AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        let min_x = self.start[0].min(self.end[0]);
        let min_y = self.start[1].min(self.end[1]);
        let max_x = self.start[0].max(self.end[0]);
        let max_y = self.start[1].max(self.end[1]);
        
        AABB::from_corners([min_x, min_y], [max_x, max_y])
    }
}

impl PointDistance for IndexedLineSegment {
    fn distance_2(&self, point: &[f64; 2]) -> f64 {
        let dist = distance_point_to_line_segment(*point, self.start, self.end);
        dist * dist // Return squared distance for efficiency
    }
}

impl RTreeSpatialAlgo {
    pub fn new(parking_lines: &[MiljoeDataClean]) -> Self {
        let segments: Vec<IndexedLineSegment> = parking_lines
            .iter()
            .enumerate()
            .filter_map(|(idx, line)| {
                let start = [
                    line.coordinates[0][0].to_f64()?,
                    line.coordinates[0][1].to_f64()?,
                ];
                let end = [
                    line.coordinates[1][0].to_f64()?,
                    line.coordinates[1][1].to_f64()?,
                ];
                
                Some(IndexedLineSegment {
                    index: idx,
                    start,
                    end,
                })
            })
            .collect();
        
        Self {
            rtree: RTree::bulk_load(segments),
        }
    }
}

impl CorrelationAlgo for RTreeSpatialAlgo {
    fn correlate(
        &self,
        address: &AdressClean,
        _parking_lines: &[MiljoeDataClean],
    ) -> Option<(usize, f64)> {
        let point = [
            address.coordinates[0].to_f64()?,
            address.coordinates[1].to_f64()?,
        ];
        
        // O(log n) nearest neighbor query
        let nearest = self.rtree.nearest_neighbor(&point)?;
        
        let dist = distance_point_to_line_segment(point, nearest.start, nearest.end);
        
        Some((nearest.index, dist))
    }
    
    fn name(&self) -> &'static str {
        "R-Tree Spatial Index"
    }
}

/// Calculate perpendicular distance from point to line segment
fn distance_point_to_line_segment(point: [f64; 2], line_start: [f64; 2], line_end: [f64; 2]) -> f64 {
    let line_vec = [line_end[0] - line_start[0], line_end[1] - line_start[1]];
    let point_vec = [point[0] - line_start[0], point[1] - line_start[1]];
    
    let line_len_sq = line_vec[0] * line_vec[0] + line_vec[1] * line_vec[1];
    
    if line_len_sq == 0.0 {
        let dx = point[0] - line_start[0];
        let dy = point[1] - line_start[1];
        return (dx * dx + dy * dy).sqrt();
    }
    
    let t = ((point_vec[0] * line_vec[0] + point_vec[1] * line_vec[1]) / line_len_sq)
        .max(0.0)
        .min(1.0);
    
    let closest = [
        line_start[0] + t * line_vec[0],
        line_start[1] + t * line_vec[1],
    ];
    
    let dx = point[0] - closest[0];
    let dy = point[1] - closest[1];
    (dx * dx + dy * dy).sqrt()
}

#[cfg(test)]
mod tests {
    use rstar::RTreeObject;
    use super::*;

    #[test]
    fn test_rtree_envelope() {
        let seg = IndexedLineSegment {
            index: 0,
            start: [0.0, 0.0],
            end: [10.0, 10.0],
        };
        
        let env = seg.envelope();
        assert_eq!(env.lower(), [0.0, 0.0]);
        assert_eq!(env.upper(), [10.0, 10.0]);
    }
    
    #[test]
    fn test_point_distance() {
        let seg = IndexedLineSegment {
            index: 0,
            start: [0.0, 0.0],
            end: [10.0, 0.0],
        };
        
        let dist_sq = seg.distance_2(&[5.0, 3.0]);
        assert!((dist_sq - 9.0).abs() < 0.001); // 3^2 = 9
    }
}
