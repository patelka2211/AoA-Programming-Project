use std::ops::Range;

use rand::random_range;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
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

    /// Generates a random point within the specified range
    ///
    /// # Arguments
    ///
    /// * `range` - A reference to a PointRange that defines the minimum and maximum values for coordinates
    ///
    /// # Returns
    ///
    /// A new Point instance with randomly generated x and y coordinates, rounded to 2 decimal places
    pub fn generate_random(range: &PointRange) -> Self {
        let mut x = random_range(range.to_native_range());
        let mut y = random_range(range.to_native_range());

        // Round points to 2 decimal places.
        x = Self::round(x, 2);
        y = Self::round(y, 2);

        Point { x, y }
    }
}

pub struct PointRange {
    pub min: f64,
    pub max: f64,
}

impl PointRange {
    /// Creates a new PointRange centered at origin based on capacity
    ///
    /// # Arguments
    ///
    /// * `capacity` - A positive integer value that determines the range's extent
    ///
    /// # Returns
    ///
    /// A new PointRange with min and max values set symmetrically around zero,
    /// where the max value is the ceiling of half the capacity, and min is its negative
    pub fn new_from_capacity(capacity: u32) -> Self {
        let mut max = capacity as f64 / 2.0;
        max = max.ceil();

        PointRange { min: -max, max }
    }

    /// Converts the PointRange to a standard Rust range
    ///
    /// # Arguments
    ///
    /// * `&self` - The reference to the current PointRange
    ///
    /// # Returns
    ///
    /// A Range<f64> representing the range from min to max
    pub fn to_native_range(&self) -> Range<f64> {
        self.min..self.max
    }
}

pub type Points = Vec<Point>;

pub type PairOfPoints = (Point, Point);
