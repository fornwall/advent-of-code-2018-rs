use std::collections::HashMap;

pub fn solve() {
    let input_string = include_str!("day4_input.txt");
    let result_part1 = evaluate_part1(input_string);
    let result_part2 = evaluate_part2(input_string);

    println!("Part 1: {}", result_part1);
    println!("Part 2: {}", result_part2);
}

enum EntryType {
    BeginShift { guard_id: u32 },
    FallsAsleep,
    WakesUp,
}

struct LogEntry {
    time: u64,
    minute: u32,
    entry: EntryType,
}

fn parse_input(input_string: &str) -> Vec<LogEntry> {
    let mut entries: Vec<LogEntry> = input_string
        .lines()
        .map(|line| {
            let time = line[1..17] // The part inside brackets in [1518-11-02 00:40].
                .replace("-", "")
                .replace(":", "")
                .replace(" ", "")
                .parse::<u64>()
                .unwrap();

            let parts: Vec<&str> = line.split_whitespace().collect();

            let minute = parts[1][3..5].parse().unwrap();

            let entry = match *parts.last().unwrap() {
                "shift" => EntryType::BeginShift {
                    guard_id: parts[3][1..].parse().expect("Could not parse guard"),
                },
                "asleep" => EntryType::FallsAsleep,
                "up" => EntryType::WakesUp,
                _ => panic!("Invalid line"),
            };

            LogEntry {
                time,
                minute,
                entry,
            }
        }).collect();

    entries.sort_by_key(|entry| entry.time);

    entries
}

fn evaluate_part1(input_string: &str) -> u64 {
    let entries = parse_input(input_string);

    let mut sleepers = HashMap::new();
    let mut current_guard = 0;
    let mut start_minute = 0;

    for log_entry in entries.iter() {
        match log_entry.entry {
            EntryType::BeginShift { guard_id } => current_guard = guard_id,
            EntryType::FallsAsleep => start_minute = log_entry.minute,
            EntryType::WakesUp => {
                let duration = log_entry.minute - start_minute;
                *sleepers.entry(current_guard).or_insert(0) += duration;
            }
        }
    }

    let most_sleepy_guard = *sleepers
        .iter()
        .max_by_key(|(_key, value)| *value)
        .unwrap()
        .0;

    let mut sleep_record = vec![0; 61];
    for log_entry in entries.iter() {
        match log_entry.entry {
            EntryType::BeginShift { guard_id } => current_guard = guard_id,
            EntryType::FallsAsleep => start_minute = log_entry.minute,
            EntryType::WakesUp => {
                if current_guard == most_sleepy_guard {
                    for minute in start_minute..log_entry.minute {
                        sleep_record[minute as usize] += 1;
                    }
                }
            }
        }
    }

    let most_sleepy_minute = sleep_record
        .iter()
        .enumerate()
        .max_by_key(|(_minute, count)| *count)
        .unwrap()
        .0 as u64;

    u64::from(most_sleepy_guard) * most_sleepy_minute
}

fn evaluate_part2(input_string: &str) -> u64 {
    let entries = parse_input(input_string);

    let mut sleepers = HashMap::new();
    let mut current_guard = 0;
    let mut start_minute = 0;

    for log_entry in entries.iter() {
        match log_entry.entry {
            EntryType::BeginShift { guard_id } => current_guard = guard_id,
            EntryType::FallsAsleep => start_minute = log_entry.minute,
            EntryType::WakesUp => {
                let sleep_record = sleepers.entry(current_guard).or_insert_with(|| vec![0; 61]);
                for minute in start_minute..log_entry.minute {
                    sleep_record[minute as usize] += 1;
                }
            }
        }
    }

    let (&guard_id, most_sleepy_minute, _) = sleepers
        .iter()
        .map(|(guard_id, sleep_record)| {
            let (most_sleepy_minute, sleep_count) = sleep_record
                .iter()
                .enumerate()
                .max_by_key(|(_minute, count)| *count)
                .unwrap();
            (guard_id, most_sleepy_minute, sleep_count)
        }).max_by_key(|(_, _, sleep_count)| *sleep_count)
        .unwrap();

    u64::from(guard_id) * most_sleepy_minute as u64
}

#[test]
fn tests_part1() {
    assert_eq!(
        240,
        evaluate_part1(
            "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up"
        )
    );
}

#[test]
fn tests_part2() {
    assert_eq!(
        4455,
        evaluate_part2(
            "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up"
        )
    );
}
