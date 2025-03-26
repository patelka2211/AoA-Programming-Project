use std::{env::args, fs::{create_dir_all, write}, path::Path};

use data_types::{Point, PointRange, Points};
use ron::to_string;

fn generate_points(capacity: u32, range: PointRange) -> Points {
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

fn main() {
    let capacity = args()
        .nth(1)
        .expect("Please provide capacity as the first argument")
        .parse::<u32>()
        .expect("Capacity must be a number");

    let points = generate_points(capacity, PointRange::new_from_capacity(capacity));

    let ron_string = to_string(&points).expect("Failed to serialize points to RON");

    let file_path = format!("test_data/{}_points.ron", capacity);

    // Ensure the directory exists
    if let Some(parent) = Path::new(&file_path).parent() {
        create_dir_all(parent).expect("Failed to create directory");
    }

    write(&file_path, ron_string).expect("Failed to write points to file");

    println!("Points have been written to {}", file_path);
}
