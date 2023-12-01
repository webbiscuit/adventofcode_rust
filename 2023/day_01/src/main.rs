use std::io::{self, prelude::*};

fn find_calibration_value(line: &str) -> u32 {
    let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
    let first = digits.first().unwrap_or(&0);
    let last = digits.last().unwrap_or(&0);

    first * 10 + last
}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let calibration_values: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    let sum = calibration_values
        .iter()
        .fold(0u32, |acc, v| acc + find_calibration_value(v));

    println!("The sum of calibration values is {sum}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("1abc2", 12)]
    #[test_case("pqr3stu8vwx", 38)]
    #[test_case("a1b2c3d4e5f", 15)]
    #[test_case("treb7uchet", 77)]
    fn test_finding_calibration_values(line: &str, expected: u32) {
        let calibration_value = find_calibration_value(line);

        assert_eq!(calibration_value, expected);
    }
}
