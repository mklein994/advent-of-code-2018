use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

fn main() {
    let input = aoc2018::read_file(10);
    println!("{}", input);

    part1(&input);
}

fn part1(input: &str) {
    unimplemented!();
}

#[derive(Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

type Coord = Coordinate;
type Velocity = Coordinate;

#[derive(Debug)]
struct Point {
    coord: Coord,
    velocity: Velocity,
}

impl FromStr for Point {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}
