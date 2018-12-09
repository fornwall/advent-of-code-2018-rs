use std::collections::VecDeque;

pub fn part1(input_string: &str) -> String {
    let parts: Vec<&str> = input_string.split_whitespace().collect();
    let num_players = parts[0].parse::<usize>().unwrap();
    let num_marbles = parts[6].parse::<usize>().unwrap() + 1; // 0 based.

    let mut player_scores = vec![0; num_players];
    let mut marbles = VecDeque::with_capacity(num_marbles);

    marbles.push_back(0);
    let mut current_index = 0;

    for marble_number in 1..=num_marbles {
        if marble_number % 23 == 0 {
            let player_number = marble_number % num_players;

            // The current player keeps the marble they would have placed, adding it to their score:
            player_scores[player_number] += marble_number;

            // The marble 7 marbles counter-clockwise from the current marble is removed from the
            // circle and also added to the current player's score. The marble located immediately
            // clockwise of the marble that was removed becomes the new current marble:
            if current_index < 7 {
                current_index = marbles.len() - (7 - current_index);
            } else {
                current_index -= 7;
            }
            player_scores[player_number] += marbles.remove(current_index).unwrap();
        } else {
            current_index = (current_index + 2) % marbles.len();
            marbles.insert(current_index, marble_number);
        }
    }

    player_scores.iter().max().unwrap().to_string()
}

pub fn part2(input_string: &str) -> String {
    part1(input_string)
}

#[test]
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

    assert_eq!("423717", part1(include_str!("day9_input.txt")));
}

#[test]
#[ignore]
fn tests_part2() {
    assert_eq!(
        "",
        part2("419 players; last marble is worth 7216400 points")
    );
}
