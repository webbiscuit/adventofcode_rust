use std::io::{self, prelude::*};

use fancy_regex::Regex;

fn find_calibration_value(line: &str) -> u32 {
    let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
    let first = digits.first().unwrap_or(&0);
    let last = digits.last().unwrap_or(&0);

    first * 10 + last
}

fn number_word_to_number(word: &str) -> Option<u8> {
    match word {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}

fn find_calibration_value_using_written_numbers_too(line: &str) -> u32 {
    let re = Regex::new(r"(?=(one|two|three|four|five|six|seven|eight|nine))").unwrap();
    let converted_line = re.replace_all(line, |caps: &fancy_regex::Captures| {
        let capture = &caps[1];
        number_word_to_number(capture).unwrap().to_string()
    });

    find_calibration_value(&converted_line)
}

// fn find_calibration_value_using_written_numbers_too(line: &str) -> u32 {
//     let mut converted_line = line.to_string();
//     let words_to_find = [
//         "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
//     ];

//     // Sort by order these thing appear
//     let found_order = words_to_find.map(|w| (w, converted_line.find(w)));
//     let found_order = found_order
//         .iter()
//         .filter(|(_, ix)| ix.is_some())
//         .map(|(s, ix)| (s, ix.unwrap()))
//         .collect::<Vec<_>>();

//     if !found_order.is_empty() {
//         let min = found_order.iter().min_by_key(|(_, ix)| ix).unwrap();
//         let word = min.0.to_string();
//         let digit = number_word_to_number(&word).unwrap().to_string();

//         converted_line.insert(min.1, digit.chars().next().unwrap());
//     }

//     let found_order = words_to_find.map(|w| (w, converted_line.rfind(w)));
//     let found_order = found_order
//         .iter()
//         .filter(|(_, ix)| ix.is_some())
//         .map(|(s, ix)| (s, ix.unwrap()))
//         .collect::<Vec<_>>();

//     if !found_order.is_empty() {
//         let max = found_order.iter().max_by_key(|(_, ix)| ix).unwrap();
//         let word = max.0.to_string();
//         let digit = number_word_to_number(&word).unwrap().to_string();

//         converted_line.insert(max.1 + 1, digit.chars().next().unwrap());
//     }

//     // println!("Converted line: {} to {}", line, converted_line);

//     find_calibration_value(&converted_line)
// }

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let calibration_values: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    let sum = calibration_values
        .iter()
        .fold(0u32, |acc, v| acc + find_calibration_value(v));

    let sum2 = calibration_values.iter().fold(0u32, |acc, v| {
        acc + find_calibration_value_using_written_numbers_too(v)
    });

    println!("The sum of calibration values is {sum}");
    println!("The sum including written calibration values is {sum2}");

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

    #[test_case("two1nine", 29)]
    #[test_case("eightwothree", 83)]
    #[test_case("abcone2threexyz", 13)]
    #[test_case("xtwone3four", 24)]
    #[test_case("4nineeightseven2", 42)]
    #[test_case("zoneight234", 14)]
    #[test_case("7pqrstsixteen", 76)]
    #[test_case("eighteightsrfcxtvg7three1two9nineeightwolqn", 82)]
    #[test_case("eighthree", 83)]
    #[test_case("sevenine", 79)]
    fn test_finding_written_calibration_values(line: &str, expected: u32) {
        let calibration_value = find_calibration_value_using_written_numbers_too(line);

        assert_eq!(calibration_value, expected);
    }
}
