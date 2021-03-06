fn main() {
    // let input = "dabAcCaCBAcCcaDA";
    let input = aoc2018::read_file(5);

    let polymer_count = part1(&input);
    println!("{}", polymer_count);

    let shortest_polymer = part2(&input);
    println!("{}", shortest_polymer);
}

fn part1(input: &str) -> usize {
    polymer_units(&input).len()
}

fn part2(input: &str) -> usize {
    let mut count = vec![];
    for i in b'a'..=b'z' {
        let polymer = input.replace(|c: char| c.eq_ignore_ascii_case(&char::from(i)), "");
        count.push(part1(&polymer));
    }

    *count.iter().min().unwrap()
}

fn polymer_units(input: &str) -> String {
    let mut polymer = input.as_bytes().to_vec();
    let mut reacted_polymer = vec![];

    // Adapted from BurntSushi's solution:
    // https://github.com/BurntSushi/advent-of-code/blob/master/aoc05/src/main.rs
    loop {
        let mut i = 1;
        let mut reacted = false;
        while i < polymer.len() {
            if react(polymer[i - 1], polymer[i]) {
                reacted = true;
                i += 2;
                continue;
            }

            reacted_polymer.push(polymer[i - 1]);
            i += 1;
        }

        // If we're on the last one, add it to the list since it doesn't have anything to react
        // with.
        if i % 2 == 0 && i == polymer.len() {
            reacted_polymer.push(polymer[i - 1]);
        }

        // Swap the reacted polymer the current one and clear the next one.
        std::mem::swap(&mut polymer, &mut reacted_polymer);
        reacted_polymer.clear();

        // Break with the resulting string if no reactions are left.
        if !reacted {
            break String::from_utf8(polymer).expect("couldn't parse bytes as string");
        }
    }
}

/// The pairs react only if `a` equals `b` and they have opposite casing.
///
/// # Example
/// | Pair | React?  |
/// | ---- | ------  |
/// | `Aa` | `true`  |
/// | `aA` | `true`  |
/// | `AA` | `false` |
/// | `aa` | `false` |
/// | `Ab` | `false` |
/// | `aB` | `false` |
fn react(a: u8, b: u8) -> bool {
    if a.eq_ignore_ascii_case(&b) {
        a.is_ascii_lowercase() ^ b.is_ascii_lowercase()
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../input/5.txt");

    #[test]
    fn test_react() {
        for (a, b, expected) in &[
            (b'A', b'A', false),
            (b'A', b'B', false),
            (b'A', b'a', true),
            (b'a', b'B', false),
            (b'a', b'a', false),
            (b'a', b'b', false),
        ] {
            assert_eq!(*expected, react(*a, *b));
            assert_eq!(*expected, react(*b, *a));
        }
    }

    #[test]
    fn part_1_example_reaction() {
        let input = "dabAcCaCBAcCcaDA";
        assert_eq!("dabCBAcaDA", polymer_units(input));
    }

    #[test]
    fn part_1_example() {
        let input = "dabAcCaCBAcCcaDA";
        assert_eq!(10, part1(input));
    }

    #[test]
    fn part_1_test_input() {
        // correct
        assert_eq!(9822, part1(INPUT));
    }

    #[test]
    fn test_part_2() {
        let input = "dabAcCaCBAcCcaDA";

        assert_eq!(4, part2(input));
    }

    #[test]
    fn test_part_2_example() {
        let input = "dabAcCaCBAcCcaDA";

        assert_eq!(4, part2(input));
    }

    #[test]
    fn test_part_2_input() {
        // correct
        assert_eq!(5726, part2(&INPUT));
    }
}
