mod closest_pair_in_strip;
mod recursive;
mod sort_by;

use data_types::{PairOfPoints, Points};
use recursive::closest_pair_recursive;
use sort_by::{sort_points_by_x, sort_points_by_y};

/// Finds the closest pair of points in a set of points.
///
/// # Arguments
///
/// * `points` - A reference to a collection of points.
///
/// # Returns
///
/// * `Option<PairOfPoints>` - The closest pair of points if the collection contains at least 2 points,
///                            or None if there are fewer than 2 points.
///
/// # Time Complexity
///
/// O(n log n) where n is the number of points. This is achieved through a divide-and-conquer
/// approach that sorts points by coordinates once (O(n log n)) and then recursively finds
/// the closest pair in subproblems.
pub fn closest_pair(points: &Points) -> Option<PairOfPoints> {
    // Sort points by x-coordinate
    let px = sort_points_by_x(points);

    // Sort points by y-coordinate
    let py = sort_points_by_y(points);

    let closest_pair = closest_pair_recursive(&px, &py);

    closest_pair
}
