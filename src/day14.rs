pub fn part1(input_string: &str) -> String {
    let input_number = input_string.parse::<usize>().unwrap();
    let desired_length = input_number + 10;

    let mut scores = Vec::new();
    scores.push(3_u8);
    scores.push(7_u8);

    let mut elf_positions = Vec::new();
    elf_positions.push(0);
    elf_positions.push(1);

    while scores.len() < desired_length {
        let current_recipes_score = scores[elf_positions[0]] + scores[elf_positions[1]];
        if current_recipes_score < 10 {
            scores.push(current_recipes_score);
        } else {
            scores.push(current_recipes_score / 10);
            scores.push(current_recipes_score % 10);
        }

        for i in 0..elf_positions.len() {
            // "To do this, the Elf steps forward through the scoreboard a number of
            // recipes equal to 1 plus the score of their current recipe."
            elf_positions[i] =
                (elf_positions[i] + 1 + scores[elf_positions[i] as usize] as usize) % scores.len();
        }
    }

    let mut result = String::new();
    for i in 0..10 {
        result.push_str(scores[input_number + i].to_string().as_ref());
    }
    result
}

pub fn part2(_input_string: &str) -> String {
    "".to_string()
}

#[test]
fn tests_part1() {
    assert_eq!("5158916779", part1("9"));
    assert_eq!("0124515891", part1("5"));
    assert_eq!("9251071085", part1("18"));
    assert_eq!("5941429882", part1("2018"));

    assert_eq!("1150511382", part1(include_str!("day14_input.txt")));
}

#[test]
#[ignore]
fn tests_part2() {
    assert_eq!("", part2(""));

    assert_eq!("", part2(include_str!("day14_input.txt")));
}
