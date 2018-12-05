use std::fmt;
use std::str::FromStr;

fn main() -> Result<(), Box<std::error::Error>> {
    // let input = aoc2018::read_file(3);
    let input = "\
#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";

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

fn parse_claims(input: &str) -> Result<Vec<Claim>, <Claim as FromStr>::Err> {
    let mut claims: Vec<Claim> = Vec::with_capacity(input.lines().count());

    for line in input.lines() {
        claims.push(Claim::from_str(&line)?);
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

impl FromStr for Claim {
    type Err = std::num::ParseIntError;

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
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut groups = s.split_whitespace();
        let id_str = groups.next().unwrap();
        // skip the '@' sign
        groups.next();
        let start_str = groups.next().unwrap();
        let size_str = groups.next().unwrap();

        let id = id_str[1..].parse()?;
        let start = {
            let mut points = start_str.trim_end_matches(':').split(',');
            Point {
                x: points.next().expect("x point missing").parse::<u32>()?,
                y: points.next().expect("y point missing").parse::<u32>()?,
            }
        };

        let size = {
            let mut size = size_str.split('x');
            Size {
                width: size.next().expect("width missing").parse::<u32>()?,
                height: size.next().expect("height missing").parse::<u32>()?,
            }
        };

        Ok(Claim { id, start, size })
    }
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

        for line in input.lines() {
            let claim = Claim::from_str(&line).unwrap();
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
