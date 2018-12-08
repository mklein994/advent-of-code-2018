use std::str::FromStr;

fn main() {
    let input = aoc2018::read_file(4);
    println!("{}", input);

    let part_1_answer = part_1(&input);

    println!("{}", part_1_answer);
}

fn part_1(input: &str) -> u32 {
    unimplemented!()
}

fn parse_guards(input: &str) -> Result<Vec<Guard>, Box<dyn std::error::Error>> {
    let mut guards: Vec<Guard> = Vec::with_capacity(input.lines().count());

    for line in input.lines() {
        guards.push(Guard::from_str(line)?);
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
    time: DateTime,
    state: SleepKind,
}

#[derive(Debug)]
struct DateTime {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

#[derive(Debug)]
enum SleepKind {
    Awake,
    Asleep,
}

impl Default for SleepKind {
    fn default() -> Self {
        SleepKind::Awake
    }
}

impl FromStr for Guard {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        unimplemented!()
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
