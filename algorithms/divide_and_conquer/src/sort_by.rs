use data_types::{Point, Points};

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
pub fn merge_sort_points<F>(points: &Points, compare: &F) -> Points
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
pub fn sort_points_by_x(points: &Points) -> Points {
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
pub fn sort_points_by_y(points: &Points) -> Points {
    merge_sort_points(points, &|point1, point2| point1.y < point2.y)
}
