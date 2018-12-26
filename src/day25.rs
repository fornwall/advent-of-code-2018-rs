pub fn part1(input_string: &str) -> String {
    let mut points: Vec<(i32, i32, i32, i32, usize)> = input_string
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let parts: Vec<&str> = line.split(',').collect();
            (
                parts[0].parse::<i32>().unwrap(),
                parts[1].parse::<i32>().unwrap(),
                parts[2].parse::<i32>().unwrap(),
                parts[3].parse::<i32>().unwrap(),
                i,
            )
        })
        .collect();

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let a = points[i];
            let b = points[j];

            if ((a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs() + (a.3 - b.3).abs()) <= 3
            {
                for p in points.iter_mut().filter(|p| p.4 == b.4) {
                    p.4 = a.4;
                }
            }
        }
    }

    points.sort_by(|a, b| a.4.cmp(&b.4));
    points.dedup_by(|a, b| a.4 == b.4);
    points.len().to_string()
}

pub fn part2(_input_string: &str) -> String {
    "".to_string()
}

#[test]
fn tests_part1() {
    assert_eq!(
        "2",
        part1(
            "0,0,0,0
3,0,0,0
0,3,0,0
0,0,3,0
0,0,0,3
0,0,0,6
9,0,0,0
12,0,0,0"
        )
    );
    assert_eq!(
        "4",
        part1(
            "-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0"
        )
    );
    assert_eq!(
        "3",
        part1(
            "1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2"
        )
    );
    assert_eq!(
        "8",
        part1(
            "1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2"
        )
    );

    assert_eq!("399", part1(include_str!("day25_input.txt")));
}
