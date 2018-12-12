#[cfg(target_arch = "wasm32")]
extern crate wasm_bindgen;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

mod day1;
mod day10;
mod day11;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub fn get_problem_set(day: u8, part: u8) -> Option<fn(&str) -> String> {
    struct Solutions(fn(&str) -> String, fn(&str) -> String);

    let parts: Solutions = match day {
        1 => Solutions(day1::part1, day1::part2),
        2 => Solutions(day2::part1, day2::part2),
        3 => Solutions(day3::part1, day3::part2),
        4 => Solutions(day4::part1, day4::part2),
        5 => Solutions(day5::part1, day5::part2),
        6 => Solutions(day6::part1, day6::part2),
        7 => Solutions(day7::part1, day7::part2),
        8 => Solutions(day8::part1, day8::part2),
        9 => Solutions(day9::part1, day9::part2),
        10 => Solutions(day10::part1, day10::part2),
        11 => Solutions(day11::part1, day11::part2),
        _ => {
            return None;
        }
    };
    match part {
        1 => Some(parts.0),
        2 => Some(parts.1),
        _ => None,
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn solve(day: u8, part: u8, input: String) -> String {
    if let Some(solver) = get_problem_set(day, part) {
        solver(&input)
    } else {
        format!("Solution for day {} part {} not implemented yet", day, part)
    }
}
