use std::collections::{BinaryHeap, HashSet};

enum RegionType {
    Rocky,
    Narrow,
    Wet,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
enum Equipment {
    Torch,
    ClimbingGear,
    Neither,
}

fn is_compatible(region_type: RegionType, equipment: Equipment) -> bool {
    // In rocky regions, you can use the climbing gear or the torch. You cannot use neither (you'll likely slip and fall).
    // In wet regions, you can use the climbing gear or neither tool. You cannot use the torch (if it gets wet, you won't have a light source).
    // In narrow regions, you can use the torch or neither tool. You cannot use the climbing gear (it's too bulky to fit).
    match (region_type, equipment) {
        (RegionType::Rocky, Equipment::ClimbingGear) => true,
        (RegionType::Rocky, Equipment::Torch) => true,
        (RegionType::Wet, Equipment::ClimbingGear) => true,
        (RegionType::Wet, Equipment::Neither) => true,
        (RegionType::Narrow, Equipment::Torch) => true,
        (RegionType::Narrow, Equipment::Neither) => true,
        _ => false,
    }
}

fn other_equipment(region_type: RegionType, equipment: Equipment) -> Equipment {
    // In rocky regions, you can use the climbing gear or the torch. You cannot use neither (you'll likely slip and fall).
    // In wet regions, you can use the climbing gear or neither tool. You cannot use the torch (if it gets wet, you won't have a light source).
    // In narrow regions, you can use the torch or neither tool. You cannot use the climbing gear (it's too bulky to fit).
    match (region_type, equipment) {
        (RegionType::Rocky, Equipment::ClimbingGear) => Equipment::Torch,
        (RegionType::Rocky, Equipment::Torch) => Equipment::ClimbingGear,
        (RegionType::Wet, Equipment::ClimbingGear) => Equipment::Neither,
        (RegionType::Wet, Equipment::Neither) => Equipment::ClimbingGear,
        (RegionType::Narrow, Equipment::Torch) => Equipment::Neither,
        (RegionType::Narrow, Equipment::Neither) => Equipment::Torch,
        _ => {
            panic!();
        }
    }
}

struct Grid {
    cells: Vec<i64>, // Storing geological index.
    width: usize,
    height: usize,
    depth: usize,
    target_x: usize,
    target_y: usize,
}

impl Grid {
    fn parse(input_string: &str, padding: usize) -> Grid {
        let lines: Vec<&str> = input_string.lines().collect();
        let depth = lines[0][7..].parse::<usize>().unwrap();

        let parts: Vec<&str> = lines[1][8..].split(',').collect();
        let target_x = parts[0].parse::<usize>().unwrap();
        let target_y = parts[1].parse::<usize>().unwrap();

        let width = target_x + 1 + padding;
        let height = target_y + 1 + padding;

        let cells = vec![-1; width * height];
        Grid {
            cells,
            width,
            height,
            depth,
            target_x,
            target_y,
        }
    }

    fn geological_index(&mut self, x: usize, y: usize) -> usize {
        let index_value = y as usize * self.width + x as usize;
        let cached_value = self.cells[index_value];
        if cached_value != -1 {
            return cached_value as usize;
        }

        let result = if (x == 0 && y == 0) || (x == self.target_x && y == self.target_y) {
            // The region at 0,0 (the mouth of the cave) has a geologic index of 0:
            // The region at the coordinates of the target has a geologic index of 0:
            0
        } else if y == 0 {
            // If the region's Y coordinate is 0, the geologic index is its X coordinate times 16807:
            x * 16807usize
        } else if x == 0 {
            // If the region's X coordinate is 0, the geologic index is its Y coordinate times 48271:
            y * 48271usize
        } else {
            // Otherwise, the region's geologic index is the result of multiplying the erosion levels of the regions at X-1,Y and X,Y-1:
            self.erosion_level(x - 1, y) * self.erosion_level(x, y - 1)
        };

        self.cells[index_value] = result as i64;
        result as usize
    }

    /// A region's erosion level is its geologic index plus the cave system's depth, all modulo 20183.
    fn erosion_level(&mut self, x: usize, y: usize) -> usize {
        (self.geological_index(x, y) + self.depth) % 20183usize
    }

    fn risk_level(&mut self, x: usize, y: usize) -> usize {
        self.erosion_level(x, y) % 3
    }

    fn region_type(&mut self, x: usize, y: usize) -> RegionType {
        match self.risk_level(x, y) {
            0 => RegionType::Rocky,
            1 => RegionType::Wet,
            2 => RegionType::Narrow,
            _ => {
                panic!();
            }
        }
    }
}

pub fn part1(input_string: &str) -> String {
    let mut grid = Grid::parse(input_string, 0);
    let mut sum = 0;
    for y in 0..=grid.target_y {
        for x in 0..=grid.target_x {
            sum += grid.risk_level(x, y);
        }
    }
    sum.to_string()
}

pub fn part2(input_string: &str) -> String {
    let mut grid = Grid::parse(input_string, 100);
    let mut to_visit = BinaryHeap::new();
    let mut visited = HashSet::new();

    // (-cost, x, y, equipment)
    to_visit.push((0i32, 0, 0, Equipment::Torch));

    while let Some(visiting) = to_visit.pop() {
        let cost = -visiting.0;
        let visiting_x = visiting.1 as i32;
        let visiting_y = visiting.2 as i32;
        let equipment = visiting.3;

        if !visited.insert((visiting_x, visiting_y, equipment)) {
            continue;
        }

        if visiting_x == grid.target_x as i32
            && visiting_y == grid.target_y as i32
            && equipment == Equipment::Torch
        {
            return cost.to_string();
        }

        let region_type_visiting = grid.region_type(visiting_x as usize, visiting_y as usize);

        let other_equipment = other_equipment(region_type_visiting, equipment);
        let new_cost = cost + 7;
        to_visit.push((-new_cost, visiting_x, visiting_y, other_equipment));

        for (nx, ny) in [(0, -1i32), (-1i32, 0), (1, 0), (0, 1)].iter() {
            let new_x = (visiting_x as i32 + *nx) as i32;
            let new_y = (visiting_y as i32 + *ny) as i32;
            if new_x < 0 || new_x >= grid.width as i32 || new_y < 0 || new_y >= grid.height as i32 {
                continue;
            }

            let region_type_new = grid.region_type(new_x as usize, new_y as usize);
            if is_compatible(region_type_new, equipment) {
                let new_cost = cost + 1;
                to_visit.push((-new_cost, new_x, new_y, equipment));
            }
        }
    }

    panic!();
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
fn tests_part2() {
    assert_eq!(
        "45",
        part2(
            "depth: 510
target: 10,10"
        )
    );

    assert_eq!("1078", part2(include_str!("day22_input.txt")));
}
