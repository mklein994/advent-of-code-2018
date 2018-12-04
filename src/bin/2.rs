use std::collections::BTreeMap;

fn main() {
    let input = aoc2018::read_file(2);

    let hash = checksum(&input);
    println!("{}", hash);

    let common_chars = common_chars(&input);
    println!("{}", common_chars);
}

/// | Example | Two | Three |
/// | ------- | --: | ----: |
/// | `abcdef` contains no letters that appear exactly two or three times.  |   |   |
/// | `bababc` contains two `a` and three `b`, so it counts for both.       | ✓ | ✓ |
/// | `abbcde` contains two `b`, but no letter appears exactly three times. | ✓ |   |
/// | `abcccd` contains three `c`, but no letter appears exactly two times. |   | ✓ |
/// | `aabcdd` contains two `a` and two `d`, but it only counts once.       | ✓ |   |
/// | `abcdee` contains two `e`.                                            | ✓ |   |
/// | `ababab` contains three `a` and three `b`, but it only counts once.   |   | ✓ |
///
/// `4 * 3 = 12`
fn checksum(input: &str) -> u32 {
    let mut two_count: u32 = 0;
    let mut three_count: u32 = 0;

    for line in input.lines() {
        let mut count = BTreeMap::new();

        for c in line.chars() {
            *count.entry(c).or_insert(0) += 1u32;
        }

        let mut two_counted = false;
        let mut three_counted = false;

        for (_, count) in count {
            if !two_counted && count == 2 {
                two_count += 1;
                two_counted = true;
            } else if !three_counted && count == 3 {
                three_count += 1;
                three_counted = true;
            }
        }
    }

    two_count * three_count
}

#[derive(Debug, Default)]
struct Occurrence {
    count: u32,
    three: bool,
    two: bool,
}

fn common_chars(input: &str) -> String {
    for source_line in input.lines() {
        for test_line in input.lines().filter(|l| *l != source_line) {
            let comparisons = source_line
                .chars()
                .zip(test_line.chars())
                .filter_map(|(s, t)| if s == t { Some(s) } else { None })
                .collect::<String>();
            if comparisons.len() == source_line.len() - 1 {
                return comparisons;
            }
        }
    }

    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let input = "\
abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab";
        assert_eq!(12, checksum(&input));
    }

    #[test]
    fn part_1_test_input() {
        let input = include_str!("../../input/2.txt");
        // correct
        assert_eq!(4980, checksum(&input));
    }

    #[test]
    fn part_2_example() {
        let input = "\
abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz";
        assert_eq!("fgij", common_chars(&input));
    }
}
