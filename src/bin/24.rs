use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::Ordering;
use std::error::Error;
use std::str::FromStr;

fn main() {
    // let input = aoc2018::read_file(24);

    let input = "\
Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with \
an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, \
slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack \
that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, \
cold) with an attack that does 12 slashing damage at initiative 4";

    let unit_count = part1(&input).unwrap();
    println!("{}", unit_count);
}

fn part1(input: &str) -> Result<usize, Box<Error>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?x)
            (?P<unit_count>\d+)(?-x: units each with )
            (?P<hit_points>\d+)(?-x: hit points )
            (?:\(
                (?:(?P<defense_kind>weak|immune)\ to\ (?P<defense>(?:\w+(?-x:, )?)+))?
                (?:;\ )?
                (?:(?P<other_defense_kind>weak|immune)\ to\ (?P<other_defense>(?:\w+(?-x:, )?)+))?
            \)\ )?
            (?-x:with an attack that does )
            (?P<damage>\d+)\ (?P<attack_kind>\w+)(?-x: damage at initiative )
            (?P<initiative>\d+)"
        )
        .unwrap();
    }

    // Skip the heading, and stop at the next empty line.
    for line in input.lines().skip(1).take_while(|l| l.len() != 0) {
        let caps = RE
            .captures(line)
            .expect("unknown immune system group captures");
        println!("{:#?}", caps);
    }

    // Skip until the next section, then skip the blank line and the heading.
    for line in input.lines().skip_while(|l| l.len() != 0).skip(2) {
        let caps = RE.captures(line).expect("unknown infection group captures");
        println!("{:#?}", caps);
    }

    Ok(0)
}

//TODO: use BTreeSet instead of Vec
type Armies = Vec<Group>;

#[derive(Debug)]
enum Allegiance {
    ImmuneSystem,
    Infection,
}

impl Default for Allegiance {
    fn default() -> Self {
        Allegiance::ImmuneSystem
    }
}

#[derive(Debug, Default)]
struct Group {
    allegiance: Allegiance,
    unit_count: u32,
    initiative: u32,
    immunities: Vec<AttackKind>,
    weaknesses: Vec<AttackKind>,
    attack_damage: u32,
    attack_kind: AttackKind,
}

#[derive(Debug, Default)]
struct AttackKind(String);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let input = "\
Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with \
an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, \
slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack \
that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, \
cold) with an attack that does 12 slashing damage at initiative 4";

        assert_eq!(5216, part1(&input).unwrap());
    }
}
