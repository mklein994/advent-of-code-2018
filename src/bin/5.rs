fn main() {
    let input = aoc2018::read_file(5);
    println!("{}", input);

    let units = polymer_units(&input);
    println!("{}", units);
}

fn polymer_units(input: &str) -> u32 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let input = "dabAcCaCBAcCcaDA";

        assert_eq!(10, polymer_units(input));
    }

    #[test]
    #[ignore]
    fn part_1_test_input() {
        let input = include_str!("../../input/5.txt");
        assert_eq!(0, polymer_units(&input));
    }
}
