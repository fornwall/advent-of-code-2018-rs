use std::env;
use std::io::Read;

mod day1;
mod day2;
mod day3;
mod day4;

pub struct ProblemSet {
    default_input: &'static str,
    part1: fn(&str) -> String,
    part2: fn(&str) -> String,
}

fn main() {
    if let Some(day) = env::args().nth(1) {
        if let Some(part) = env::args().nth(2) {
            let problem_set = get_problem_set(&day);

            let mut buffer = String::new();
            let input = if env::args().nth(3).is_some() {
                std::io::stdin()
                    .read_to_string(&mut buffer)
                    .expect("Error reading input");
                buffer.as_ref()
            } else {
                problem_set.default_input
            };

            let solution = match part.as_ref() {
                "1" => (problem_set.part1)(input),
                "2" => (problem_set.part2)(input),
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

fn get_problem_set(day: &str) -> ProblemSet {
    match day {
        "1" => ProblemSet {
            default_input: include_str!("day1_input.txt"),
            part1: day1::part1,
            part2: day1::part2,
        },
        "2" => ProblemSet {
            default_input: include_str!("day2_input.txt"),
            part1: day2::part1,
            part2: day2::part2,
        },
        "3" => ProblemSet {
            default_input: include_str!("day3_input.txt"),
            part1: day3::part1,
            part2: day3::part2,
        },
        "4" => ProblemSet {
            default_input: include_str!("day4_input.txt"),
            part1: day4::part1,
            part2: day4::part2,
        },
        //"5" => ProblemSet { default_input: include_str!("day5_input.txt"), part1: day5::part1, part2: day5::part2 },
        _ => {
            println!("No such problem set: {}", day);
            std::process::exit(0);
        }
    }
}
