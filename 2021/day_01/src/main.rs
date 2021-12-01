use itertools::Itertools;
use std::error::Error;
use std::io::{self, prelude::*};

fn count_consecutive_increments(depth_measurements: &Vec<u32>) -> u32 {
    let mut total = 0u32;

    for (current, next) in depth_measurements.iter().tuple_windows() {
        if next > current {
            total += 1;
        }
    }

    total
}

fn count_sums_larger_than_three_measurement_window(depth_measurements: &Vec<u32>) -> u32 {
    let mut total = 0u32;

    for (a, _, _, d) in depth_measurements.iter().tuple_windows() {
        // if (b + c + d) > (a + b + c) {
        //     total += 1;
        // }

        if d > a {
            total += 1;
        }
    }

    total
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let parsed: Result<Vec<_>, Box<dyn Error>> = lines
        .map(|line| -> Result<_, Box<dyn Error>> { Ok(line?.parse::<u32>()?) })
        .collect();

    match parsed {
        Ok(items) => {
            let larger_measurements = count_consecutive_increments(&items);
            println!(
                "Measurements larger than the previous measurement: {}",
                larger_measurements
            );

            let larger_measurements_in_window =
                count_sums_larger_than_three_measurement_window(&items);
            println!(
                "Measurements larger in 3 measurement window: {}",
                larger_measurements_in_window
            );
        }
        Err(e) => {
            eprintln!("Error parsing file: {}", e);
            return Err(e);
        }
    }

    Ok(())
}

#[test]
fn test_count_consecutive_increments() {
    let count = count_consecutive_increments(&vec![1, 2, 3, 2]);

    assert_eq!(count, 2);
}

#[test]
fn test_count_sums_larger_than_three_measurement_window() {
    let count = count_sums_larger_than_three_measurement_window(&vec![1, 2, 3, 2]);

    assert_eq!(count, 1);
}
