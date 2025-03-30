use data_types::{PairOfPoints, Points};

use crate::{
    closest_pair_in_strip::find_closest_pair_and_distance_in_strip, sort_by::sort_points_by_y,
};

/// Implements the divide-and-conquer algorithm to find the closest pair of points in a set.
///
/// # Arguments
///
/// * `sorted_by_x` - A vector of points sorted by their x-coordinates
/// * `sorted_by_y` - A vector of the same points sorted by their y-coordinates
///
/// # Returns
///
/// * `Option<PairOfPoints>` - The closest pair of points if at least two points exist, None otherwise
///
/// # Time Complexity
///
/// O(n log n) where n is the number of points. The algorithm recursively divides the problem
/// into halves and combines the results in linear time, following the recurrence relation:
/// T(n) = 2T(n/2) + O(n), which resolves to O(n log n).
pub fn closest_pair_recursive(sorted_by_x: &Points, sorted_by_y: &Points) -> Option<PairOfPoints> {
    let n = sorted_by_x.len();

    // BASE CASE
    // If there are less than 3 points then simply find the closest pair and return.
    if n <= 3 {
        if let Some((_, closest_pair_strip)) = find_closest_pair_and_distance_in_strip(sorted_by_y)
        {
            return Some(closest_pair_strip);
        }
    }

    // Divide the points into two halves
    let mid = n / 2;

    // Left half of the points sorted by x-coordinate
    let left_half_sorted_by_x = sorted_by_x[0..mid].to_vec();
    // Right half of the points sorted by x-coordinate
    let right_half_sorted_by_x = sorted_by_x[mid..n].to_vec();

    // Left half of the points sorted by y-coordinate
    let left_half_sorted_by_y = sorted_by_y[0..mid].to_vec();
    // Right half of the points sorted by y-coordinate
    let right_half_sorted_by_y = sorted_by_y[mid..n].to_vec();

    // Recursively find the closest pair in each half
    let (q0, q1) = closest_pair_recursive(&left_half_sorted_by_x, &left_half_sorted_by_y).unwrap();
    let (r0, r1) =
        closest_pair_recursive(&right_half_sorted_by_x, &right_half_sorted_by_y).unwrap();

    // Determine the minimum distance from the two halves
    let delta_left_half = q0.distance(&q1);
    let delta_right_half = r0.distance(&r1);

    let (delta_minimum, closest_pair) = if delta_left_half < delta_right_half {
        (delta_left_half, Some((q0, q1)))
    } else {
        (delta_right_half, Some((r0, r1)))
    };

    let max_x_coordinate_in_left_half = sorted_by_x[mid].x;

    // Create a strip of points closer than minimum delta to the midpoint
    let mut strip = Vec::new();
    for point in sorted_by_x {
        if ((point.x as i128 - max_x_coordinate_in_left_half as i128).abs() as f64) < delta_minimum
        {
            strip.push(point.clone());
        }
    }

    // Sort the strip by y-coordinate
    let strip = sort_points_by_y(&strip);

    if let Some((min_distance_of_strip, closest_pair_of_strip)) =
        find_closest_pair_and_distance_in_strip(&strip)
    {
        // Check if a closer pair exists in the strip than in either half.
        // If so, return this pair as our solution.
        if min_distance_of_strip < delta_minimum {
            return Some(closest_pair_of_strip);
        }
    }

    closest_pair
}
