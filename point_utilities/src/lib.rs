use rand::random_range;

#[derive(Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    /// Creates a new Point with specified coordinates
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate as a 64-bit floating point number
    /// * `y` - The y-coordinate as a 64-bit floating point number
    ///
    /// # Returns
    ///
    /// A new Point instance with the given coordinates
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Calculates the Euclidean distance between two points
    ///
    /// # Arguments
    ///
    /// * `&self` - The reference to the current point
    /// * `other` - A reference to another Point instance to calculate distance to
    ///
    /// # Returns
    ///
    /// A 64-bit floating point number representing the distance between the two points
    pub fn distance(&self, other: &Point) -> f64 {
        if other.x == self.x {
            return (other.y as i128 - self.y as i128).abs() as f64;
        }

        if other.y == self.y {
            return (other.x as i128 - self.x as i128).abs() as f64;
        }

        let x_diff_squared = (other.x - self.x).powi(2);
        let y_diff_squared = (other.y - self.y).powi(2);

        ((x_diff_squared + y_diff_squared) as f64).sqrt()
    }

    /// Rounds a floating-point coordinate to a specified number of decimal places
    ///
    /// # Arguments
    ///
    /// * `coordinate` - The floating-point number to be rounded
    /// * `round_to` - The number of decimal places to round to. Must be a positive integer.
    ///
    /// # Returns
    ///
    /// A rounded floating-point number with the specified precision
    fn round(coordinate: f64, round_to: i32) -> f64 {
        let power10 = (10 as f64).powi(round_to.abs());

        (coordinate * power10).round() / power10
    }

    /// Generates a random floating-point number between 0 and 10^6
    ///
    /// # Returns
    ///
    /// A random 64-bit floating point number within the range [0, 1,000,000)
    fn generate_random_number() -> f64 {
        let range = (0 as f64)..((10 as f64).powi(6));
        random_range(range)
    }

    /// Generates a random point with coordinates between 0 and 10^6
    ///
    /// # Returns
    ///
    /// A new Point instance with randomly generated x and y coordinates, rounded to 2 decimal places
    pub fn generate_random() -> Self {
        let mut x = Self::generate_random_number();
        let mut y = Self::generate_random_number();

        // Round points to 2 decimal places.
        x = Self::round(x, 2);
        y = Self::round(y, 2);

        Point { x, y }
    }
}

pub type Points = Vec<Point>;

pub type PairOfPoints = (Point, Point);

/// Generates a collection of unique random points.
///
/// # Arguments
///
/// * `quantity` - The number of unique points to generate.
///
/// # Returns
///
/// A vector of unique `Point` instances.
pub fn generate_array_of_random_points(quantity: u32) -> Points {
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
