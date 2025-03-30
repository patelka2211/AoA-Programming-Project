use std::cmp::min;

use data_types::{PairOfPoints, Points};

/// Finds the closest pair of points in a strip and calculates their distance.
///
/// This function is typically used in a divide-and-conquer algorithm for finding
/// the closest pair of points. It takes a slice of points that are within a vertical strip
/// and finds the pair with the minimum distance.
///
/// # Arguments
///
/// * `points_inside_strip` - A slice of points that are inside a vertical strip
///
/// # Returns
///
/// * `Option<(f64, PairOfPoints)>` - If points exist in the strip, returns Some with a tuple containing:
///   - The minimum distance found
///   - The pair of points with that minimum distance
///   If the strip is empty, returns None
///
/// # Time Complexity
///
/// O(n), where n is the number of points in the strip. This is because for each point,
/// we only check at most 15 subsequent points (constant time operation), resulting in linear time overall.
/// This optimization relies on a geometric property that in a sorted strip of width d, a point can have
/// at most 7 points within distance d in each direction, thus we only need to check a constant number
/// of subsequent points.
pub fn find_closest_pair_and_distance_in_strip(
    points_inside_strip: &Points,
) -> Option<(f64, PairOfPoints)> {
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
        Some((min_distance, (point0.clone(), point1.clone())))
    } else {
        None
    }
}
