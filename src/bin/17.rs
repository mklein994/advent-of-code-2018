use lazy_static::lazy_static;
use regex::Regex;
use std::cmp;
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
    let bounds = get_bounds(&clay_pieces);
    clay_pieces.iter_mut().for_each(|p| {
        p.x1 -= bounds.left;
        p.x2 -= bounds.left;
        p.y1 -= bounds.top;
        p.y2 -= bounds.top;
    });
    clay_pieces.iter().for_each(|p| println!("{:?}", p));

    draw_grid(&clay_pieces);

    Ok(0)
}

#[derive(Debug)]
struct Bounds {
    top: u32,
    bottom: u32,
    left: u32,
    right: u32,
}

fn draw_grid(pieces: &[ClayPiece]) {
    let bounds = get_bounds(&pieces);
    println!("{:?}", bounds);

    for y in bounds.top..=bounds.bottom {
        for x in bounds.left..=bounds.right {
            println!("{:?}", (x, y));
        }
    }
}

fn get_bounds(pieces: &[ClayPiece]) -> Bounds {
    let initial_bounds = Bounds {
        top: pieces[0].y1,
        bottom: pieces[0].y2,
        left: pieces[0].x1,
        right: pieces[0].x2,
    };

    pieces.iter().fold(initial_bounds, |mut b, p| {
        b.top = cmp::min(b.top, p.y1);
        b.bottom = cmp::max(b.bottom, p.y2);
        b.left = cmp::min(b.left, p.x1);
        b.right = cmp::max(b.right, p.x2);
        b
    })
}

#[derive(Debug, Default)]
struct ClayPiece {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
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
                x1: coord,
                x2: coord,
                y1: start,
                y2: end,
            },
            "y" => ClayPiece {
                x1: start,
                x2: end,
                y1: coord,
                y2: coord,
            },
            _ => panic!("unknown clay piece"),
        };

        Ok(piece)
    }
}
