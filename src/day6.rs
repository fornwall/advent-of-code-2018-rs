struct Grid {
    cells: Vec<i32>,
    width: u32,
    height: u32
}

impl Grid {
    fn new(width: u32, height: u32) -> Grid {
        Grid {
            cells: vec![-1; (width*height) as usize ],
            width,
            height
        }
    }

    fn set(&mut self, x: u32, y: u32, value: i32) {
        let index = (y*self.width + x) as usize;
        self.cells[index] = value;
    }
}

pub fn part1(input_string: &str) -> String {
    let points: Vec<(u32, u32)> = input_string.lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(", ").collect();
            (parts[0].parse::<u32>().unwrap(), parts[1].parse::<u32>().unwrap())
        })
        .collect();

    let max_x = *points.iter().map(|(x, _y)| x).min().unwrap();
    let max_y = *points.iter().map(|(_x, y)| y).max().unwrap();
    let grid = Grid::new(max_x, max_y);

    String::from("")
}

pub fn part2(_input_string: &str) -> String {
    String::from("")
}

#[test]
fn tests_part1() {
    assert_eq!("17", part1(
        "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9"));

    //assert_eq!("477", part1(include_str!("day6_input.txt")));
}

#[test]
#[ignore]
fn tests_part2() {
    assert_eq!("0", part2("+1,-1"));

    //assert_eq!("390", part2(include_str!("day6_input.txt")));
}
