use std::collections::HashSet;

pub fn solve() {
    let input_string = include_str!("day1_input.txt");
    let result_part1 = evaluate_part1(input_string);
    let result_part2 = evaluate_part2(input_string);
    let result_part2_scan = evaluate_part2_scan(input_string);

    println!("Part 1: {}", result_part1);
    println!("Part 2: {}", result_part2);
    println!("Part 2 (scan): {}", result_part2_scan);
}

fn evaluate_part1(input_string: &str) -> i32 {
    input_string
        .replace(',', "\n")
        .lines()
        .map(|w| w.parse::<i32>().unwrap())
        .sum()
}

fn evaluate_part2(input_string: &str) -> i32 {
    let input: Vec<i32> = input_string
        .replace(',', "\n")
        .lines()
        .map(|w| w.parse::<i32>().unwrap())
        .collect();

    let mut i = 0;
    let mut frequency = 0;
    let mut seen_frequencies = HashSet::new();

    while seen_frequencies.insert(frequency) {
        frequency += input[i];
        i = (i + 1) % input.len();
    }

    frequency
}

fn evaluate_part2_scan(input_string: &str) -> i32 {
    let mut seen_frequencies = HashSet::new();
    seen_frequencies.insert(0);

    input_string
        .replace(',', "\n")
        .lines()
        .map(|w| w.parse::<i32>().unwrap())
        .cycle()
        .scan(0, |frequency, change| {
            *frequency += change;
            Some(*frequency)
        }).find(|frequency| !seen_frequencies.insert(*frequency))
        .unwrap()
}

#[test]
fn tests_part1() {
    assert_eq!(3, evaluate_part1("+1,-2,+3,+1"));
    assert_eq!(3, evaluate_part1("+1,+1,+1"));
    assert_eq!(0, evaluate_part1("+1,+1,-2"));
    assert_eq!(-6, evaluate_part1("-1,-2,-3"));
}

#[test]
fn tests_part2() {
    assert_eq!(0, evaluate_part2("+1,-1"));
    assert_eq!(10, evaluate_part2("+3,+3,+4,-2,-4"));
    assert_eq!(5, evaluate_part2("-6,+3,+8,+5,-6"));
    assert_eq!(14, evaluate_part2("+7,+7,-2,-7,-4"));

    assert_eq!(0, evaluate_part2_scan("+1,-1"));
    assert_eq!(10, evaluate_part2_scan("+3,+3,+4,-2,-4"));
    assert_eq!(5, evaluate_part2_scan("-6,+3,+8,+5,-6"));
    assert_eq!(14, evaluate_part2_scan("+7,+7,-2,-7,-4"));
}
