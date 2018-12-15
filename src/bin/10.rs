use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

fn main() {
    let input = aoc2018::read_file(10);
    // println!("{}", input);

    part1(&input).unwrap();
}

fn part1(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut points: Vec<Point> = Vec::with_capacity(input.lines().count());
    for line in input.lines() {
        // println!("{}", line);
        points.push(line.parse::<Point>()?);
    }

    points.iter().for_each(|p| println!("{:?}", p));

    Ok(())
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
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?x)
              position=<\s*(?P<px>[-0-9]+),\s*(?P<py>[-0-9]+)>
              \s+
              velocity=<\s*(?P<vx>[-0-9]+),\s*(?P<vy>[-0-9]+)>"
            )
            .unwrap();
        }

        let caps = RE.captures(s).expect("missing captures");

        let coord: Coord = Coordinate {
            x: caps["px"].parse()?,
            y: caps["py"].parse()?,
        };

        let velocity: Velocity = Coordinate {
            x: caps["vx"].parse()?,
            y: caps["vy"].parse()?,
        };

        Ok(Point { coord, velocity })
    }
}
