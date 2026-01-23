use crate::structs::*;
use crate::correlation_algorithms::CorrelationAlgo;
use std::collections::HashMap;
use rust_decimal::prelude::ToPrimitive;

pub struct OverlappingChunksAlgo {
    chunks: HashMap<(i32, i32), Vec<usize>>,
}

const MAX_DISTANCE_METERS: f64 = 50.0;
const CHUNK_SIZE: f64 = 100.0;
const OVERLAP: f64 = 50.0;

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

fn get_chunks(x: f64, y: f64) -> Vec<(i32, i32)> {
    let chunk_x = (x / (CHUNK_SIZE - OVERLAP)) as i32;
    let chunk_y = (y / (CHUNK_SIZE - OVERLAP)) as i32;

    vec![
        (chunk_x, chunk_y),
        (chunk_x + 1, chunk_y),
        (chunk_x, chunk_y + 1),
        (chunk_x + 1, chunk_y + 1),
    ]
}

fn distance_point_to_segment(point: [f64; 2], start: [f64; 2], end: [f64; 2]) -> f64 {
    let point_vec = [point[0] - start[0], point[1] - start[1]];
    let line_vec = [end[0] - start[0], end[1] - start[1]];
    let line_len_sq = line_vec[0] * line_vec[0] + line_vec[1] * line_vec[1];

    if line_len_sq == 0.0 {
        return (point_vec[0] * point_vec[0] + point_vec[1] * point_vec[1]).sqrt();
    }

    let t = ((point_vec[0] * line_vec[0] + point_vec[1] * line_vec[1]) / line_len_sq)
        .clamp(0.0, 1.0);
    let proj = [start[0] + t * line_vec[0], start[1] + t * line_vec[1]];
    let diff = [point[0] - proj[0], point[1] - proj[1]];

    (diff[0] * diff[0] + diff[1] * diff[1]).sqrt()
}

impl OverlappingChunksAlgo {
    pub fn new(zones: &[MiljoeDataClean]) -> Self {
        let mut chunks: HashMap<(i32, i32), Vec<usize>> = HashMap::new();

        for (idx, zone) in zones.iter().enumerate() {
            let x1 = zone.coordinates[0][0].to_f64().unwrap_or(0.0);
            let y1 = zone.coordinates[0][1].to_f64().unwrap_or(0.0);
            let x2 = zone.coordinates[1][0].to_f64().unwrap_or(0.0);
            let y2 = zone.coordinates[1][1].to_f64().unwrap_or(0.0);

            let chunk_set1 = get_chunks(x1, y1);
            let chunk_set2 = get_chunks(x2, y2);

            for chunk in chunk_set1.into_iter().chain(chunk_set2) {
                chunks.entry(chunk).or_default().push(idx);
            }
        }

        OverlappingChunksAlgo { chunks }
    }
}

impl CorrelationAlgo for OverlappingChunksAlgo {
    fn correlate(
        &self,
        address: &AdressClean,
        zones: &[MiljoeDataClean],
    ) -> Option<(usize, f64)> {
        let addr_x = address.coordinates[0].to_f64()?;
        let addr_y = address.coordinates[1].to_f64()?;
        let relevant_chunks = get_chunks(addr_x, addr_y);

        let addr_lat_f64 = address.coordinates[1].to_f64()?;
        let addr_lon_f64 = address.coordinates[0].to_f64()?;
        let (addr_lat, addr_lon) = sweref_to_latlon(addr_lon_f64, addr_lat_f64);

        let mut closest: Option<(usize, f64)> = None;
        let mut checked = std::collections::HashSet::new();

        for chunk in relevant_chunks {
            if let Some(zone_indices) = self.chunks.get(&chunk) {
                for &idx in zone_indices {
                    if checked.insert(idx) {
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
                            if closest.is_none() || min_dist < closest.unwrap().1 {
                                closest = Some((idx, min_dist));
                            }
                        }
                    }
                }
            }
        }

        closest
    }

    fn name(&self) -> &'static str {
        "Overlapping Chunks"
    }
}
