use std::fs::read;

use data_types::Points;
use measure_execution_time::measure_execution_time;

fn main() {
    let file = read("test_data/10000_points.ron").unwrap();
    let file = String::from_utf8(file).unwrap();

    let points = ron::from_str::<Points>(&file).unwrap();

    let test = measure_execution_time(brute_force::closest_pair, &points);

    println!("{:?}", test.duration.as_millis());
}
