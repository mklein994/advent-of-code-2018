use std::collections::HashSet;

fn main() {
    let input = aoc2018::read_file(2);

    let hash = checksum(&input);
    println!("{}", hash);
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

    for line in input.lines().map(|c| c.chars()) {
        let (a, b): (u32, u32) = line.fold((0, 0), |count: (u32, u32), i: char| {
            count
        });

        two_count += a;
        three_count += b;
    }

    two_count * three_count
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
}
