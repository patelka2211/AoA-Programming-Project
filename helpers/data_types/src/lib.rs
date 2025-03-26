use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Point {
    pub x: u128,
    pub y: u128,
}

impl Point {
    pub fn new(x: u128, y: u128) -> Self {
        Self { x, y }
    }

    pub fn distance(&self, other: &Point) -> f64 {
        if other.x == self.x {
            return (other.y as i128 - self.y as i128).abs() as f64;
        }

        if other.y == self.y {
            return (other.x as i128 - self.x as i128).abs() as f64;
        }

        let x_diff_squared = (other.x - self.x).pow(2);
        let y_diff_squared = (other.y - self.y).pow(2);

        ((x_diff_squared + y_diff_squared) as f64).sqrt()
    }
}

pub type PairOfPoints = (Point, Point);

pub struct AlgorithmOutput {
    pub time_elapsed: u128,
    pub closest_pair: Option<PairOfPoints>,
}
