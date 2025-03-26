use std::time::Instant;

use data_types::{AlgorithmOutput, Point};

pub fn closest_pair(points: &Vec<Point>) -> AlgorithmOutput {
    if points.len() < 2 {
        return AlgorithmOutput {
            closest_pair: None,
            time_elapsed: 0,
        };
    }

    let start_time = Instant::now();

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

    let stop_time = Instant::now();

    AlgorithmOutput {
        closest_pair,
        time_elapsed: (stop_time - start_time).as_millis(),
    }
}
