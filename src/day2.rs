pub fn solve() {
    let input_string = include_str!("day2_input.txt");
    let result_part1 = evaluate_part1(input_string);
    let result_part2 = evaluate_part2(input_string);

    println!("Part 1: {}", result_part1);
    println!("Part 2: {}", result_part2);
}

fn line_checksum(line: &[u8]) -> (i64, i64) {
    let mut occurrences = (0i64, 0i64);
    let mut current_char: i16 = -1;
    let mut current_count = 0;

    for c in line.iter() {
        if *c == current_char as u8 {
            current_count += 1;
        } else {
            match current_count {
                2 => occurrences.0 = 1,
                3 => occurrences.1 = 1,
                _ => (),
            }
            current_char = i16::from(*c);
            current_count = 1;
        }
    }

    match current_count {
        2 => occurrences.0 = 1,
        3 => occurrences.1 = 1,
        _ => (),
    }

    occurrences
}

fn evaluate_part1(input_string: &str) -> i64 {
    let occurrences = input_string
        .split_whitespace()
        .map(|line| {
            let mut res = line.as_bytes().to_owned();
            res.sort();
            res
        }).fold((0, 0), |state, line| {
            let line_occurrences = line_checksum(&line);
            (state.0 + line_occurrences.0, state.1 + line_occurrences.1)
        });

    occurrences.0 * occurrences.1
}

fn evaluate_part2(_: &str) -> i32 {
    1
}

#[test]
fn tests_part1() {
    assert_eq!(
        12,
        evaluate_part1(
            "abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab
"
        )
    );
}

#[test]
#[ignore]
fn tests_part2() {
    assert_eq!(0, evaluate_part2(""));
    assert_eq!(0, evaluate_part2(""));
    assert_eq!(0, evaluate_part2(""));
    assert_eq!(0, evaluate_part2(""));
}
