mod points;

use std::{
    env::args,
    fs::{create_dir_all, write},
    path::Path,
};

use data_types::PointRange;
use points::generate_points;
use ron::to_string;

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
