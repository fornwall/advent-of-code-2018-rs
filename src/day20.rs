use std::cmp::max;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn part1(input_string: &str) -> String {
    if !(input_string.starts_with('^') && input_string.ends_with('$')) {
        return "Invalid input not prefixed with ^ and ending with $".to_string();
    }
    let input_string = &input_string[1..input_string.len() - 1];

    let mut room_doors = HashMap::new();
    let mut last_positions = Vec::new();
    let mut current_position = (0, 0);
    for char in input_string.chars() {
        match char {
            'N' | 'E' | 'S' | 'W' => {
                room_doors
                    .entry(current_position)
                    .or_insert_with(Vec::new)
                    .push(char);
                match char {
                    'N' => {
                        current_position.1 -= 1;
                    }
                    'E' => {
                        current_position.0 += 1;
                    }
                    'S' => {
                        current_position.1 += 1;
                    }
                    'W' => {
                        current_position.0 -= 1;
                    }
                    _ => {}
                }
            }
            '(' => {
                last_positions.push(current_position);
            }
            '|' => {
                current_position = last_positions.pop().unwrap();
                last_positions.push(current_position)
            }
            ')' => {
                current_position = last_positions.pop().unwrap();
            }
            _ => {
                return format!("Unexpected char: {}", char);
            }
        }
    }

    let mut to_visit = VecDeque::new();
    let mut visited = HashSet::new();
    let mut highest_cost = 0;

    to_visit.push_back((0i32, 0, 0));

    while let Some(visiting) = to_visit.pop_front() {
        highest_cost = max(visiting.0, highest_cost);

        if let Entry::Occupied(doors) = room_doors.entry((visiting.1, visiting.2)) {
            for char in doors.get().iter() {
                let mut adjacent_room = (visiting.1, visiting.2);
                //println!("From {:?}, going {}, cost={}", adjacent_room, char, visiting.0);
                match char {
                    'N' => {
                        adjacent_room.1 -= 1;
                    }
                    'E' => {
                        adjacent_room.0 += 1;
                    }
                    'S' => {
                        adjacent_room.1 += 1;
                    }
                    'W' => {
                        adjacent_room.0 -= 1;
                    }
                    _ => {}
                }
                if visited.insert(adjacent_room) {
                    to_visit.push_back((visiting.0 + 1, adjacent_room.0, adjacent_room.1));
                }
            }
        };
    }

    highest_cost.to_string()
}

pub fn part2(_input_string: &str) -> String {
    "".to_string()
}

#[test]
fn tests_part1() {
    assert_eq!("3", part1("^WNE$"));
    assert_eq!("10", part1("^ENWWW(NEEE|SSE(EE|N))$"));
    assert_eq!("18", part1("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$"));

    assert_eq!("3151", part1(include_str!("day20_input.txt")));
}

#[test]
#[ignore]
fn tests_part2() {
    assert_eq!("", part2(""));

    assert_eq!("", part2(include_str!("day20_input.txt")));
}
