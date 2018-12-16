#[derive(Copy, Clone, PartialEq)]
struct Registers {
    values: [u8; 4],
    error: bool,
}

#[derive(Copy, Clone, Debug)]
enum Opcode {
    Addr, // (add register) stores into register C the result of adding register A and register B
    Addi, // (add immediate) stores into register C the result of adding register A and value B.
    Mulr, // (multiply register) stores into register C the result of multiplying register A and register B.
    Muli, // (multiply immediate) stores into register C the result of multiplying register A and value B.
    Banr, // (bitwise AND register) stores into register C the result of the bitwise AND of register A and register B.
    Bani, // (bitwise AND immediate) stores into register C the result of the bitwise AND of register A and value B.
    Borr, // (bitwise OR register) stores into register C the result of the bitwise OR of register A and register B.
    Bori, // (bitwise OR immediate) stores into register C the result of the bitwise OR of register A and value B.
    Setr, // (set register) copies the contents of register A into register C. (Input B is ignored.)
    Seti, // (set immediate) stores value A into register C. (Input B is ignored.)
    Gtir, // (greater-than immediate/register) sets register C to 1 if value A is greater than register B. Otherwise, register C is set to 0.
    Gtri, // (greater-than register/immediate) sets register C to 1 if register A is greater than value B. Otherwise, register C is set to 0.
    Gtrr, // (greater-than register/register) sets register C to 1 if register A is greater than register B. Otherwise, register C is set to 0.
    Eqir, // (equal immediate/register) sets register C to 1 if value A is equal to register B. Otherwise, register C is set to 0.
    Eqri, // (equal register/immediate) sets register C to 1 if register A is equal to value B. Otherwise, register C is set to 0.
    Eqrr, // (equal register/register) sets register C to 1 if register A is equal to register B. Otherwise, register C is set to 0.
}

impl Registers {
    fn of(r0: u8, r1: u8, r2: u8, r3: u8) -> Registers {
        Registers {
            values: [r0, r1, r2, r3],
            error: false,
        }
    }

    fn reg(&mut self, index: u8) -> u8 {
        self.values[index as usize]
    }

    fn apply(&mut self, opcode: Opcode, a: u8, b: u8, c: u8) {
        let c = c as usize;
        self.values[c] = match opcode {
            Opcode::Addr => self.reg(a) + self.reg(b),
            Opcode::Addi => self.reg(a) + b,
            Opcode::Mulr => self.reg(a) * self.reg(b),
            Opcode::Muli => self.reg(a) * b,
            Opcode::Banr => self.reg(a) & self.reg(b),
            Opcode::Bani => self.reg(a) & b,
            Opcode::Borr => self.reg(a) | self.reg(b),
            Opcode::Bori => self.reg(a) | b,
            Opcode::Setr => self.reg(a),
            Opcode::Seti => a,
            Opcode::Gtir => (a > self.reg(b)) as u8,
            Opcode::Gtri => (self.reg(a) > b) as u8,
            Opcode::Gtrr => (self.reg(a) > self.reg(b)) as u8,
            Opcode::Eqir => (a == self.reg(b)) as u8,
            Opcode::Eqri => (self.reg(a) == b) as u8,
            Opcode::Eqrr => (self.reg(a) == self.reg(b)) as u8,
        }
    }
}

pub fn part1(input_string: &str) -> String {
    let mut registers_before = Registers::of(0, 0, 0, 0);
    let mut instruction: Vec<u8> = Vec::new();
    let mut last_blank = true;
    let mut result = 0;

    for line in input_string.lines() {
        if line.is_empty() {
            if last_blank {
                break;
            } else {
                last_blank = true;
                continue;
            }
        }

        last_blank = false;

        let before = line.starts_with("Before:");
        let after = line.starts_with("After:");
        if before || after {
            let parts: Vec<u8> = line[9..]
                .split(|c| c == '[' || c == ']' || c == ',')
                .filter(|s| !s.is_empty())
                .map(|n| n.trim().parse::<u8>().unwrap())
                .collect();

            if before {
                registers_before = Registers::of(parts[0], parts[1], parts[2], parts[3]);
            } else {
                let registers_after = Registers::of(parts[0], parts[1], parts[2], parts[3]);
                let mut sum = 0;

                for opcode in [
                    Opcode::Addr,
                    Opcode::Addi,
                    Opcode::Mulr,
                    Opcode::Muli,
                    Opcode::Banr,
                    Opcode::Bani,
                    Opcode::Borr,
                    Opcode::Bori,
                    Opcode::Setr,
                    Opcode::Seti,
                    Opcode::Gtir,
                    Opcode::Gtri,
                    Opcode::Gtrr,
                    Opcode::Eqir,
                    Opcode::Eqri,
                    Opcode::Eqrr,
                ]
                .iter()
                {
                    let mut registers_applied = registers_before;
                    registers_applied.apply(
                        *opcode,
                        instruction[1],
                        instruction[2],
                        instruction[3],
                    );
                    if registers_applied == registers_after {
                        sum += 1;
                    }
                }
                if sum >= 3 {
                    result += 1;
                }
            }
        } else {
            instruction = line
                .split_whitespace()
                .map(|n| n.trim().parse::<u8>().unwrap())
                .collect();
        }
    }

    result.to_string()
}

pub fn part2(_input_string: &str) -> String {
    "".to_string()
}

#[test]
fn tests_part1() {
    assert_eq!(
        "1",
        part1(
            "Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]"
        )
    );

    assert_eq!("624", part1(include_str!("day16_input.txt")));
}

#[test]
#[ignore]
fn tests_part2() {
    assert_eq!("", part2(""));

    assert_eq!("", part2(include_str!("day16_input.txt")));
}
