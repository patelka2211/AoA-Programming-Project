use std::cmp::min;

use data_types::{PairOfPoints, Points};

pub fn closest_pair_in_strip(points_inside_strip: &Points) -> (f64, Option<PairOfPoints>) {
    let mut min_distance = f64::MAX;

    let mut closest_pair = None;

    // Traverse from 0th to (n - 1)th element in strip array.
    for outer_index in 0..points_inside_strip.len() {
        let point0 = points_inside_strip.get(outer_index).unwrap();

        // Compute distance from point0 to next 15 points in the strip.
        for inner_index in (outer_index + 1)..min(outer_index + 16, points_inside_strip.len()) {
            let point1 = points_inside_strip.get(inner_index).unwrap();

            let distance = point0.distance(point1);

            if distance < min_distance {
                min_distance = distance;
                closest_pair = Some((point0, point1));
            }
        }
    }

    if let Some((point0, point1)) = closest_pair {
        (min_distance, Some((point0.clone(), point1.clone())))
    } else {
        (min_distance, None)
    }
}
