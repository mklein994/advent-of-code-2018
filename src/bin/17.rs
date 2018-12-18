use lazy_static::lazy_static;
use regex::Regex;
use std::cmp;
use std::fmt;
use std::str::FromStr;

const WATER_SPRING: (u32, u32) = (0, 500);

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

    let clay = Clay::new(clay_pieces);

    // clay_pieces.iter().for_each(|p| println!("{:?}", p));

    let mut grid = Grid::from(clay);
    grid.add_water_spring();

    println!("{}", grid);

    Ok(0)
}

struct Grid {
    grid: Vec<Vec<u8>>,
    bounds: Bounds,
    time: u64,
}

impl Grid {
    fn add_water_spring(&mut self) {
        self.grid[WATER_SPRING.0 as usize][WATER_SPRING.1 as usize - self.bounds.left as usize] =
            b'+';
    }

    fn step(&mut self) {
        self.time += 1;
    }
}

impl From<Clay> for Grid {
    fn from(clay: Clay) -> Self {
        let mut grid = vec![vec![b'.'; clay.bounds.width() + 1]; clay.bounds.height() + 1];

        for piece in clay.pieces {
            for row in piece.y1..=piece.y2 {
                for column in piece.x1..=piece.x2 {
                    grid[row as usize][column as usize] = b'#';
                }
            }
        }

        Grid {
            grid,
            bounds: clay.bounds,
            time: 0,
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.grid
            .iter()
            .try_for_each(|row| writeln!(f, "{}", std::str::from_utf8(&row).unwrap()))
    }
}

#[derive(Debug)]
struct Clay {
    pieces: Vec<ClayPiece>,
    bounds: Bounds,
}

impl Clay {
    fn new(mut pieces: Vec<ClayPiece>) -> Self {
        let bounds = Self::get_bounds(&pieces);

        // Normalize the pieces to fit within the bounds.
        pieces.iter_mut().for_each(|p| {
            p.x1 -= bounds.left;
            p.x2 -= bounds.left;
            p.y1 -= bounds.top;
            p.y2 -= bounds.top;
        });

        Self { pieces, bounds }
    }

    fn get_bounds(pieces: &[ClayPiece]) -> Bounds {
        let initial_bounds = Bounds {
            // The water spring starts at (0, 500), so the top must be 0. The x coordinates will be
            // normalized to fit within the bounds.
            top: WATER_SPRING.0,
            bottom: pieces[0].y2,
            left: pieces[0].x1,
            right: pieces[0].x2,
        };

        let bounds = pieces.iter().fold(initial_bounds, |mut b, p| {
            b.top = cmp::min(b.top, p.y1);
            b.bottom = cmp::max(b.bottom, p.y2);
            b.left = cmp::min(b.left, p.x1);
            b.right = cmp::max(b.right, p.x2);
            b
        });
        assert!(
            bounds.left <= 500 && bounds.right >= 500,
            "water from spring does not fall on clay pieces"
        );
        bounds
    }
}

#[derive(Debug)]
struct Bounds {
    top: u32,
    bottom: u32,
    left: u32,
    right: u32,
}

impl Bounds {
    fn width(&self) -> usize {
        (self.right - self.left) as usize
    }

    fn height(&self) -> usize {
        (self.bottom - self.top) as usize
    }
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
            "x" => Self {
                x1: coord,
                x2: coord,
                y1: start,
                y2: end,
            },
            "y" => Self {
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
