# Analysis Of Algorithms - Programming Project

## Project Setup and Execution

This project requires the Rust programming language to run. (Rust can be installed from [rustup.rs](https://rustup.rs/).)

If Rust is installed, you can run the command `cargo run --bin algorithm_testing` at the root level of the project. This will compile and run the code inside the `algorithm_testing/src/lib.rs` file, which is the main algorithm testing file.

## Code Organization

The code implementation for brute-force and divide-and-conquer algorithms can be found in the `algorithms/` folder.

### Helper Modules

- **Data Generator**: Located in the `helpers/data_generator` folder, this module contains code to generate vectors of randomly generated unique points. The algorithm takes the number of points to be generated (e.g., 10,000, 20,000, 300,000, etc.).

- **Data Types**: Located in the `helpers/data_types` folder, this module contains various helper functionalities such as types of data used in the project. For example, the Point data type consists of x and y coordinates, and also includes implementations for calculating distance between two distinct points, generating random points, and much more.
