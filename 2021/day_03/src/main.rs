use std::io::{self, prelude::*};
use std::num::ParseIntError;

fn calculate_power_consumption(report_items: &[u16]) -> u32 {
    let max = report_items.iter().max().unwrap();
    let max_ones = u16::BITS - max.leading_zeros();

    let mut result = 0u32;

    for n in 0..max_ones {
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

fn calculate_life_support_rating(report_items: &[u16]) -> u32 {
    let oxygen_rating = find_rating(report_items.to_vec(), true) as u32;
    let c01_scrubber_rating = find_rating(report_items.to_vec(), false) as u32;

    oxygen_rating * c01_scrubber_rating
}

fn find_rating(mut report_items: Vec<u16>, most_common: bool) -> u16 {
    let max = report_items.iter().max().unwrap();
    let max_ones = u16::BITS - max.leading_zeros();

    for n in 0..max_ones {
        let mask = 1 << (max_ones - n - 1);
        // println!("Mask: {:5b}", mask);

        let mut hi_bit_count = 0;

        for item in &report_items {
            if (item & mask) == mask {
                hi_bit_count += 1;
            }
        }

        let check = (report_items.len() as f32 / 2.0).ceil() as i32;

        let use_ones = (hi_bit_count >= check) == most_common;

        // println!("use ones {}", use_ones);
        // report_items.iter().for_each(|f| println!("{:05b}", f));

        report_items = report_items
            .iter()
            .filter(|&i| ((i & mask) == mask) == use_ones)
            .cloned()
            .collect();

        if report_items.len() == 1 {
            let x = report_items[0];
            // println!("{}", x);
            return x;
        }
    }

    0
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

            let rating = calculate_life_support_rating(&report);

            println!("Life support rating of the submarine: {}", rating);
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

#[test]
fn test_calculate_life_support_rating() {
    let rating = calculate_life_support_rating(&vec![0b00100, 0b11110, 0b10110]);

    assert_eq!(rating, 0b11110 * 0b00100);
}
