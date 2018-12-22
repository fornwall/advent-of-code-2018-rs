struct Grid {
    cells: Vec<i64>, // Storing geological index.
    width: usize,
    height: usize,
    depth: usize,
}

impl Grid {
    fn parse(input_string: &str) -> Grid {
        let lines: Vec<&str> = input_string.lines().collect();

        let depth = lines[0][7..].parse::<usize>().unwrap();

        let parts: Vec<&str> = lines[1][8..].split(',').collect();
        let width = parts[0].parse::<usize>().unwrap() + 1;
        let height = parts[1].parse::<usize>().unwrap() + 1;
        let cells = vec![-1; width * height];
        println!("depth = {}, width = {}, height = {}", depth, width, height);
        Grid {
            cells,
            width,
            height,
            depth,
        }
    }

    fn geological_index(&mut self, x: usize, y: usize) -> usize {
        let index_value = y as usize * self.width + x as usize;
        let cached_value = self.cells[index_value];
        if cached_value != -1 {
            return cached_value as usize;
        }

        let result = if (x == 0 && y == 0) || (x == (self.width - 1) && y == (self.height - 1)) {
            // The region at 0,0 (the mouth of the cave) has a geologic index of 0:
            // The region at the coordinates of the target has a geologic index of 0:
            0
        } else if y == 0 {
            // If the region's Y coordinate is 0, the geologic index is its X coordinate times 16807:
            x * 16807
        } else if x == 0 {
            // If the region's X coordinate is 0, the geologic index is its Y coordinate times 48271:
            y * 48271
        } else {
            // Otherwise, the region's geologic index is the result of multiplying the erosion levels of the regions at X-1,Y and X,Y-1:
            self.erosion_level(x - 1, y) * self.erosion_level(x, y - 1)
        };

        self.cells[index_value] = result as i64;
        result as usize
    }

    /// A region's erosion level is its geologic index plus the cave system's depth, all modulo 20183.
    fn erosion_level(&mut self, x: usize, y: usize) -> usize {
        // A region's erosion level is its geologic index plus the cave system's depth, all modulo 20183
        (self.geological_index(x, y) + self.depth) % 20183
    }

    fn risk_level(&mut self, x: usize, y: usize) -> usize {
        self.erosion_level(x, y) % 3
    }
}

pub fn part1(input_string: &str) -> String {
    let mut grid = Grid::parse(input_string);
    let mut sum = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            let c = match grid.risk_level(x, y) {
                0 => '.',
                1 => '=',
                2 => '|',
                _ => '?',
            };
            print!("{}", c);
            sum += grid.risk_level(x, y);
        }
        println!();
    }
    sum.to_string()
}

pub fn part2(_input_string: &str) -> String {
    "".to_string()
}

#[test]
fn tests_part1() {
    assert_eq!(
        "114",
        part1(
            "depth: 510
target: 10,10"
        )
    );

    assert_eq!("11843", part1(include_str!("day22_input.txt")));
}

#[test]
#[ignore]
fn tests_part2() {
    assert_eq!("", part2(""));

    assert_eq!("", part2(include_str!("day22_input.txt")));
}
