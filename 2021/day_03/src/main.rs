use std::io::{self, prelude::*};
use std::num::ParseIntError;

fn calculate_power_consumption(report_items: &[u16]) -> u32 {
    let max = report_items.iter().max().unwrap();
    let max_ones = u16::BITS - max.leading_zeros();

    let mut result = 0u32;

    for n in 0..=max_ones {
        let mut hi_bit_count = 0;

        for i in report_items {
            if (i & 1 << n) == (1 << n) {
                hi_bit_count += 1;
            }

            if hi_bit_count > report_items.len() / 2 {
                result |= 1 << n;
            }
        }
    }

    let mut mask = 0u32;

    for n in 0..max_ones {
        mask |= 1 << n;
    }

    result * (result ^ mask)
}

fn main() -> Result<(), ParseIntError> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let parsed: Result<Vec<u16>, ParseIntError> = lines
        .map(|line| u16::from_str_radix(&line.unwrap(), 2))
        .collect();

    match parsed {
        Ok(report) => {
            let power = calculate_power_consumption(&report);

            println!("Power consumption of the submarine: {}", power);
        }
        Err(e) => {
            eprintln!("Error parsing file: {}", e);
            return Err(e);
        }
    }

    Ok(())
}

#[test]
fn test_calculate_power_consumption() {
    let count = calculate_power_consumption(&vec![0b00100, 0b11110, 0b10110]);

    assert_eq!(count, 0b10110 * 0b01001);
}
