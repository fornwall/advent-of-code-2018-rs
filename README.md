[![Build Status](https://travis-ci.org/fornwall/advent-of-code-2018-rs.svg?branch=master)](https://travis-ci.org/fornwall/advent-of-code-2018-rs)

# Advent of Code 2018 in Rust
These are solutions to the problems in [Advent of Code 2018](https://adventofcode.com/2018) in Rust.

# Running the solutions against a file
The test can be run to read from stdin with

    cargo run <day> <part> < path/to/input.txt
    # Example:
    cargo run 2 1 < path/to/input_day2_part1.txt

There is also a Docker image published for running the tests:

    docker run -i fredrikfornwall/advent-of-code-2018-rs:latest 2 2 < path/to/input.txt
    # Example
    curl https://public.infinisil.com/aoc18/day2/10000.txt | \
        docker run -i fredrikfornwall/advent-of-code-2018-rs:latest 2 2

# Days
| Puzzle                                         | Solution                     | Alternatives |
| ---------------------------------------------- | ---------------------------- | ------------ |
| [Day 1](https://adventofcode.com/2018/day/1)   | [src/day1.rs](src/day1.rs)   | [Reddit](https://www.reddit.com/r/adventofcode/comments/a20646/2018_day_1_solutions/)
| [Day 2](https://adventofcode.com/2018/day/2)   | [src/day2.rs](src/day2.rs)   | [Reddit](https://www.reddit.com/r/adventofcode/comments/a2aimr/2018_day_2_solutions/)
| [Day 3](https://adventofcode.com/2018/day/3)   | [src/day3.rs](src/day3.rs)   | [Reddit](https://www.reddit.com/r/adventofcode/comments/a2lesz/2018_day_3_solutions/)
| [Day 4](https://adventofcode.com/2018/day/4)   | [src/day4.rs](src/day4.rs)   | [Reddit](https://www.reddit.com/r/adventofcode/comments/a2xef8/2018_day_4_solutions/)
| [Day 5](https://adventofcode.com/2018/day/5)   | [src/day5.rs](src/day5.rs)   | [Reddit](https://www.reddit.com/r/adventofcode/comments/a3912m/2018_day_5_solutions/)
| [Day 6](https://adventofcode.com/2018/day/6)   | [src/day6.rs](src/day6.rs)   | [Reddit](https://www.reddit.com/r/adventofcode/comments/a3kr4r/2018_day_6_solutions/)
