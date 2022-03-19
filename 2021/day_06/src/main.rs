use std::error::Error;
use std::io::{self, prelude::*};

type FishTimer = i8;

fn count_lanternfish_after_days(fishies: &[FishTimer], days: u32) -> u64 {
    let mut fishy_count = [0; 9];

    for fish in fishies {
        fishy_count[*fish as usize] += 1;
    }

    // println!("{:?}", fishy_count);

    for _ in 0..days {
        fishy_count.rotate_left(1);
        fishy_count[6] += fishy_count[8];
    }

    // println!("{:?}", fishy_count);

    fishy_count.iter().sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let fish_ages = lines
        .next()
        .unwrap()?
        .split(',')
        .map(|a| a.parse::<i8>().unwrap())
        .collect::<Vec<_>>();

    let fishies_after_80_days = count_lanternfish_after_days(&fish_ages, 80);
    println!(
        "After 80 days there are {} lanternfish.",
        fishies_after_80_days
    );

    let fishies_after_256_days = count_lanternfish_after_days(&fish_ages, 256);
    println!(
        "After 256 days there are {} lanternfish.",
        fishies_after_256_days
    );

    Ok(())
}

#[test]
fn test_fish_reproduce() {
    let fishies = vec![3, 4, 3, 1, 2];

    assert_eq!(count_lanternfish_after_days(&fishies, 1), 5);
    assert_eq!(count_lanternfish_after_days(&fishies, 2), 6);
    assert_eq!(count_lanternfish_after_days(&fishies, 18), 26);
}
