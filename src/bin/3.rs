use lazy_static::lazy_static;
use regex::Regex;
use std::fmt;

fn main() -> Result<(), Box<std::error::Error>> {
    let input = aoc2018::read_file(3);
    // let input = "\
    // #1 @ 1,3: 4x4
    // #2 @ 3,1: 4x4
    // #3 @ 5,5: 2x2";

    let claimed_area = claimed_area(&input)?;
    println!("{}", claimed_area);

    Ok(())
}

fn claimed_area(input: &str) -> Result<u32, Box<std::error::Error>> {
    let claims = parse_claims(&input)?;

    let _ = claims
        .iter()
        .map(|c| {
            [
                (c.start.x, c.start.y),
                (c.start.x + c.size.width, c.start.y + c.size.height),
            ]
        })
        .inspect(|c| println!("{:?}", c))
        .collect::<Vec<_>>();

    Ok(0)
}

/// Parse a new `Claim` from a string.
///
/// # Example:
///
/// `#123 @ 3,2: 5x4` would be parsed as follows:
///
/// | Name              | Value |
/// | ----------------- | ----: |
/// | Id                |   123 |
/// | Inches from left  |    3" |
/// | Inches from right |    2" |
/// | Width             |    5" |
/// | Height            |    4" |
fn parse_claims(input: &str) -> Result<Vec<Claim>, Box<std::error::Error>> {
    lazy_static! {
        pub static ref CLAIM_PATTERN: Regex =
            Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
    }

    let mut claims: Vec<Claim> = Vec::with_capacity(input.lines().count());

    for line in input.lines() {
        let cap = CLAIM_PATTERN.captures(&line).unwrap();
        let id = cap.get(1).unwrap().as_str().parse()?;
        let start = Point {
            x: cap.get(2).unwrap().as_str().parse()?,
            y: cap.get(3).unwrap().as_str().parse()?,
        };
        let size = Size {
            width: cap.get(4).unwrap().as_str().parse()?,
            height: cap.get(5).unwrap().as_str().parse()?,
        };
        claims.push(Claim { id, start, size });
    }
    Ok(claims)
}

#[derive(Debug)]
struct Claim {
    id: u32,
    start: Point,
    size: Size,
}

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Size {
    width: u32,
    height: u32,
}

impl fmt::Display for Claim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            return write!(
                f,
                "#{id} @ {x},{y}: {width}x{height}",
                id = self.id,
                x = self.start.x,
                y = self.start.y,
                width = self.size.width,
                height = self.size.height
            );
        }
        for i in 1..=8 {
            for j in 1..=8 {
                let mark = if i > self.start.x
                    && i <= self.start.x + self.size.width
                    && j > self.start.y
                    && j <= self.start.y + self.size.height
                {
                    "#"
                } else {
                    "."
                };
                write!(f, "{}", mark)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_example_1() {
        let input = "\
#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";

        let claims = parse_claims(&input).unwrap();
        for claim in claims {
            println!("{:#}", claim);
            println!("{}", claim);
        }
        assert!(true);
    }

    #[test]
    #[ignore]
    fn test_example_1() {
        let input = "\
#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";
        assert_eq!(4, claimed_area(&input).unwrap());
    }
}
