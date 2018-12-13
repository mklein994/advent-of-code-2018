use std::str::FromStr;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    // let input = aoc2018::read_file(6);
    let input = "\
1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";

    run(&input, part1);
}

/// Generic run function to handle errors.
///
/// This is generic to allow both part 1 and 2 to use it.
fn run<F, T>(input: &str, func: F)
where
    F: Fn(&str) -> Result<T>,
{
    if let Err(e) = func(input) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn part1(input: &str) -> Result<usize> {
    let points = parse_points(input)?;

    let largest_area = largest_area(&points)?;
    Ok(largest_area)
}

fn largest_area(points: &[Point]) -> Result<usize> {
    for point in points {
        let shortest_distance = point.shortest_distance(&points);
        println!("{:?}: {}", point, shortest_distance);
    }
    unimplemented!()
}

fn parse_points(input: &str) -> Result<Vec<Point>> {
    let mut points = Vec::with_capacity(input.lines().count());
    for line in input.lines() {
        points.push(line.parse()?);
    }

    Ok(points)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point(i32, i32);

impl Point {
    fn distance_to_point(&self, other: &Self) -> i32 {
        i32::abs(self.0 - other.0) + i32::abs(self.1 - other.1)
    }

    fn shortest_distance(&self, points: &[Point]) -> i32 {
        points
            .iter()
            .filter(|p| *p != self)
            .map(|p| p.distance_to_point(self))
            .min()
            .unwrap()
    }
}

impl FromStr for Point {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(", ").collect();
        let point = Point(coords[0].parse()?, coords[1].parse()?);
        Ok(point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";

    #[test]
    fn part1_example() {
        assert_eq!(17, part1(&EXAMPLE_INPUT).unwrap());
    }
}
