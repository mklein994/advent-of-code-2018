use std::str::FromStr;

fn main() {
    let input = aoc2018::read_file(6);
    println!("{}", input);
}

fn part1(input: &str) -> usize {
    unimplemented!()
}

#[derive(Debug)]
struct Point(i32, i32);

impl FromStr for Point {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
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
        assert_eq!(17, part1(&EXAMPLE_INPUT));
    }
}
