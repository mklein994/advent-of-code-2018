use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

fn main() {
    // let input = aoc2018::read_file(7);
    let input = "\
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";
    // println!("{}", input);

    let step_order = part1(&input);
    println!("{}", step_order);
}

fn part1(input: &str) -> &str {
    let steps = input
        .lines()
        .map(|line| {
            line.parse::<Instruction>()
                .expect("Couldn't parse instruction")
        })
        .collect::<Vec<_>>();
    steps.iter().for_each(|s| println!("{:?}", s));

    unimplemented!()
}

#[derive(Debug)]
struct Instruction(char, char);

impl FromStr for Instruction {
    type Err = std::char::ParseCharError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin.")
                    .unwrap();
        }
        let caps = RE.captures(s).unwrap();

        Ok(Instruction(caps[1].parse()?, caps[2].parse()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        let input = "\
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";
        assert_eq!("CABDFE", part1(&input));
    }

    #[test]
    #[ignore]
    fn test_part_1_input() {
        let input = include_str!("../../input/7.txt");
        assert_eq!("", part1(&input));
    }
}
