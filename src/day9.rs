pub fn part1(_input_string: &str) -> String {
    "".to_string()
}

pub fn part2(_input_string: &str) -> String {
    "".to_string()
}

#[test]
#[ignore]
fn tests_part1() {
    assert_eq!("32", part1("9 players; last marble is worth 25 points"));
    assert_eq!(
        "8317",
        part1("10 players; last marble is worth 1618 points")
    );
    assert_eq!(
        "146373",
        part1("13 players; last marble is worth 7999 points")
    );
    assert_eq!(
        "2764",
        part1("17 players; last marble is worth 1104 points")
    );
    assert_eq!(
        "54718",
        part1("21 players; last marble is worth 6111 points")
    );
    assert_eq!(
        "37305",
        part1("30 players; last marble is worth 5807 points")
    );

    //assert_eq!("", part1(include_str!("day8_input.txt")));
}

#[test]
#[ignore]
fn tests_part2() {
    assert_eq!("", part2(""));
    //assert_eq!("", part2(include_str!("day8_input.txt")));
}
