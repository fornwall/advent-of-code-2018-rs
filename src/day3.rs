pub fn solve() {
    let input_string = include_str!("day3_input.txt");
    let result_part1 = evaluate_part1(input_string);
    let result_part2 = evaluate_part2(input_string);

    println!("Part 1: {}", result_part1);
    println!("Part 2: {}", result_part2);
}

struct Fabric {
    num_claims: Vec<u32>,
}
struct Claim {
    id: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl Fabric {
    fn new() -> Fabric {
        Fabric {
            num_claims: vec![0; 1_000_000],
        }
    }
    fn add_claim(&mut self, claim: &Claim) {
        for y in claim.y..claim.y + claim.height {
            let row_offset = y * 1000;
            for x in claim.x..claim.x + claim.width {
                self.num_claims[(row_offset + x) as usize] += 1u32;
            }
        }
    }
    fn inches_claimed_multiple(&self) -> usize {
        self.num_claims.iter().filter(|&&c| c > 1).count()
    }
    fn is_claimed_once(&self, claim: &Claim) -> bool {
        let mut result = true;
        for y in claim.y..claim.y + claim.height {
            let row_offset = y * 1000;
            for x in claim.x..claim.x + claim.width {
                if self.num_claims[(row_offset + x) as usize] > 1 {
                    result = false;
                }
            }
        }
        result
    }
}

fn evaluate_part1(input_string: &str) -> usize {
    let mut fabric = Fabric::new();

    input_string
        .lines()
        .map(|line| {
            let parts: Vec<u32> = line
                .replace("#", "")
                .replace("@", "")
                .replace(",", " ")
                .replace(":", "")
                .replace("x", " ")
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect();
            Claim {
                id: parts[0],
                x: parts[1],
                y: parts[2],
                width: parts[3],
                height: parts[4],
            }
        }).for_each(|claim| fabric.add_claim(&claim));

    fabric.inches_claimed_multiple()
}

fn evaluate_part2(input_string: &str) -> u32 {
    let mut fabric = Fabric::new();

    let claims: Vec<Claim> = input_string
        .lines()
        .map(|line| {
            let parts: Vec<u32> = line
                .replace("#", "")
                .replace("@", "")
                .replace(",", " ")
                .replace(":", "")
                .replace("x", " ")
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect();

            Claim {
                id: parts[0],
                x: parts[1],
                y: parts[2],
                width: parts[3],
                height: parts[4],
            }
        }).collect();

    claims.iter().for_each(|claim| fabric.add_claim(claim));

    claims
        .iter()
        .find(|claim| fabric.is_claimed_once(claim))
        .expect("No result found")
        .id
}

#[test]
fn tests_part1() {
    assert_eq!(
        4,
        evaluate_part1(
            "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2"
        )
    );
}

#[test]
fn tests_part2() {
    assert_eq!(
        3,
        evaluate_part2(
            "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2"
        )
    );
}
