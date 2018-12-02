use std::fs::File;
use std::io::Read;

/// Pass in the day as input, and it will read that day's input for you as a file.
pub fn read_file(day: u16) -> String {
    assert!(day > 0 && day < 25, "Not a valid day.");

    let mut contents = String::new();
    let path = format!("{}/input/{}.txt", env!("CARGO_MANIFEST_DIR"), day);
    let mut f = File::open(path).unwrap_or_else(|_| panic!("failed to open file for day {}", day));

    f.read_to_string(&mut contents)
        .expect("failed to read file to string");
    contents
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn invalid_date_over() {
        read_file(300);
    }

    #[test]
    #[should_panic]
    fn invalid_date_under() {
        read_file(0);
    }

    #[test]
    fn input_1() {
        read_file(1);
    }
}
