use lazy_static::lazy_static;
use regex::Regex;
use std::collections::BTreeMap;
use std::str::FromStr;

fn main() {
    // let input = aoc2018::read_file(9);
    let input = "9 players; last marble is worth 25 points";
    // println!("{}", input);

    let high_score = part1(&input);
    println!("{}", high_score);
}

type Players = BTreeMap<u32, u32>;

fn part1(input: &str) -> u32 {
    let settings = Settings::from_str(input).unwrap();

    let mut players: Players = BTreeMap::new();
    (1..=settings.player_count).for_each(|i| {
        players.entry(i).or_default();
    });

    let mut circle: Vec<u32> = vec![0];

    let mut current_marble = 0;

    println!(
        "start:\n\tcircle: {:?}\n\tcurrent_marble: {}",
        circle, current_marble
    );

    for marble in 1..=settings.last_marble {
        println!("{}", "-".repeat(10));
        println!("marble: {}", marble);

        match circle.get(current_marble + 1) {
            Some(_) => {
                circle.insert((current_marble + 1) as usize, marble);
                current_marble = marble as usize;
            }
            None => {
                circle.push(marble);
                // current_marble = marble as usize;
            }
        }

        println!("current_marble: {}", current_marble);
        println!("{:?}", circle);
    }

    // println!("{:?}", circle);

    // players.iter().for_each(|p| println!("{:?}", p));

    unimplemented!()
}

#[derive(Debug)]
struct Settings {
    player_count: u32,
    last_marble: u32,
}

impl FromStr for Settings {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?P<count>[0-9]+) players; last marble is worth (?P<points>[0-9]+) points"
            )
            .unwrap();
        }

        let caps = RE.captures(s).unwrap();

        Ok(Self {
            player_count: caps["count"].parse()?,
            last_marble: caps["points"].parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test {
        ($name:ident, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let input = $input;
                assert_eq!($expected, part1(input));
            }
        };
    }

    test!(example_0, "9 players; last marble is worth 25 points", 32);

    test!(
        example_1,
        "10 players; last marble is worth 1618 points",
        8317
    );
    test!(
        example_2,
        "13 players; last marble is worth 7999 points",
        146373
    );
    test!(
        example_3,
        "17 players; last marble is worth 1104 points",
        2764
    );
    test!(
        example_4,
        "21 players; last marble is worth 6111 points",
        54718
    );
    test!(
        example_5,
        "30 players; last marble is worth 5807 points",
        37305
    );

    #[test]
    #[ignore]
    fn test_given_input() {
        let input = include_str!("../../input/9.txt");
        assert_eq!(0, part1(input));
    }
}
