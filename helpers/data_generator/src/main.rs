use std::fs::write;

use data_types::Point;
use rand::random_range;
use ron::to_string;

fn generate_points(capacity: u128, range: (u128, u128)) -> Vec<Point> {
    if !(range.0 < range.1) {
        panic!("MIN range must be less than MAX range.")
    }

    let actual_capacity = (range.1 - range.0).pow(2);

    if capacity > actual_capacity {
        panic!("Capacity must be in range of possible combinations based on provided range.")
    }

    let mut points: Vec<Point> = vec![];

    while points.len() < capacity as usize {
        let x = random_range((range.0 as u128)..(range.1 as u128));
        let y = random_range((range.0 as u128)..(range.1 as u128));

        let new_point = Point { x, y };

        if !points.iter().any(|point| point.x == x && point.y == y) {
            points.push(new_point);
        }
    }

    points
}

fn main() {
    let capacity = std::env::args()
        .nth(1)
        .expect("Please provide capacity as the first argument")
        .parse::<u128>()
        .expect("Capacity must be a number");

    let min_range = 0;

    let max_range = capacity.pow(2);

    let points = generate_points(capacity, (min_range, max_range));

    let ron_string = to_string(&points).expect("Failed to serialize points to RON");

    let file_path = format!("test_data/{}_points.ron", capacity);

    write(&file_path, ron_string).expect("Failed to write points to file");

    println!("Points have been written to {}", file_path);
}
