fn main() {
    let input = aoc2018::read_file(6);
    println!("{}", input);
}

fn part1(input: &str) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";

    #[test]
    fn part1_example() {
        assert_eq!(17, part1(&EXAMPLE_INPUT));
    }
}
