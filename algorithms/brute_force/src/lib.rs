use data_types::{PairOfPoints, Points};

/// Finds the closest pair of points in a given set of points.
///
/// # Arguments
///
/// * `points` - A reference to a collection of points.
///
/// # Returns
///
/// * `Option<PairOfPoints>` - Some((point1, point2)) if there are at least two points,
///   where point1 and point2 are the closest pair of points in the set.
///   None if there are fewer than two points.
///
/// # Complexity
///
/// * Time complexity: O(n²) where n is the number of points.
///   This implementation uses a brute force approach that compares all pairs of points.
pub fn closest_pair(points: &Points) -> Option<PairOfPoints> {
    if points.len() < 2 {
        return None;
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

    closest_pair
}
