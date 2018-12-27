use std::cmp;
use std::collections::{HashMap, HashSet};

struct Point {
    id: i32,
    x: i32,
    y: i32,
}

pub fn part1(input_string: &str) -> String {
    let mut counter = 0;
    let points: Vec<Point> = input_string
        .lines()
        .map(|line| {
            counter += 1;
            let parts: Vec<&str> = line.split(", ").collect();
            Point {
                id: counter,
                x: parts[0].parse::<i32>().unwrap(),
                y: parts[1].parse::<i32>().unwrap(),
            }
        })
        .collect();

    let (left, top, right, bottom) = points.iter().fold(
        (std::i32::MAX, std::i32::MAX, std::i32::MIN, std::i32::MIN),
        |(left, top, right, bottom), point| {
            (
                cmp::min(left, point.x),
                cmp::min(top, point.y),
                cmp::max(right, point.x),
                cmp::max(bottom, point.y),
            )
        },
    );

    let mut id_to_count = HashMap::new();
    let mut points_at_edges = HashSet::new();

    for y in top..=bottom {
        for x in left..=right {
            let mut closest_distance = std::i32::MAX;
            let mut closest_point_id = -1;

            for point in points.iter() {
                let distance = (x - point.x).abs() + (y - point.y).abs();

                if distance < closest_distance {
                    closest_distance = distance;
                    closest_point_id = point.id;
                } else if distance == closest_distance {
                    closest_point_id = -1;
                };
            }

            if x == left || x == right || y == top || y == bottom {
                points_at_edges.insert(closest_point_id);
            }

            *id_to_count.entry(closest_point_id).or_insert(0) += 1;
        }
    }

    id_to_count
        .iter()
        .max_by_key(|(&key, &value)| {
            if points_at_edges.contains(&key) {
                -1
            } else {
                value
            }
        })
        .unwrap()
        .1
        .to_string()
}

pub fn part2_param(input_string: &str, max_distance_exclusive: i32) -> i32 {
    let mut counter = 0;
    let points: Vec<Point> = input_string
        .lines()
        .map(|line| {
            counter += 1;
            let parts: Vec<&str> = line.split(", ").collect();
            Point {
                id: counter,
                x: parts[0].parse::<i32>().unwrap(),
                y: parts[1].parse::<i32>().unwrap(),
            }
        })
        .collect();

    let (left, top, right, bottom) = points.iter().fold(
        (std::i32::MAX, std::i32::MAX, std::i32::MIN, std::i32::MIN),
        |(left, top, right, bottom), point| {
            (
                cmp::min(left, point.x),
                cmp::min(top, point.y),
                cmp::max(right, point.x),
                cmp::max(bottom, point.y),
            )
        },
    );

    let mut sum: i32 = 0;

    for y in top..=bottom {
        for x in left..=right {
            let total_distance = points.iter().fold(0, |acc, point| {
                acc + (x - point.x).abs() + (y - point.y).abs()
            });
            if total_distance < max_distance_exclusive {
                sum += 1;
            }
        }
    }

    sum
}

pub fn part2(input_string: &str) -> String {
    part2_param(input_string, 10000).to_string()
}

#[test]
fn tests_part1() {
    assert_eq!(
        "17",
        part1(
            "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9"
        )
    );

    assert_eq!(
        "1876",
        part1(
            "0, 0
0, 100
1, 50
80, 20
80, 50
80, 80
100, 0
100, 50
100, 100"
        )
    );

    assert_eq!("5333", part1(include_str!("day6_input.txt")));
}

#[test]
fn tests_part2() {
    assert_eq!(
        16,
        part2_param(
            "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9",
            32
        )
    );

    assert_eq!("35334", part2(include_str!("day6_input.txt")));
}
