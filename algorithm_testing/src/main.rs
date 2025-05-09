use std::{
    collections::HashMap,
    io::{stdin, stdout, Write},
    time::Instant,
};

use brute_force::closest_pair as finding_closest_pair_using_brute_force;
use divide_and_conquer::closest_pair as finding_closest_pair_using_divide_and_conquer;
use point_utilities::generate_array_of_random_points;
use prettytable::{row, Table};

fn main() {
    let mut input = String::new();

    // read the number of test iterations from command prompt
    print!("\nEnter the number of test iterations: ");
    Write::flush(&mut stdout()).expect("Failed to flush stdout");
    stdin().read_line(&mut input).expect("Failed to read line");

    let total_test_iterations = input
        .trim()
        .parse::<u32>()
        .expect("Please enter a valid number");

    input.clear();

    // read the number of test sizes from command prompt
    print!("\nEnter the number of test sizes (each size will be multiplied by 10k): ");
    Write::flush(&mut stdout()).expect("Failed to flush stdout");
    stdin().read_line(&mut input).expect("Failed to read line");

    let total_test_size = input
        .trim()
        .parse::<u32>()
        .expect("Please enter a valid number");

    let mut time_map_for_divide_and_conquer: HashMap<u32, f64> = HashMap::new();
    let mut time_map_for_brute_force: HashMap<u32, f64> = HashMap::new();

    for test_size in 1..(total_test_size + 1) {
        let mut time_taken_by_divide_and_conquer: u32 = 0;
        let mut time_taken_by_brute_force: u32 = 0;

        for current_iteration in 1..(total_test_iterations + 1) {
            println!("\n> Testing iteration {current_iteration} of {test_size}0k points:\n---");

            let points = generate_array_of_random_points(test_size * 10000);

            // start time counter for divide and conquer algo
            let mut time_counter = Instant::now();
            finding_closest_pair_using_divide_and_conquer(&points);

            // calculate time for divide and conquer algo
            let elapsed_time = time_counter.elapsed().as_millis();
            println!("Time taken by Divide and Conquer: {elapsed_time} ms.");
            time_taken_by_divide_and_conquer += elapsed_time as u32;

            // start time counter for brute force algo
            time_counter = Instant::now();
            finding_closest_pair_using_brute_force(&points);

            // calculate time for brute force algo
            let elapsed_time = time_counter.elapsed().as_millis();
            println!("Time taken by Brute force: {elapsed_time} ms.");
            time_taken_by_brute_force += elapsed_time as u32;
        }

        // take the average time for divide and conquer algo
        time_map_for_divide_and_conquer.insert(
            test_size,
            time_taken_by_divide_and_conquer as f64 / total_test_iterations as f64,
        );

        // take the average time for brute force algo
        time_map_for_brute_force.insert(
            test_size,
            time_taken_by_brute_force as f64 / total_test_iterations as f64,
        );
    }

    // show the final output in table format
    let mut table = Table::new();

    table.add_row(row![
        "Test size",
        format!(
            "Divide and Conquer (Avg. RT for {} test(s)).",
            total_test_iterations
        ),
        format!(
            "Brute force (Avg. RT for {} test(s)).",
            total_test_iterations
        ),
    ]);

    for test_size in 1..(total_test_size + 1) {
        table.add_row(row![
            test_size * 10000,
            format!(
                "{} ms",
                time_map_for_divide_and_conquer.get(&test_size).unwrap()
            ),
            format!("{} ms", time_map_for_brute_force.get(&test_size).unwrap()),
        ]);
    }

    println!("");
    table.printstd();
}
