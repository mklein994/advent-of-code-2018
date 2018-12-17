use std::str::FromStr;
use regex::Regex;
use lazy_static::lazy_static;
use std::ops::Range;

fn main() {
    let input = aoc2018::read_file(17);
    println!("{}", input);
}

#[derive(Debug)]
struct ClayPiece {
    x: Range<u32>,
    y: Range<u32>,
}

impl FromStr for ClayPiece {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}
