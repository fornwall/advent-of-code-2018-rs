use std::env;
use std::io::Read;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
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

fn usage() {
    println!("Arguments: day part [-]")
}

fn get_problem_set(day: &str, part: &str) -> fn(&str) -> String {
    struct Solutions(fn(&str) -> String, fn(&str) -> String);

    let parts: Solutions = match day {
        "1" => Solutions(day1::part1, day1::part2),
        "2" => Solutions(day2::part1, day2::part2),
        "3" => Solutions(day3::part1, day3::part2),
        "4" => Solutions(day4::part1, day4::part2),
        "5" => Solutions(day5::part1, day5::part2),
        "6" => Solutions(day6::part1, day6::part2),
        "7" => Solutions(day7::part1, day7::part2),
        "8" => Solutions(day8::part1, day8::part2),
        "9" => Solutions(day9::part1, day9::part2),
        _ => {
            println!("No such day: {}", day);
            std::process::exit(1);
        }
    };
    match part {
        "1" => parts.0,
        "2" => parts.1,
        _ => {
            println!("No such part: {}", part);
            std::process::exit(1);
        }
    }
}
