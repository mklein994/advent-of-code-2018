use lazy_static::lazy_static;
use regex::Regex;
use std::error::Error;
use std::str::FromStr;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<Error>::from(format!($($tt)*))) }
}

type Result<T> = std::result::Result<T, Box<Error>>;

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

    let area = part1(&input).unwrap();
}

fn part1(input: &str) -> Result<i64> {
    let mut veins: Vec<ClayVein> = vec![];
    for line in input.lines() {
        veins.push(line.parse()?);
    }
    println!("{:?}", veins);

    unimplemented!()
}

#[derive(Debug)]
struct ClayVein {
    x1: i64,
    x2: i64,
    y1: i64,
    y2: i64,
}

impl FromStr for ClayVein {
    type Err = Box<Error>;
    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?x)
                [xy]=(?P<coord>[0-9]+)
                ,\s+
                [xy]=(?P<start>[0-9]+)
                \.{2}
                (?P<end>[0-9]+)"
            )
            .unwrap();
        }

        let caps = RE.captures(s).expect("Failed to collect captures");

        let coord = caps["coord"].parse()?;
        let start = caps["start"].parse()?;
        let end = caps["end"].parse()?;

        match &s[0..1] {
            "x" => Ok(Self {
                x1: coord,
                x2: coord,
                y1: start,
                y2: end,
            }),
            "y" => Ok(Self {
                x1: start,
                x2: end,
                y1: coord,
                y2: coord,
            }),
            _ => err!("invalid clay vein"),
        }
    }
}
