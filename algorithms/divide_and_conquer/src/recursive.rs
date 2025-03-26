use data_types::{PairOfPoints, Points};

use crate::{sort_by::sort_points_by_y, strip::closest_pair_in_strip};

pub fn closest_pair_recursive(px: &Points, py: &Points) -> Option<PairOfPoints> {
    let n = px.len();

    // BASE CASE
    // If there are less than 3 points then simply find the closest pair and return.
    if n <= 3 {
        let (_, closest_pair_strip) = closest_pair_in_strip(py);
        return closest_pair_strip;
    }

    // Divide the points into two halves
    let mid = n / 2;

    // Left half of the points sorted by x-coordinate
    let qx = px[0..mid].to_vec();
    // Right half of the points sorted by x-coordinate
    let rx = px[mid..n].to_vec();

    // Left half of the points sorted by y-coordinate
    let qy = py[0..mid].to_vec();
    // Right half of the points sorted by y-coordinate
    let ry = py[mid..n].to_vec();

    // Recursively find the closest pair in each half
    let (q0, q1) = closest_pair_recursive(&qx, &qy).unwrap();
    let (r0, r1) = closest_pair_recursive(&rx, &ry).unwrap();

    // Determine the minimum distance from the two halves
    let delta_left = q0.distance(&q1);
    let delta_right = r0.distance(&r1);

    let (delta_minimum, mut closest_pair) = if delta_left < delta_right {
        (delta_left, Some((q0, q1)))
    } else {
        (delta_right, Some((r0, r1)))
    };

    let l = px[mid].x;

    // Create a strip of points closer than minimum delta to the midpoint
    let mut strip = Vec::new();
    for p in px {
        if ((p.x as i128 - l as i128).abs() as f64) < delta_minimum {
            strip.push(p.clone());
        }
    }

    // Sort the strip by y-coordinate
    let strip = sort_points_by_y(&strip);

    let (min_distance_strip, closest_pair_strip) = closest_pair_in_strip(&strip);

    if min_distance_strip < delta_minimum {
        closest_pair = closest_pair_strip;
    }

    closest_pair
}
