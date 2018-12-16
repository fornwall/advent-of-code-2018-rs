use std::collections::{BinaryHeap, HashSet};

#[derive(Debug, Copy, Clone)]
enum MapCell {
    Wall,
    Open,
    Unit{ hit_points: i32, elf: bool, even: bool },
}

struct Board {
    width: u32,
    height: u32,
    cells: Vec<MapCell>,
    round: i32,
    full_round: bool,
    elves_alive: i32,
    goblins_alive: i32
}

impl Board {
    fn parse(input_string: &str) -> Board {
        let width = match input_string.find("\n") {
            Some(len) => len as u32,
            None => {
                panic!("No line found");
            }
        };


        let mut elves_alive = 0;
        let mut goblins_alive = 0;
        let mut cells = Vec::new();
        let mut height = 0;
        for line in input_string.lines() {
            height += 1;
            assert_eq!(width, line.len() as u32);
            for c in line.chars() {
                cells.push(match c {
                    '#' => MapCell::Wall,
                    '.' => MapCell::Open,
                    'G' => {
                        goblins_alive += 1;
                        MapCell::Unit {hit_points:200, elf: false, even:false}
                    },
                    'E' => {
                        elves_alive += 1;
                        MapCell::Unit {hit_points:200, elf: true, even:false}
                    },
                    _ => {
                        panic!("Unrecognized cell: {}", c);
                    }
                });
            }
        }

        Board {
            width,
            height,
            cells,
            round: 0,
            full_round:false,
            elves_alive,
            goblins_alive
        }
    }

    fn at(&mut self, x: u32, y: u32) -> &mut MapCell {
        &mut self.cells[(x + self.width * y) as usize]
    }

    fn put(&mut self, x: u32, y: u32, value: MapCell) {
        self.cells[(x + self.width * y) as usize] = value;
    }

    fn calculate_outcome(&self) -> Option<i32> {
        if self.round > 200 {
            return Some(1337);
        }
        let mut elf_alive = false;
        let mut goblin_alive = false;
        let mut hit_point_sum = 0;

        for cell in self.cells.iter() {
            if let MapCell::Unit { hit_points, elf, .. } = *cell {
                hit_point_sum += hit_points;
                if elf {
                    elf_alive = true;
                } else {
                    goblin_alive = true;
                }
            }
        }
        if goblin_alive && elf_alive {
            return None;
        } else {
            let round_for_score = self.round - if self.full_round {
                0
            } else {
                1
            };
            return Some(hit_point_sum * round_for_score);
        }
    }

