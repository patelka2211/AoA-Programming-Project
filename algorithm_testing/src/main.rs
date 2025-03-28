use std::{
    collections::HashMap,
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

    let test_iteration = 10;
    let number_of_test_inputs = 10;

    let mut time_map_for_brute_force = HashMap::new();
    let mut time_map_for_divide_and_conquer = HashMap::new();

    for iteration in 1..(test_iteration + 1) {
        for test in 1..(number_of_test_inputs + 1) {
            let points = generate_points(test * 10000);

            let mut time_counter = Instant::now();
            divide_and_conquer::closest_pair(&points);
            let time_taken_by_divide_and_conquer = time_counter.elapsed().as_millis();

            time_map_for_divide_and_conquer.insert(
                iteration,
                time_taken_by_divide_and_conquer
                    + (match time_map_for_divide_and_conquer.get(&iteration) {
                        Some(time) => time,
                        None => &0,
                    }),
            );

            time_counter = Instant::now();
            brute_force::closest_pair(&points);
            let time_taken_by_brute_force = time_counter.elapsed().as_millis();

            time_map_for_brute_force.insert(
                iteration,
                time_taken_by_brute_force
                    + (match time_map_for_brute_force.get(&iteration) {
                        Some(time) => time,
                        None => &0,
                    }),
            );
        }
    }

    let mut total_time_taken_by_divide_and_conquer = 0;
    let mut total_time_taken_by_brute_force = 0;

    append_to_test_file(
        "Set, Running Time (Divide and Conquer), Running Time (Brute force)".to_string(),
    );

    for iteration in 1..(test_iteration + 1) {
        let mut record = format!("{}", iteration);

        if let Some(time) = time_map_for_divide_and_conquer.get(&iteration) {
            total_time_taken_by_divide_and_conquer += time;
            record += &format!(", {}", time);
        }

        if let Some(time) = time_map_for_brute_force.get(&iteration) {
            total_time_taken_by_brute_force += time;
            record += &format!(", {}", time);
        }
        append_to_test_file(record);
    }

    append_to_test_file("".to_string());
    append_to_test_file("Average Running Time:".to_string());
    append_to_test_file(format!(
        "Divide and Conquer: {}",
        total_time_taken_by_divide_and_conquer / test_iteration
    ));
    append_to_test_file(format!(
        "Brute force: {}",
        total_time_taken_by_brute_force / test_iteration
    ));
}
