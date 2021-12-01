use itertools::Itertools;
use std::error::Error;
use std::io::{self, prelude::*};

fn count_consecutive_increments(depth_measurements: Vec<u32>) -> u32 {
    let mut total = 0u32;

    for (current, next) in depth_measurements.iter().tuple_windows() {
        if next > current {
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
            let larger_measurements = count_consecutive_increments(items);
            println!(
                "Measurements larger than the previous measurement: {}",
                larger_measurements
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
    let count = count_consecutive_increments(vec![1, 2, 3, 2]);

    assert_eq!(count, 2);
}