    fn perform_round(&mut self) {
        self.round += 1;
        self.full_round = true;
        let even_round = self.round % 2 == 0;

        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.at(x, y);
                match cell {
                    &mut MapCell::Open => {},
                    &mut MapCell::Wall => {},
                    &mut MapCell::Unit{even, elf , ..} => {
                        if even == even_round {
                            self.attack_or_move_towards(x, y, !elf);
                        }
                    },
                }
            }
        }
    }

    fn attack(&mut self, x: u32, y: u32, elf_target: bool) -> bool {
        let mut lowest_hit_points = std::i32::MAX;
        let mut target_position = (0, 0);

        for (dx, dy) in [(0,-1i32), (-1i32, 0), (1,0), (0,1)].iter() {
            let target_x = x as i32 + *dx;
            let target_y = y as i32 + *dy;
            if let MapCell::Unit { hit_points, elf, .. } = self.at(target_x as u32, target_y as u32) {
                if *elf == elf_target {
                    if *hit_points < lowest_hit_points {
                        lowest_hit_points = *hit_points;
                        target_position = (target_x, target_y);
                    }
                }
            }
        }

        if lowest_hit_points != std::i32::MAX {
            if let MapCell::Unit { hit_points, .. } = self.at(target_position.0 as u32, target_position.1 as u32) {
                *hit_points -= 3;
                if *hit_points <= 0 {
                    self.put(target_position.0 as u32, target_position.1 as u32, MapCell::Open);
                    if elf_target {
                        self.elves_alive -= 1;
                    } else {
                        self.goblins_alive -= 1;
                    }
                }
                return true;
            }
        }

        false
    }

    fn attack_or_move_towards(&mut self, x: u32, y:u32, elf_target: bool) {
        if self.elves_alive == 0 || self.goblins_alive == 0 {
            self.full_round = false;
            return;
        }

        // Attack.
        if self.attack(x, y, elf_target) {
            if let MapCell::Unit { ref mut even, .. } = self.at(x, y) {
                *even = !*even;
            }
            return;
        }

        // Move.
        let mut closest_distance = std::u32::MAX;
        let mut closest_target = (0, 0);
        for target_y in 0..self.height {
            for target_x in 0..self.width {
                if x == target_x && y == target_y {
                    continue;
                }
                let cell = self.at(target_x, target_y);
                match cell {
                    MapCell::Unit{elf , ..} => {
                        if *elf == elf_target {
                            let distance = self.shortest_distance(x, y, target_x, target_y);
                            if distance < closest_distance {
                                closest_distance = distance;
                                closest_target = (target_x, target_y);
                            }
                        }
                    },
                    _ => {}
                }
            }
        }

        if closest_distance != std::u32::MAX {
            // Which direction?
            for (dx, dy) in [(0, -1i32), (-1i32, 0), (1, 0), (0, 1)].iter() {
                let nx = (x as i32 + *dx) as u32;
                let ny = (y as i32 + *dy) as u32;
                if let MapCell::Open = self.at(nx, ny) {
                    if self.shortest_distance(nx, ny, closest_target.0, closest_target.1) == closest_distance - 1 {
                        let mut cell_value = *self.at(x, y);
                        if let MapCell::Unit { ref mut even, .. } = cell_value {
                            *even = !*even;
                        }
                        self.put(nx, ny, cell_value);
                        self.put(x, y, MapCell::Open);

                        // Attack from new position if possible.
                        self.attack(nx, ny, elf_target);

                        return;
                    }
                }
            }
        }


        if let MapCell::Unit { ref mut even, .. } = self.at(x, y) {
            *even = !*even;
        }
    }

    // TODO: We are visting everything, do not throw away for each opponent.
    fn shortest_distance(&mut self, sx: u32, sy: u32, dx: u32, dy: u32) -> u32 {
        let mut to_visit = BinaryHeap::new();
        let mut visited = HashSet::new();

        to_visit.push((0i32, sx, sy));

        while let Some(visiting) = to_visit.pop() {
            visited.insert((visiting.1, visiting.2));
            for (nx, ny) in [(0,-1i32), (-1i32, 0), (1,0), (0,1)].iter() {
                let cost = -visiting.0 + 1;
                let x = (visiting.1 as i32 + *nx) as u32;
                let y = (visiting.2 as i32 + *ny) as u32;
                if x == dx && y == dy {
                    return cost as u32;
                }
                if let MapCell::Open = self.at(x, y) {
                    if !visited.contains(&(x, y)) {
                        to_visit.push((-cost, x, y));
                    }
                }
            }
        }

        std::u32::MAX
    }

    fn print(&mut self) {
        println!("Round {}", self.round);
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.at(x, y);
                let c = match cell {
                    &mut MapCell::Open => ".",
                    &mut MapCell::Wall => "#",
                    &mut MapCell::Unit{elf:false, ..} => "G",
                    &mut MapCell::Unit{elf:true, ..} => "E"
                };
                print!("{}", c);
            }
            print!("   ");
            for x in 0..self.width {
                let cell = self.at(x, y);
                if let MapCell::Unit { hit_points, elf:true, .. } = cell {
                    print!("E({}), ", hit_points);
                } else if let MapCell::Unit { hit_points, elf:false, .. } = cell {
                    print!("G({}), ", hit_points);
                }
            }
            println!();
        }
        println!();
    }
}

pub fn part1(input_string: &str) -> String {
    let mut board = Board::parse(input_string);
    loop {
        board.perform_round();
//        board.print();
        if let Some(outcome) = board.calculate_outcome() {
            return outcome.to_string()
        }
    }
}

pub fn part2(_input_string: &str) -> String {
    "".to_string()
}

#[test]
fn tests_part1() {
    assert_eq!("27730", part1("#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######"));

    assert_eq!("36334", part1("#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######"));


    assert_eq!("39514", part1("#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######"));

    assert_eq!("27755", part1("#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######"));

    assert_eq!("28944", part1("#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######"));

    assert_eq!("18740", part1("#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########"));

    //assert_eq!("27730", part1(include_str!("day15_input.txt")));
}

#[test]
#[ignore]
fn tests_part2() {
    assert_eq!("", part2(""));

    assert_eq!("", part2(include_str!("day14_input.txt")));
}
