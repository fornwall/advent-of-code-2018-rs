use std::cmp::max;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn visit_rooms<F>(input_string: &str, mut callback: F)
where
    F: FnMut(i32),
{
    let input_string = &input_string[1..input_string.len() - 1];

    let apply_direction = |direction, position: &mut (i32, i32)| match direction {
        'N' => {
            position.1 -= 1;
        }
        'E' => {
            position.0 += 1;
        }
        'S' => {
            position.1 += 1;
        }
        'W' => {
            position.0 -= 1;
        }
        _ => {}
    };

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
                apply_direction(char, &mut current_position);
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
                panic!("Unexpected char: {}", char);
            }
        }
    }

    let mut visited = HashSet::new();
    let mut to_visit = VecDeque::new();
    to_visit.push_back((0i32, 0, 0));

    while let Some(visiting) = to_visit.pop_front() {
        callback(visiting.0);

        if let Entry::Occupied(doors) = room_doors.entry((visiting.1, visiting.2)) {
            for char in doors.get() {
                let mut adjacent_room = (visiting.1, visiting.2);
                apply_direction(*char, &mut adjacent_room);
                if visited.insert(adjacent_room) {
                    to_visit.push_back((visiting.0 + 1, adjacent_room.0, adjacent_room.1));
                }
            }
        };
    }
}

pub fn part1(input_string: &str) -> String {
    let mut highest_cost = 0;
    visit_rooms(input_string, |cost| {
        highest_cost = max(highest_cost, cost);
    });
    highest_cost.to_string()
}

pub fn part2(input_string: &str) -> String {
    let mut room_count = 0;
    visit_rooms(input_string, |cost| {
        if cost >= 1000 {
            room_count += 1;
        }
    });
    room_count.to_string()
}

#[test]
fn tests_part1() {
    assert_eq!("3", part1("^WNE$"));
    assert_eq!("10", part1("^ENWWW(NEEE|SSE(EE|N))$"));
    assert_eq!("18", part1("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$"));

    assert_eq!("3151", part1(include_str!("day20_input.txt")));
}

#[test]
fn tests_part2() {
    assert_eq!("8784", part2(include_str!("day20_input.txt")));
}
