use std::collections::HashSet;

pub fn solve() {
    let input_string = include_str!("day1_input.txt");
    let result_part1 = evaluate_part1(input_string);
    let result_part2 = evaluate_part2(input_string);
    println!("Part 1: {}", result_part1);
    println!("Part 2: {}", result_part2);
}

fn parse_word(word: &str) -> i32 {
    let sign = match word.chars().next().unwrap() {
        '+' => 1,
        '-' => -1,
        _ => panic!("Unexpected word: {}", word),
    };
    sign * (word[1..]).parse::<i32>().unwrap()
}

fn evaluate_part1(input_string: &str) -> i32 {
    input_string.split_whitespace().map(parse_word).sum()
}

fn evaluate_part2(input_string: &str) -> i32 {
    let input: Vec<i32> = input_string.split_whitespace().map(parse_word).collect();

    let mut i = 0;
    let mut frequency = 0;
    let mut seen_frequencies = HashSet::new();

    while seen_frequencies.insert(frequency) {
        frequency += input[i];
        i = (i + 1) % input.len();
    }

    frequency
}

#[test]
fn tests_part1() {
    assert_eq!(
        3,
        evaluate_part1(
            "+1
-2
+3
+1"
        )
    );
    assert_eq!(
        3,
        evaluate_part1(
            "+1
+1
+1"
        )
    );
    assert_eq!(
        0,
        evaluate_part1(
            "+1
+1
-2"
        )
    );
    assert_eq!(
        -6,
        evaluate_part1(
            "-1
-2
-3"
        )
    );
}

#[test]
fn tests_part2() {
    assert_eq!(
        0,
        evaluate_part2(
            "+1
-1"
        )
    );
    assert_eq!(
        10,
        evaluate_part2(
            "+3
+3
+4
-2
-4"
        )
    );
    assert_eq!(
        5,
        evaluate_part2(
            "-6
+3
+8
+5
-6"
        )
    );
    assert_eq!(
        14,
        evaluate_part2(
            "+7
+7
-2
-7
-4"
        )
    );
}
