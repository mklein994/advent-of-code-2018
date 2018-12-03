use std::collections::HashSet;

fn main() {
    let input = aoc2018::read_file(1);
    let freq = get_freq(&input);

    println!("{}", freq);

    let calib_freq = calibrate_freq(&input);

    println!("{}", calib_freq);
}

fn get_freq(input: &str) -> i32 {
    input.lines().map(|c| c.parse::<i32>().unwrap()).sum()
}

fn calibrate_freq(input: &str) -> i32 {
    let mut seen = HashSet::new();
    let mut sum = 0;
    loop {
        for i in input.lines().map(|c| c.parse::<i32>().unwrap()) {
            seen.insert(sum);
            sum += i;
            if seen.contains(&sum) {
                return sum;
            }
        }
    }
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

    #[test]
    fn part_2_example() {
        assert_eq!(2, calibrate_freq("+1\n-2\n+3\n+1"));
    }

    #[test]
    fn part_2_test_1() {
        assert_eq!(0, calibrate_freq("+1\n-1"));
    }

    #[test]
    fn part_2_test_2() {
        assert_eq!(10, calibrate_freq("+3\n+3\n+4\n-2\n-4"));
    }

    #[test]
    fn part_2_test_3() {
        assert_eq!(5, calibrate_freq("-6\n+3\n+8\n+5\n-6"));
    }

    #[test]
    fn part_2_test_4() {
        assert_eq!(14, calibrate_freq("+7\n+7\n-2\n-7\n-4"))
    }

    #[test]
    fn test_part_2() {
        assert_eq!(481, calibrate_freq(include_str!("../../input/1.txt")));
    }
}
