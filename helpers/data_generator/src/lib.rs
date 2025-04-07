use data_types::{Point, Points};

/// Generates a collection of unique random points.
///
/// # Arguments
///
/// * `quantity` - The number of unique points to generate.
///
/// # Returns
///
/// A vector of unique `Point` instances.
pub fn generate_points(quantity: u32) -> Points {
    let mut points: Points = vec![];

    while points.len() < quantity as usize {
        let new_point = Point::generate_random();

        // Check if the new point already exists or not. If not then insert new point.
        if !points.iter().any(|existing_point| {
            existing_point.x == new_point.x && existing_point.y == new_point.y
        }) {
            points.push(new_point);
        }
    }

    points
}
