use std::time::Instant;

use data_types::{ClosestPairWithDuration, Points};

pub fn closest_pair(points: &Points) -> ClosestPairWithDuration {
    let start_time = Instant::now();

    if points.len() < 2 {
        return ClosestPairWithDuration {
            duration: start_time.elapsed(),
            closest_pair: None,
        };
    }

    let mut min_distance = f64::MAX;
    let mut closest_pair = None;

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let distance = points[i].distance(&points[j]);
            if distance < min_distance {
                min_distance = distance;
                closest_pair = Some((points[i].clone(), points[j].clone()));
            }
        }
    }

    ClosestPairWithDuration {
        duration: start_time.elapsed(),
        closest_pair,
    }
}
