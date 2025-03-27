use std::time::Duration;

use rand::random_range;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

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

    fn round(coordinate: f64, round_to: i32) -> f64 {
        let power10 = (10 as f64).powi(round_to.abs());

        (coordinate * power10).round() / power10
    }

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
    pub fn new_from_capacity(capacity: u32) -> Self {
        let mut max = capacity as f64 / 2.0;
        max = max.ceil();

        PointRange { min: -max, max }
    }

    pub fn to_native_range(&self) -> std::ops::Range<f64> {
        self.min..self.max
    }
}

pub type Points = Vec<Point>;

pub type PairOfPoints = (Point, Point);

pub struct ClosestPairWithDuration {
    pub duration: Duration,
    pub closest_pair: Option<PairOfPoints>,
}
