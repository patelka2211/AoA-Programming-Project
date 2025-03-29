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

    let max_test_iterations = 10;
    let max_test_input_scale_factor = 10;

    let mut time_map_for_divide_and_conquer = HashMap::new();
    let mut time_map_for_brute_force = HashMap::new();

    for current_input_scale_factor in 1..(max_test_input_scale_factor + 1) {
        let mut total_time_taken_by_divide_and_conquer = 0;
        let mut total_time_taken_by_brute_force = 0;

        for current_iteration in 1..(max_test_iterations + 1) {
            println!(
                "> Testing iteration {current_iteration} of {current_input_scale_factor}0k points:\n---"
            );

            let points = generate_points(current_input_scale_factor * 10000);

            let mut time_counter = Instant::now();
            divide_and_conquer::closest_pair(&points);
            let time_taken_by_divide_and_conquer = time_counter.elapsed().as_millis();
            println!("Time taken by Divide and Conquer: {time_taken_by_divide_and_conquer} ms.");
            total_time_taken_by_divide_and_conquer += time_taken_by_divide_and_conquer;

            time_counter = Instant::now();
            brute_force::closest_pair(&points);
            let time_taken_by_brute_force = time_counter.elapsed().as_millis();
            println!("Time taken by Brute force: {time_taken_by_brute_force} ms.");
            total_time_taken_by_brute_force += time_taken_by_brute_force;

            println!("");
        }

        time_map_for_divide_and_conquer.insert(
            current_input_scale_factor,
            total_time_taken_by_divide_and_conquer / max_test_iterations,
        );
        time_map_for_brute_force.insert(
            current_input_scale_factor,
            total_time_taken_by_brute_force / max_test_iterations,
        );
    }

    append_to_test_file(
        "Input size (x10k), Average Running Time (Divide and Conquer), Average Running Time (Brute force)"
            .to_string(),
    );

    for test_input in 1..(max_test_input_scale_factor + 1) {
        append_to_test_file(format!(
            "{}, {}, {}",
            test_input,
            time_map_for_divide_and_conquer.get(&test_input).unwrap(),
            time_map_for_brute_force.get(&test_input).unwrap()
        ));
    }
}
