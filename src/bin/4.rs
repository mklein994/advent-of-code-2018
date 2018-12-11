use chrono::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

type Guards = HashMap<u32, Vec<SleepTime>>;

fn main() {
    // let input = aoc2018::read_file(4);
    let input = "\
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

    let part_1_answer = part_1(&input);

    println!("{}", part_1_answer);
}

fn part_1(input: &str) -> u32 {
    let guards = parse_guards(input).unwrap();

    // zip up offset by one, and sum up the differences.
    let sleepiest = guards
        .iter()
        .map(|(id, times)| {
            let total: u32 = times
                .iter()
                .zip(&times[1..])
                .filter_map(|(a, b)| {
                    if a.is_asleep {
                        Some((b.time - a.time).num_minutes() as u32)
                    } else {
                        None
                    }
                })
                .sum();

            (id, total)
        })
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap();

    println!("{:?}", sleepiest);

    sleepiest.0 * sleepiest.1
}

fn parse_guards(input: &str) -> Result<Guards, Box<dyn std::error::Error>> {
    let mut guard_lines: Vec<GuardLine> = Vec::with_capacity(input.lines().count());

    for line in input.lines() {
        guard_lines.push(GuardLine::from_str(line)?);
    }

    guard_lines.sort_unstable_by(|gl1, gl2| gl1.datetime.cmp(&gl2.datetime));

    let mut guards: Guards = HashMap::new();
    let mut current_id = None;

    for line in guard_lines {
        if let SleepKind::BeginShift { id } = line.kind {
            current_id = Some(id);
        }

        let sleep = match line.kind {
            SleepKind::BeginShift { .. } | SleepKind::Awake => SleepTime {
                time: line.datetime,
                is_asleep: false,
            },
            SleepKind::Asleep => SleepTime {
                time: line.datetime,
                is_asleep: true,
            },
        };

        guards
            .entry(current_id.expect("line found without an associated ID"))
            .or_default()
            .push(sleep);
    }

    Ok(guards)
}

#[derive(Debug)]
struct Guard {
    id: u32,
    sleep_time: Vec<SleepTime>,
}

#[derive(Debug)]
struct SleepTime {
    time: NaiveDateTime,
    is_asleep: bool,
}

impl Default for SleepTime {
    fn default() -> Self {
        SleepTime {
            time: NaiveDate::from_ymd(0, 0, 0).and_hms(0, 0, 0),
            is_asleep: false,
        }
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
    datetime: NaiveDateTime,
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
                    (?P<datetime>[0-9]{4}-[0-9]{2}-[0-9]{2}
                    \s+
                    [0-9]{2}:[0-9]{2})
                \]
                \s+
                (?:Guard\ \#(?P<id>[0-9]+)\ begins\ shift|(?P<sleep>.+))"
            )
            .unwrap();
        }

        let caps = RE.captures(s).expect("couldn't parse event");

        let datetime = NaiveDateTime::parse_from_str(&caps["datetime"], "%Y-%m-%d %H:%M")?;

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
