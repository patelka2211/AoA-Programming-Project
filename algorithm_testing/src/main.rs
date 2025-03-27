use std::{
    fs::{read_to_string, write},
    time::Instant,
};

use data_generator::generate_points;

fn init_test_file() {
    write("test_result.txt", "").expect("Failed to write to test_result.txt");
}

fn append_to_test_file(content: String) {
    let existing_content = read_to_string("test_result.txt").unwrap_or_default();
    let updated_content = existing_content + &content + "\n";
    write("test_result.txt", updated_content).expect("Failed to write to test_result.txt");
}

fn main() {
    init_test_file();

    for set in 1..11 {
        let mut total_time_taken_by_brute_force = 0;
        let mut total_time_taken_by_divide_and_conquer = 0;

        for test in 1..11 {
            let points = generate_points(test * 10000);

            let mut time_counter = Instant::now();
            divide_and_conquer::closest_pair(&points);
            total_time_taken_by_divide_and_conquer += time_counter.elapsed().as_millis();

            time_counter = Instant::now();
            brute_force::closest_pair(&points);
            total_time_taken_by_brute_force += time_counter.elapsed().as_millis();
        }

        append_to_test_file(format!("Test Set: {}", set));
        append_to_test_file(format!(
            "Empirical running time of Divide and Conquer: {}",
            total_time_taken_by_divide_and_conquer / 10
        ));
        append_to_test_file(format!(
            "Empirical running time of Brute force: {}",
            total_time_taken_by_brute_force / 10
        ));
        append_to_test_file("".to_string());
    }
}
