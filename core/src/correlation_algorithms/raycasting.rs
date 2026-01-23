use crate::structs::*;
use crate::correlation_algorithms::CorrelationAlgo;

pub struct RaycastingAlgo;

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

fn line_intersection(p1: [f64; 2], p2: [f64; 2], p3: [f64; 2], p4: [f64; 2]) -> Option<[f64; 2]> {
    let x1 = p1[0];
    let y1 = p1[1];
    let x2 = p2[0];
    let y2 = p2[1];
    let x3 = p3[0];
    let y3 = p3[1];
    let x4 = p4[0];
    let y4 = p4[1];

    let denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

    if denom.abs() < 1e-10 {
        return None;
    }

    let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / denom;
    let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / denom;

    if (0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u) {
        let x = x1 + t * (x2 - x1);
        let y = y1 + t * (y2 - y1);
        Some([x, y])
    } else {
        None
    }
}

impl CorrelationAlgo for RaycastingAlgo {
    fn correlate(
        &self,
        address: &AdressClean,
        zones: &[MiljoeDataClean],
    ) -> Option<(usize, f64)> {
        let addr_lat_f64 = address.coordinates[1].to_f64()?;
        let addr_lon_f64 = address.coordinates[0].to_f64()?;
        let (addr_lat, addr_lon) = sweref_to_latlon(addr_lon_f64, addr_lat_f64);

        let mut closest: Option<(usize, f64)> = None;

        for (idx, zone) in zones.iter().enumerate() {
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

            // Cast 36 rays from the address point
            let mut min_dist = f64::INFINITY;
            for ray_idx in 0..36 {
                let angle = (ray_idx as f64) * std::f64::consts::TAU / 36.0;
                let ray_end = [
                    addr_lon + angle.cos() * 0.01,
                    addr_lat + angle.sin() * 0.01,
                ];

                if let Some(_intersection) = line_intersection(
                    [addr_lon, addr_lat],
                    ray_end,
                    [start_lon, start_lat],
                    [end_lon, end_lat],
                ) {
                    let dist_to_start = haversine_distance(addr_lat, addr_lon, start_lat, start_lon);
                    let dist_to_end = haversine_distance(addr_lat, addr_lon, end_lat, end_lon);
                    let min_zone_dist = dist_to_start.min(dist_to_end);
                    min_dist = min_dist.min(min_zone_dist);
                }
            }

            if min_dist <= MAX_DISTANCE_METERS {
                if closest.is_none() || min_dist < closest.unwrap().1 {
                    closest = Some((idx, min_dist));
                }
            }
        }

        closest
    }
}
