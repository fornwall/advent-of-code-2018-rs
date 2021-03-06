# OUTDATED
See https://github.com/fornwall/advent-of-code instead.

[![Build Status](https://travis-ci.org/fornwall/advent-of-code-2018-rs.svg?branch=master)](https://travis-ci.org/fornwall/advent-of-code-2018-rs)

# Advent of Code 2018 in Rust
These are solutions to the problems in [Advent of Code 2018](https://adventofcode.com/2018) in Rust.

# Running the solutions against a file
The test can be run to read from stdin:

    cargo run <day> <part> < path/to/input.txt

    # Example:
    cargo run 2 1 < path/to/input_day2_part1.txt

# Running using Docker
There is also a Docker image published for running the tests:

    docker run -i fredrikfornwall/advent-of-code-2018-rs:latest <day> <part> < path/to/input.txt

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
| [Day 7](https://adventofcode.com/2018/day/7)   | [src/day7.rs](src/day7.rs)   | [Reddit](https://www.reddit.com/r/adventofcode/comments/a3wmnl/2018_day_7_solutions/)
| [Day 8](https://adventofcode.com/2018/day/8)   | [src/day8.rs](src/day8.rs)   | [Reddit](https://www.reddit.com/r/adventofcode/comments/a47ubw/2018_day_8_solutions/)
| [Day 9](https://adventofcode.com/2018/day/9)   | [src/day9.rs](src/day9.rs)   | [Reddit](https://www.reddit.com/r/adventofcode/comments/a4i97s/2018_day_9_solutions/)
| [Day 10](https://adventofcode.com/2018/day/10) | [src/day10.rs](src/day10.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/a4skra/2018_day_10_solutions/)
| [Day 11](https://adventofcode.com/2018/day/11) | [src/day11.rs](src/day11.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/a53r6i/2018_day_11_solutions/)
| [Day 12](https://adventofcode.com/2018/day/12) | [src/day12.rs](src/day12.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/a5eztl/2018_day_12_solutions/)
| [Day 13](https://adventofcode.com/2018/day/13) | [src/day13.rs](src/day13.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/a5qd71/2018_day_13_solutions/)
| [Day 14](https://adventofcode.com/2018/day/14) | [src/day14.rs](src/day14.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/a61ojp/2018_day_14_solutions/)
| [Day 15](https://adventofcode.com/2018/day/15) | [src/day15.rs](src/day15.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/a6chwa/2018_day_15_solutions/)
| [Day 16](https://adventofcode.com/2018/day/16) | [src/day16.rs](src/day16.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/a6mf8a/2018_day_16_solutions/)
| [Day 17](https://adventofcode.com/2018/day/17) | [src/day17.rs](src/day17.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/a6wpup/2018_day_17_solutions/)
| [Day 18](https://adventofcode.com/2018/day/18) | [src/day18.rs](src/day18.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/a77xq6/2018_day_18_solutions/)
| [Day 19](https://adventofcode.com/2018/day/19) | [src/day19.rs](src/day19.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/a7j9zc/2018_day_19_solutions/)
| [Day 20](https://adventofcode.com/2018/day/20) | [src/day20.rs](src/day20.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/7kz6ik/2017_day_20_solutions/)
| [Day 21](https://adventofcode.com/2018/day/21) | [src/day21.rs](src/day21.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/a86jgt/2018_day_21_solutions/)
| [Day 22](https://adventofcode.com/2018/day/22) | [src/day22.rs](src/day22.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/a8i1cy/2018_day_22_solutions/)
| [Day 23](https://adventofcode.com/2018/day/23) | [src/day23.rs](src/day23.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/a8s17l/2018_day_23_solutions/)
| [Day 24](https://adventofcode.com/2018/day/24) | [src/day24.rs](src/day24.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/a91ysq/2018_day_24_solutions/)
| [Day 25](https://adventofcode.com/2018/day/25) | [src/day25.rs](src/day25.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/a9c61w/2018_day_25_solutions/)
