#[derive(Debug)]
struct Nanobot {
    x: i64,
    y: i64,
    z: i64,
    strength: i64,
}

impl Nanobot {
    fn parse(input_string: &str) -> Vec<Nanobot> {
        input_string
            .lines()
            .map(|line| {
                let parts: Vec<&str> = line
                    .split(|c| c == '<' || c == '>' || c == ',' || c == '=')
                    .collect();
                //println!("{:?}", parts);
                Nanobot {
                    x: parts[2].parse::<i64>().unwrap(),
                    y: parts[3].parse::<i64>().unwrap(),
                    z: parts[4].parse::<i64>().unwrap(),
                    strength: parts[7].parse::<i64>().unwrap(),
                }
            })
            .collect()
    }

    fn distance_from(&self, other: &Nanobot) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }

    fn is_within_range(&self, other: &Nanobot) -> bool {
        self.distance_from(other) <= self.strength
    }
}
pub fn part1(input_string: &str) -> String {
    let bots = Nanobot::parse(input_string);
    let strongest_bot = bots
        .iter()
        .max_by(|x, y| x.strength.cmp(&y.strength))
        .unwrap();
    bots.iter()
        //.filter(|&bot| strongest_bot as *const _ != bot as *const _ && strongest_bot.is_within_range(bot))
        .filter(|&bot| strongest_bot.is_within_range(bot))
        .count()
        .to_string()
}

pub fn part2(_input_string: &str) -> String {
    "".to_string()
}

#[test]
fn tests_part1() {
    assert_eq!(
        "7",
        part1(
            "pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1"
        )
    );

    assert_eq!("270", part1(include_str!("day23_input.txt")));
}

#[test]
#[ignore]
fn tests_part2() {
    assert_eq!("36", part2("pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5"));

    assert_eq!("_", part2(include_str!("day23_input.txt")));
}
