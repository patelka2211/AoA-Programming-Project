use data_types::{Point, PointRange, Points};

/// Generates a collection of unique random points.
///
/// # Arguments
///
/// * `capacity` - The number of unique points to generate.
///
/// # Returns
///
/// A vector of unique `Point` instances within a range determined by the capacity.
///
/// # Panics
///
/// Panics if the minimum range value is not less than the maximum range value.
pub fn generate_points(capacity: u32) -> Points {
    let range = PointRange::new_from_capacity(capacity);

    if !(range.min < range.max) {
        panic!("MIN range must be less than MAX range.")
    }

    let mut points: Points = vec![];

    while points.len() < capacity as usize {
        let new_point = Point::generate_random(&range);

        // Check if the new point already exists or not. If not then insert new point.
        if !points.iter().any(|existing_point| {
            existing_point.x == new_point.x && existing_point.y == new_point.y
        }) {
            points.push(new_point);
        }
    }

    points
}
