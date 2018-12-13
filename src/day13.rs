use std::collections::{HashMap, HashSet};

enum TrackPiece {
    // ⌜
    TopLeft,
    // ⌝
    TopRight,
    // ⌞
    BottomLeft,
    // ⌟
    BottomRight,
    // |
    Vertical,
    // +
    Intersection,
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone, Ord, PartialOrd)]
struct Vector {
    y: i32,
    x: i32,
}

impl Vector {
    fn rotate_with(&mut self, matrix: &RotationMatrix) {
        let old_x = self.x;
        self.x = self.x * matrix.0 + self.y * matrix.1;
        self.y = old_x * matrix.2 + self.y * matrix.3;
    }
    fn add(&mut self, other: Vector) {
        self.x += other.x;
        self.y += other.y;
    }
}

struct RotationMatrix(i32, i32, i32, i32);

struct Cart {
    direction: Vector,
    turns: i32,
    position: Vector,
}

impl Cart {
    fn advance(&mut self) {
        self.position.add(self.direction);
    }
}

struct Track {
    track: HashMap<Vector, TrackPiece>,
    carts: Vec<Cart>,
    cart_positions: HashSet<Vector>,
}

impl Track {
    fn parse(input_string: &str) -> Track {
        let mut carts = Vec::new();
        let mut track = HashMap::new();
        for (y, line) in input_string.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let position = Vector {
                    x: x as i32,
                    y: y as i32,
                };
                match c {
                    '^' => {
                        carts.push(Cart {
                            direction: Vector { x: 0, y: -1 },
                            turns: 0,
                            position,
                        });
                    }
                    'v' => {
                        carts.push(Cart {
                            direction: Vector { x: 0, y: 1 },
                            turns: 0,
                            position,
                        });
                    }
                    '<' => {
                        carts.push(Cart {
                            direction: Vector { x: -1, y: 0 },
                            turns: 0,
                            position,
                        });
                    }
                    '>' => {
                        carts.push(Cart {
                            direction: Vector { x: 1, y: 0 },
                            turns: 0,
                            position,
                        });
                    }
                    '+' => {
                        track.insert(position, TrackPiece::Intersection);
                    }
                    '|' => {
                        track.insert(position, TrackPiece::Vertical);
                    }
                    '/' => {
                        if y == 0 {
                            track.insert(position, TrackPiece::TopLeft);
                        } else {
                            track.insert(
                                position,
                                match track.get(&Vector {
                                    x: position.x,
                                    y: position.y - 1,
                                }) {
                                    Some(TrackPiece::Vertical) => TrackPiece::BottomRight,
                                    _ => TrackPiece::TopLeft,
                                },
                            );
                        }
                    }
                    '\\' => {
                        track.insert(position, TrackPiece::Intersection);
                        if y == 0 {
                            track.insert(position, TrackPiece::TopRight);
                        } else {
                            track.insert(
                                position,
                                match track.get(&Vector {
                                    x: position.x,
                                    y: position.y - 1,
                                }) {
                                    Some(TrackPiece::Vertical) => TrackPiece::BottomLeft,
                                    _ => TrackPiece::TopRight,
                                },
                            );
                        }
                    }
                    _ => {
                        // Ignore
                    }
                }
            }
        }
        Track {
            track,
            carts,
            cart_positions: HashSet::new(),
        }
    }

    fn find_crash(&mut self) -> Vector {
        let left_matrix = RotationMatrix(0, 1, -1, 0);
        let identity_matrix = RotationMatrix(1, 0, 0, 1);
        let right_matrix = RotationMatrix(0, -1, 1, 0);
        loop {
            self.carts.sort_by(|a, b| a.position.cmp(&b.position));

            for cart in self.carts.iter_mut() {
                self.cart_positions.remove(&cart.position);

                cart.advance();

                if !self.cart_positions.insert(cart.position) {
                    return cart.position;
                }

                match self.track.get(&cart.position) {
                    Some(TrackPiece::TopRight) => {
                        cart.direction = Vector {
                            x: cart.direction.y,
                            y: cart.direction.x,
                        };
                    }
                    Some(TrackPiece::TopLeft) => {
                        cart.direction = Vector {
                            x: -cart.direction.y,
                            y: -cart.direction.x,
                        };
                    }
                    Some(TrackPiece::BottomLeft) => {
                        cart.direction = Vector {
                            x: cart.direction.y,
                            y: cart.direction.x,
                        };
                    }
                    Some(TrackPiece::BottomRight) => {
                        cart.direction = Vector {
                            x: -cart.direction.y,
                            y: -cart.direction.x,
                        };
                    }
                    Some(TrackPiece::Intersection) => {
                        cart.direction.rotate_with(match cart.turns {
                            0 => &left_matrix,
                            2 => &right_matrix,
                            _ => &identity_matrix,
                        });
                        cart.turns = (cart.turns + 1) % 3;
                    }
                    _ => {
                        // Do nothing.
                    }
                }
            }
        }
    }

    fn find_remaining(&mut self) -> Vector {
        let left_matrix = RotationMatrix(0, 1, -1, 0);
        let identity_matrix = RotationMatrix(1, 0, 0, 1);
        let right_matrix = RotationMatrix(0, -1, 1, 0);

        loop {
            self.carts.sort_by(|a, b| a.position.cmp(&b.position));

            let mut removed_positions = HashSet::new();
            for cart in self.carts.iter_mut() {
                if removed_positions.contains(&cart.position) {
                    continue;
                }

                self.cart_positions.remove(&cart.position);

                cart.advance();

                match self.cart_positions.get(&cart.position) {
                    Some(_) => {
                        self.cart_positions.remove(&cart.position);
                        removed_positions.insert(cart.position);
                        continue;
                    }
                    None => {
                        self.cart_positions.insert(cart.position);
                    }
                };

                match self.track.get(&cart.position) {
                    Some(TrackPiece::TopRight) => {
                        cart.direction = Vector {
                            x: cart.direction.y,
                            y: cart.direction.x,
                        };
                    }
                    Some(TrackPiece::TopLeft) => {
                        cart.direction = Vector {
                            x: -cart.direction.y,
                            y: -cart.direction.x,
                        };
                    }
                    Some(TrackPiece::BottomLeft) => {
                        cart.direction = Vector {
                            x: cart.direction.y,
                            y: cart.direction.x,
                        };
                    }
                    Some(TrackPiece::BottomRight) => {
                        cart.direction = Vector {
                            x: -cart.direction.y,
                            y: -cart.direction.x,
                        };
                    }
                    Some(TrackPiece::Intersection) => {
                        cart.direction.rotate_with(match cart.turns {
                            0 => &left_matrix,
                            2 => &right_matrix,
                            _ => &identity_matrix,
                        });
                        cart.turns = (cart.turns + 1) % 3;
                    }
                    _ => {
                        // Do nothing.
                    }
                }
            }

            if !removed_positions.is_empty() {
                self.carts
                    .retain(|cart| !removed_positions.contains(&cart.position));
                if self.carts.len() == 1 {
                    return self.carts[0].position;
                }
            }
        }
    }
}

pub fn part1(input_string: &str) -> String {
    let mut track = Track::parse(input_string);
    let crash_position = track.find_crash();
    format!("{},{}", crash_position.x, crash_position.y)
}

pub fn part2(input_string: &str) -> String {
    let mut track = Track::parse(input_string);
    let remaining_position = track.find_remaining();
    format!("{},{}", remaining_position.x, remaining_position.y)
}

#[test]
fn tests_part1() {
    assert_eq!(
        "0,3",
        part1(
            "|
v
|
|
|
^
|"
        )
    );

    assert_eq!(
        "7,3",
        part1(
            r#"/->-\
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/"#
        )
    );

    assert_eq!("65,73", part1(include_str!("day13_input.txt")));
}

#[test]
fn tests_part2() {
    assert_eq!(
        "6,4",
        part2(
            r#"/>-<\
|   |
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/
"#
        )
    );

    assert_eq!("54,66", part2(include_str!("day13_input.txt")));
}
