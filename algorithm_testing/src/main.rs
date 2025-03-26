use std::fs::read;

use data_types::Point;

fn main() {
    let file = read("test_data/points.ron").unwrap();
    let file = String::from_utf8(file).unwrap();

    let points = ron::from_str::<Vec<Point>>(&file).unwrap();

    let output = brute_force::closest_pair(&points);

    println!(
        "Brute force: {:?}, time {}",
        output.closest_pair, output.time_elapsed
    );

    let output = divide_and_conquer::closest_pair(&points);

    println!(
        "Divide and Conquer: {:?}, time {}",
        output.closest_pair, output.time_elapsed
    );
}
