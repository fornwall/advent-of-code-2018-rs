use std::env;
use std::io::Read;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    if let Some(day) = env::args().nth(1) {
        if let Some(part) = env::args().nth(2) {
            let problem_set = get_problem_set(&day);

            let mut input = String::new();
            std::io::stdin()
                .read_to_string(&mut input)
                .expect("Error reading input");

            let solution = match part.as_ref() {
                "1" => (problem_set.0)(input.as_ref()),
                "2" => (problem_set.1)(input.as_ref()),
                _ => {
                    usage();
                    "".to_owned()
                }
            };

            println!("Solution part {}: {}", part, solution);
        } else {
            usage();
        }
    } else {
        usage();
    }
}

fn usage() {
    println!("Arguments: day part [-]")
}

fn get_problem_set(day: &str) -> (fn(&str) -> String, fn(&str) -> String) {
    match day {
        "1" => (day1::part1, day1::part2),
        "2" => (day2::part1, day2::part2),
        "3" => (day3::part1, day3::part2),
        "4" => (day4::part1, day4::part2),
        "5" => (day5::part1, day5::part2),
        _ => {
            println!("No such problem set: {}", day);
            std::process::exit(0);
        }
    }
}
