use std::{
    fs::{read, write},
    time::Instant,
};

use data_types::Points;
use ron::from_str;

fn init_test_file() {
    write("test_result.txt", "").expect("Failed to write to test_result.txt");
}

fn append_to_test_file(content: String) {
    let existing_content = std::fs::read_to_string("test_result.txt").unwrap_or_default();
    let updated_content = existing_content + &content + "\n";
    write("test_result.txt", updated_content).expect("Failed to write to test_result.txt");
}

macro_rules! test_writer {
    ($arg1: expr, $arg2: ident) => {
        append_to_test_file(format!("Algorithm: {}", $arg1).to_string());
        append_to_test_file("time(msec), size(x10,000)".to_string());

        for i in 1..(10 + 1) {
            let file = read(format!("test_data/{}0000_points.ron", i)).unwrap();
            let file = String::from_utf8(file).unwrap();

            let points = from_str::<Points>(&file).unwrap();

            let test = $arg2::closest_pair(&points);

            append_to_test_file(format!("{}, {}", test.duration.as_millis(), i).to_string());
        }

        append_to_test_file("".to_string());
    };
}

fn main() {
    init_test_file();

    let start_time = Instant::now();

    test_writer!("Divide and Conquer", divide_and_conquer);
    test_writer!("Brute force", brute_force);

    append_to_test_file(format!(
        "Whole test took {} second(s) to complete.",
        start_time.elapsed().as_secs()
    ));
}
