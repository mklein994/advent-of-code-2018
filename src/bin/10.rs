use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

fn main() {
    let input = aoc2018::read_file(10);
    let input = "\
position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>";
    // println!("{}", input);

    part1(&input).unwrap();
}

fn part1(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut points: Vec<Point> = Vec::with_capacity(input.lines().count());
    for line in input.lines() {
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
