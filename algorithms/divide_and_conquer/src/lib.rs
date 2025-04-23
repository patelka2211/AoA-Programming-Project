use std::cmp::min;

use data_types::{PairOfPoints, Point, Points};

/// Merges two sorted point arrays into one sorted array according to a comparison function.
///
/// # Arguments
///
/// * `left` - First sorted array of points
/// * `right` - Second sorted array of points
/// * `compare` - Function that defines the sorting order. Returns true if the first point
///               should come before the second point in the sorted result.
///
/// # Returns
///
/// A new sorted array containing all points from both input arrays.
///
/// # Complexity
///
/// * Time Complexity: O(n), where n is the total number of points in both arrays.
fn merge<F>(left: &Points, right: &Points, compare: &F) -> Points
where
    F: Fn(&Point, &Point) -> bool,
{
    let mut result = Vec::new();
    let mut left_index = 0;
    let mut right_index = 0;

    while left_index < left.len() && right_index < right.len() {
        if compare(&left[left_index], &right[right_index]) {
            result.push(left[left_index].clone());
            left_index += 1;
        } else {
            result.push(right[right_index].clone());
            right_index += 1;
        }
    }

    result.extend_from_slice(&left[left_index..]);
    result.extend_from_slice(&right[right_index..]);

    result
}

/// Sorts a vector of points using the merge sort algorithm.
///
/// # Arguments
///
/// * `points` - A vector of Point objects to be sorted
/// * `compare` - A comparison function that defines the sorting order.
///               Returns true if the first point should come before the second point.
///
/// # Returns
///
/// A new sorted vector containing all points from the input vector.
///
/// # Complexity
///
/// * Time Complexity: O(n log n), where n is the number of points.
fn merge_sort_points<F>(points: &Points, compare: &F) -> Points
where
    F: Fn(&Point, &Point) -> bool,
{
    if points.len() <= 1 {
        return points.clone();
    }

    let mid = points.len() / 2;
    let left = merge_sort_points(&points[0..mid].to_vec(), compare);
    let right = merge_sort_points(&points[mid..].to_vec(), compare);

    merge(&left, &right, compare)
}

/// Sorts a vector of points by their x-coordinate in ascending order.
///
/// # Arguments
///
/// * `points` - A vector of Point objects to be sorted
///
/// # Returns
///
/// A new sorted vector containing all points from the input vector,
/// ordered by increasing x-coordinate values.
///
/// # Example
///
/// ```
/// let unsorted_points = vec![Point {x: 3, y: 5}, Point {x: 1, y: 2}, Point {x: 4, y: 1}];
/// let sorted = sort_points_by_x(&unsorted_points);
/// // sorted will contain points ordered by x: [Point {x: 1, y: 2}, Point {x: 3, y: 5}, Point {x: 4, y: 1}]
/// ```
fn sort_points_by_x(points: &Points) -> Points {
    merge_sort_points(points, &|point1, point2| point1.x < point2.x)
}

/// Sorts a vector of points by their y-coordinate in ascending order.
///
/// # Arguments
///
/// * `points` - A vector of Point objects to be sorted
///
/// # Returns
///
/// A new sorted vector containing all points from the input vector,
/// ordered by increasing y-coordinate values.
///
/// # Example
///
/// ```
/// let unsorted_points = vec![Point {x: 3, y: 5}, Point {x: 1, y: 2}, Point {x: 4, y: 1}];
/// let sorted = sort_points_by_y(&unsorted_points);
/// // sorted will contain points ordered by y: [Point {x: 4, y: 1}, Point {x: 1, y: 2}, Point {x: 3, y: 5}]
/// ```
fn sort_points_by_y(points: &Points) -> Points {
    merge_sort_points(points, &|point1, point2| point1.y < point2.y)
}

/// Takes points that are within a vertical strip and finds the pair with the minimum distance.
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
fn find_closest_pair_and_distance_in_strip(
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
fn closest_pair_recursive(sorted_by_x: &Points, sorted_by_y: &Points) -> Option<PairOfPoints> {
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
