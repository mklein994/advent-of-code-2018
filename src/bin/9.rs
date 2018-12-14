use std::str::FromStr;

fn main() {
    // let input = aoc2018::read_file(9);
    let input = "10 players; last marble is worth 1618 points";
    // println!("{}", input);

    let high_score = part1(&input);
    println!("{}", high_score);
}

fn part1(input: &str) -> u32 {
    let settings = Settings::from_str(input).unwrap();
    unimplemented!()
}

#[derive(Debug)]
struct Settings {
    player_count: usize,
    last_marble: usize,
}

impl FromStr for Settings {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        unimplemented!()
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