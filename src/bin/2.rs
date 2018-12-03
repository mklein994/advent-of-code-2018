fn main() {
    let input = aoc2018::read_file(2);

    let hash = checksum(&input);
    println!("{}", hash);
}

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
