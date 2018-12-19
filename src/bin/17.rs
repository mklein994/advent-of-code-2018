use lazy_static::lazy_static;
use regex::Regex;
use std::cmp;
use std::str::FromStr;

fn main() {
    let input = aoc2018::read_file(17);
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
    println!("{}", area);
}

fn part1(input: &str) -> Result<usize, Box<dyn std::error::Error>> {
    const MAX_TIME: u64 = 10;

    let mut clay_veins: Vec<Clay> = vec![];
    for line in input.lines() {
        clay_veins.push(line.parse()?);
    }

    // The water spring is at (0, 500), so ensure the bounds includes that.
    let mut bounds = Bounds {
        top: 0,
        bottom: 0,
        left: 500,
        right: 500,
    };

    for vein in &clay_veins {
        // `bounds.top` is already determined at its limit.
        bounds.bottom = cmp::max(bounds.bottom, vein.y2);
        bounds.left = cmp::min(bounds.left, vein.x1 - 1);
        bounds.right = cmp::max(bounds.right, vein.x2 + 1);
    }

    println!("{:?}", bounds);

    // Make the ground sand by default.
    let mut ground = Ground {
        grid: vec![vec![b'.'; bounds.width() + 1]; bounds.height() + 1],
        time: 0,
    };

    // Add the water spring.
    ground.grid[0][500 - bounds.left] = b'+';

    // Add the clay veins.
    for clay in clay_veins {
        for row in clay.y1..=clay.y2 {
            for column in clay.x1..=clay.x2 {
                // println!("{:?}", (column, row));
                ground.grid[row][column - bounds.left] = b'#';
            }
        }
    }

    #[rustfmt::skip]
    const NEIGHBOURS: [(isize, isize); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        ( 0, -1),          ( 0, 1),
        ( 1, -1), ( 1, 0), ( 1, 1),
    ];

    while ground.time < MAX_TIME {
        ground.time += 1;

        let grid = &mut ground.grid;

        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                let neighbours: Neighbours = NEIGHBOURS
                    .into_iter()
                    .map(|&(r, c)| {
                        if row as isize + r >= 0
                            && col as isize + c >= 0
                            && ((row as isize + r) as usize) < grid.len()
                            && ((col as isize + c) as usize) < grid[0].len()
                        {
                            Some(grid[(row as isize + r) as usize][(col as isize + c) as usize])
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
                    .into();
                let mut current = &mut grid[row][col];

                process_cell(&mut current, &neighbours);
            }
        }

        draw_ground(&grid);
    }

    Ok(0)
}

#[derive(Debug)]
struct Neighbours {
    tl: Option<GroundType>,
    tm: Option<GroundType>,
    tr: Option<GroundType>,
    ml: Option<GroundType>,
    mr: Option<GroundType>,
    bl: Option<GroundType>,
    bm: Option<GroundType>,
    br: Option<GroundType>,
}

impl From<Vec<Option<u8>>> for Neighbours {
    fn from(list: Vec<Option<u8>>) -> Self {
        let list = list
            .into_iter()
            .map(|i| match i {
                Some(t) => Some(GroundType::from(t)),
                None => None,
            })
            .collect::<Vec<_>>();
        Self {
            tl: list[0],
            tm: list[1],
            tr: list[2],
            ml: list[3],
            mr: list[4],
            bl: list[5],
            bm: list[6],
            br: list[7],
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum GroundType {
    Sand,
    Spring,
    WasWet,
    Clay,
    Water,
}

impl From<u8> for GroundType {
    fn from(c: u8) -> Self {
        match c {
            b'.' => GroundType::Sand,
            b'+' => GroundType::Spring,
            b'|' => GroundType::WasWet,
            b'#' => GroundType::Clay,
            b'~' => GroundType::Water,
            _ => panic!("unknown ground type"),
        }
    }
}

fn process_cell(current: &mut u8, neighbours: &Neighbours) {
    let current_type = GroundType::from(*current);

    if current_type == GroundType::Sand
        && neighbours.tm.filter(|x| *x == GroundType::Spring).is_some()
    {
        *current = b'|';
    }

    if neighbours.tm.filter(|x| *x == GroundType::WasWet).is_some()
        && current_type == GroundType::Sand
    {
        *current = b'|';
    }

    // TODO: Don't change `current`, as this affects the surrounding cells before the minute
    // completed.
    unimplemented!("do the actual logic here");
}

fn draw_ground(grid: &[Vec<u8>]) {
    grid.iter()
        .for_each(|p| println!("{}", std::str::from_utf8(&p).unwrap()));
}

#[derive(Debug)]
struct Ground {
    grid: Vec<Vec<u8>>,
    time: u64,
}

#[derive(Debug)]
struct Clay {
    x1: usize,
    x2: usize,
    y1: usize,
    y2: usize,
}

#[derive(Debug)]
struct Bounds {
    top: usize,
    bottom: usize,
    left: usize,
    right: usize,
}

impl Bounds {
    fn width(&self) -> usize {
        self.right - self.left
    }

    fn height(&self) -> usize {
        self.bottom - self.top
    }
}

impl FromStr for Clay {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
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

        let caps = RE.captures(s).unwrap();

        let coord: usize = caps["coord"].parse()?;
        let start: usize = caps["start"].parse()?;
        let end: usize = caps["end"].parse()?;

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
            _ => panic!("unknown clay vein"),
        }
    }
}
