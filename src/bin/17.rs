use lazy_static::lazy_static;
use regex::Regex;
use std::ops::RangeInclusive;
use std::str::FromStr;

fn main() {
    // let input = aoc2018::read_file(17);
    let input = "\
x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504";
    // println!("{}", input);

    let area = part1(&input).unwrap();
    println!("{:?}", area);
}

fn part1(input: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let mut clay_pieces: Vec<ClayPiece> = Vec::new();
    for line in input.lines() {
        clay_pieces.push(line.parse()?);
    }
    clay_pieces.iter().for_each(|p| println!("{:?}", p));
    Ok(0)
}

#[derive(Debug)]
struct ClayPiece {
    x: RangeInclusive<u32>,
    y: RangeInclusive<u32>,
}

impl Default for ClayPiece {
    fn default() -> Self {
        Self { x: 0..=0, y: 0..=0 }
    }
}

impl FromStr for ClayPiece {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?x)
            [xy]=
            (?P<coord>[0-9]+)
            ,\s+
            [xy]=
            (?P<start>[0-9]+)
            \.{2}
            (?P<end>[0-9]+)"
            )
            .unwrap();
        }

        let caps = RE.captures(s).unwrap();

        let coord = caps["coord"].parse()?;
        let start = caps["start"].parse()?;
        let end = caps["end"].parse()?;

        let piece = match &s[0..1] {
            "x" => ClayPiece {
                x: coord..=coord,
                y: start..=end,
            },
            "y" => ClayPiece {
                x: start..=end,
                y: coord..=coord,
            },
            _ => panic!("unknown clay piece"),
        };

        Ok(piece)
    }
}
