use crate::structs::*;
use crate::correlation_algorithms::CorrelationAlgo;
use rust_decimal::prelude::ToPrimitive;

pub struct KDTreeSpatialAlgo {
    root: Option<Box<KDNode>>,
}

struct KDNode {
    index: usize,
    point: [f64; 2],
    left: Option<Box<KDNode>>,
    right: Option<Box<KDNode>>,
}

const MAX_DISTANCE_METERS: f64 = 50.0;

fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    const R: f64 = 6371000.0;
    let lat1_rad = lat1.to_radians();
    let lat2_rad = lat2.to_radians();
    let delta_lat = (lat2 - lat1).to_radians();
    let delta_lon = (lon2 - lon1).to_radians();

    let a = (delta_lat / 2.0).sin().powi(2)
        + lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    R * c
}

fn sweref_to_latlon(x: f64, y: f64) -> (f64, f64) {
    let lon = (x - 500000.0) / 111320.0 + 15.0;
    let lat = y / 111320.0 + 55.5;
    (lat, lon)
}

fn build_kdtree(
    zones: &[MiljoeDataClean],
    indices: Vec<usize>,
    depth: usize,
) -> Option<Box<KDNode>> {
    if indices.is_empty() {
        return None;
    }

    let axis = depth % 2;
    let mut sorted_indices = indices;
    sorted_indices.sort_by(|&a, &b| {
        let coord_a = zones[a].coordinates[0][axis].to_f64().unwrap_or(0.0);
        let coord_b = zones[b].coordinates[0][axis].to_f64().unwrap_or(0.0);
        coord_a.partial_cmp(&coord_b).unwrap_or(std::cmp::Ordering::Equal)
    });

    let median = sorted_indices.len() / 2;
    let median_idx = sorted_indices[median];

    let zone = &zones[median_idx];
    let point = [
        zone.coordinates[0][0].to_f64().unwrap_or(0.0),
        zone.coordinates[0][1].to_f64().unwrap_or(0.0),
    ];

    let mut left_indices = sorted_indices;
    let right_indices: Vec<_> = left_indices.drain(median + 1..).collect();
    left_indices.remove(median);

    Some(Box::new(KDNode {
        index: median_idx,
        point,
        left: build_kdtree(zones, left_indices, depth + 1),
        right: build_kdtree(zones, right_indices, depth + 1),
    }))
}

impl KDNode {
    fn query_nearest(
        &self,
        point: [f64; 2],
        _lines: &[MiljoeDataClean],
        best: &mut Option<(usize, f64)>,
    ) {
        let dx = self.point[0] - point[0];
        let dy = self.point[1] - point[1];
        let dist_sq = dx * dx + dy * dy;
        let dist = dist_sq.sqrt();

        if best.is_none() || dist < best.unwrap().1 {
            *best = Some((self.index, dist));
        }

        let axis = (self.index & 1) as usize;
        let axis_dist = if axis == 0 { dx } else { dy };

        let (near, far) = if axis_dist < 0.0 {
            (&self.left, &self.right)
        } else {
            (&self.right, &self.left)
        };

        if let Some(node) = near {
            node.query_nearest(point, _lines, best);
        }

        let should_search_secondary = best.is_none() || axis_dist.abs() < best.unwrap().1;
        if should_search_secondary && let Some(node) = far {
            node.query_nearest(point, _lines, best);
        }
    }
}

impl KDTreeSpatialAlgo {
    pub fn new(zones: &[MiljoeDataClean]) -> Self {
        let indices: Vec<_> = (0..zones.len()).collect();
        let root = build_kdtree(zones, indices, 0);
        KDTreeSpatialAlgo { root }
    }
}

impl CorrelationAlgo for KDTreeSpatialAlgo {
    fn correlate(
        &self,
        address: &AdressClean,
        zones: &[MiljoeDataClean],
    ) -> Option<(usize, f64)> {
        let addr_lat_f64 = address.coordinates[1].to_f64()?;
        let addr_lon_f64 = address.coordinates[0].to_f64()?;
        let (addr_lat, addr_lon) = sweref_to_latlon(addr_lon_f64, addr_lat_f64);

        let mut best: Option<(usize, f64)> = None;

        if let Some(ref root) = self.root {
            root.query_nearest([addr_lon, addr_lat], zones, &mut best);
        }

        if let Some((idx, _)) = best {
            let zone = &zones[idx];
            let start_f64 = [
                zone.coordinates[0][0].to_f64()?,
                zone.coordinates[0][1].to_f64()?,
            ];
            let end_f64 = [
                zone.coordinates[1][0].to_f64()?,
                zone.coordinates[1][1].to_f64()?,
            ];

            let (start_lat, start_lon) = sweref_to_latlon(start_f64[0], start_f64[1]);
            let (end_lat, end_lon) = sweref_to_latlon(end_f64[0], end_f64[1]);

            let dist_to_start = haversine_distance(addr_lat, addr_lon, start_lat, start_lon);
            let dist_to_end = haversine_distance(addr_lat, addr_lon, end_lat, end_lon);
            let min_dist = dist_to_start.min(dist_to_end);

            if min_dist <= MAX_DISTANCE_METERS {
                return Some((idx, min_dist));
            }
        }

        None
    }

    fn name(&self) -> &'static str {
        "KD-Tree Spatial Index"
    }
}
