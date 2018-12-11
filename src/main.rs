#[cfg(not(target_arch = "wasm32"))]
use std::env;
#[cfg(not(target_arch = "wasm32"))]
use std::io::Read;

use advent_of_code_rs::get_problem_set;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let usage = || println!("Arguments: day part");

    if let Some(day) = env::args().nth(1) {
        if let Some(part) = env::args().nth(2) {
            let solver = get_problem_set(&day, &part);

            let mut input = String::new();
            std::io::stdin()
                .read_to_string(&mut input)
                .expect("Error reading input");

            let solution = solver(input.as_ref());

            println!("{}", solution);
        } else {
            usage();
        }
    } else {
        usage();
    }
}
