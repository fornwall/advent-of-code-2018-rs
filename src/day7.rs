use std::collections::{BTreeSet, HashMap, HashSet};

struct Step {
    name: char,
    dependencies: HashSet<char>,
    needed_by: BTreeSet<char>,
}

impl Step {
    fn new(name: char) -> Step {
        Step {
            name,
            dependencies: HashSet::new(),
            needed_by: BTreeSet::new(),
        }
    }
}

pub fn part1(input_string: &str) -> String {
    let mut step_map = HashMap::new();
    let mut remaining_dependencies: HashMap<char, HashSet<char>> = HashMap::new();

    for line in input_string.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let step_name = parts[7].chars().next().unwrap();
        let depends_on = parts[1].chars().next().unwrap();

        let step = step_map
            .entry(step_name)
            .or_insert_with(|| Step::new(step_name));
        step.dependencies.insert(depends_on);
        remaining_dependencies
            .entry(step_name)
            .or_insert_with(HashSet::new)
            .insert(depends_on);

        step_map
            .entry(depends_on)
            .or_insert_with(|| Step::new(depends_on))
            .needed_by
            .insert(step_name);
    }

    // Topological sorting:
    let mut queue: Vec<&Step> = step_map
        .values()
        .clone()
        .filter(|step| step.dependencies.is_empty())
        .collect();

    let mut visited: HashSet<char> = HashSet::new();
    queue.sort_unstable_by(|a, b| b.name.cmp(&a.name));

    let mut result = String::new();

    while !queue.is_empty() {
        let step = queue.pop().unwrap();

        if visited.contains(&step.name) {
            continue;
        };

        result.push(step.name);
        visited.insert(step.name);

        for needed_by_name in step.needed_by.iter().rev() {
            let v = remaining_dependencies.get_mut(&needed_by_name).unwrap();
            v.remove(&step.name);
            if v.is_empty() {
                queue.push(&step_map[&needed_by_name]);
            };
        }
    }

    result
}

pub fn part2(_input_string: &str) -> String {
    "".to_string()
}

#[test]
fn tests_part1() {
    assert_eq!(
        "CABDFE",
        part1(
            "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin."
        )
    );

    assert_eq!(
        "BCA",
        part1(
            "Step B must be finished before step A can begin.
Step C must be finished before step A can begin."
        )
    );

    assert_eq!(
        "BCA",
        part1(
            "Step C must be finished before step A can begin.
Step B must be finished before step A can begin."
        )
    );

    assert_eq!(
        "OUGTKDLJVBRMIXSAWCYPEQNHZF",
        part1(include_str!("day7_input.txt"))
    );
}

#[test]
#[ignore]
fn tests_part2() {
    assert_eq!("", part2(""));

    //assert_eq!( "aixwcbzrmdvpsjfgllthdyoqe", part2(include_str!("day2_input.txt")) );
}
