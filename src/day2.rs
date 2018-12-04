use std::collections::HashMap;

pub fn solve() {
    let input_string = include_str!("day2_input.txt");
    let result_part1 = evaluate_part1(input_string);
    let result_part2 = evaluate_part2(input_string);

    println!("Part 1: {}", result_part1);
    println!("Part 2: {}", result_part2);
}

fn evaluate_part1(input_string: &str) -> i64 {
    let picks = input_string.lines().fold((0, 0), |state, line| {
        let mut occurrences = HashMap::new();

        line.chars()
            .for_each(|c| *occurrences.entry(c).or_insert(0) += 1);

        let has_occurrence = |count| occurrences.iter().any(|(_key, &value)| value == count);

        (
            state.0 + has_occurrence(2) as i64,
            state.1 + has_occurrence(3) as i64,
        )
    });

    picks.0 * picks.1
}

fn evaluate_part2(input_string: &str) -> String {
    let input: Vec<&str> = input_string.lines().collect();

    for i in 0..input.len() {
        for j in i + 1..input.len() {
            let s1 = input[i];
            let s2 = input[j];

            let without_diffs = s1
                .chars()
                .zip(s2.chars())
                .filter(|pair| pair.0 == pair.1)
                .map(|pair| pair.0)
                .collect::<String>();

            if without_diffs.len() == s1.len() - 1 {
                return without_diffs;
            }
        }
    }
    panic!("No solution found");
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
fn tests_part2() {
    assert_eq!(
        "fgij",
        evaluate_part2(
            "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz
"
        )
    );
}
