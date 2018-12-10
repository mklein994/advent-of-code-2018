use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::Ordering;
use std::ops::Range;

use std::str::FromStr;

fn main() {
    let input = aoc2018::read_file(4);

    let part_1_answer = part_1(&input);

    println!("{}", part_1_answer);
}

fn part_1(input: &str) -> u32 {
    let guards = parse_guards(input).unwrap();

    unimplemented!()
}

fn parse_guards(input: &str) -> Result<Vec<Guard>, Box<dyn std::error::Error>> {
    let mut guard_lines: Vec<GuardLine> = Vec::with_capacity(input.lines().count());

    for line in input.lines() {
        guard_lines.push(GuardLine::from_str(line)?);
    }

    guard_lines.sort_unstable_by(|gl1, gl2| gl1.datetime.cmp(&gl2.datetime));
    guard_lines.iter().for_each(|l| println!("{:?}", l));

    unimplemented!()
}

#[derive(Debug)]
struct Guard {
    id: u32,
    sleep_time: Vec<SleepTime>,
}

#[derive(Debug)]
struct SleepTime {
    time: DateTime,
    state: SleepKind,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct DateTime {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

impl std::fmt::Debug for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02} {:02}:{:02}",
            self.year, self.month, self.day, self.hour, self.minute
        )
    }
}

#[derive(Debug)]
enum SleepKind {
    BeginShift { id: u32 },
    Awake,
    Asleep,
}

impl Default for SleepKind {
    fn default() -> Self {
        SleepKind::Awake
    }
}

#[derive(Debug)]
struct GuardLine {
    datetime: DateTime,
    kind: SleepKind,
}

impl FromStr for GuardLine {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            // Inspired by BurntSushi's solution
            static ref RE: Regex = Regex::new(
                r"(?x)
                \[
                    (?P<year>[0-9]{4})-(?P<month>[0-9]{2})-(?P<day>[0-9]{2})
                    \s+
                    (?P<hour>[0-9]{2}):(?P<minute>[0-9]{2})
                \]
                \s+
                (?:Guard\ \#(?P<id>[0-9]+)\ begins\ shift|(?P<sleep>.+))"
            )
            .unwrap();
        }

        let caps = RE.captures(s).expect("couldn't parse event");

        let datetime = DateTime {
            year: caps["year"].parse()?,
            month: caps["month"].parse()?,
            day: caps["day"].parse()?,
            hour: caps["hour"].parse()?,
            minute: caps["minute"].parse()?,
        };

        let kind: SleepKind = if let Some(id) = caps.name("id") {
            SleepKind::BeginShift {
                id: id.as_str().parse()?,
            }
        } else if &caps["sleep"] == "falls asleep" {
            SleepKind::Asleep
        } else if &caps["sleep"] == "wakes up" {
            SleepKind::Awake
        } else {
            panic!("Couldn't determine sleep kind")
        };

        Ok(GuardLine { datetime, kind })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

    #[test]
    fn test_part_1_example() {
        assert_eq!(240, part_1(&INPUT));
    }
}
