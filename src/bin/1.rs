fn main() {
    let input = aoc2018::read_file(1);
    let freq = get_freq(&input);

    println!("{}", freq);
}

fn get_freq(input: &str) -> i32 {
    input.lines().map(|c| c.parse::<i32>().unwrap()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(3, get_freq("+1\n+1\n+1"));
    }

    #[test]
    fn test_example_2() {
        assert_eq!(0, get_freq("+1\n+1\n-2"));
    }

    #[test]
    fn test_example_3() {
        assert_eq!(-6, get_freq("-1\n-2\n-3"));
    }

    #[test]
    fn test_part_1() {
        assert_eq!(402, get_freq(include_str!("../../input/1.txt")));
    }
}
