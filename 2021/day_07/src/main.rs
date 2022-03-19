use std::error::Error;
use std::io::{self, prelude::*};

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let crab_positions = lines
        .next()
        .unwrap()?
        .split(',')
        .map(|a| a.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let fuel_used = find_fuel_used_in_optimal_crab_position(&crab_positions);
    println!("Fuel used to align to best position: {}", fuel_used);

    let fuel_used2 = find_incremental_fuel_used_in_optimal_crab_position(&crab_positions);
    println!(
        "Incremental fuel used to align to best position: {}",
        fuel_used2
    );

    Ok(())
}

fn median(nums: &[i32]) -> i32 {
    let mut copy = nums.to_vec();
    copy.sort_unstable();
    copy[copy.len() / 2]
}

fn mean(nums: &[i32]) -> f32 {
    let x = nums.iter().sum::<i32>() as f32 / nums.len() as f32;
    x
}

fn find_fuel_used_from_position(crabs: &[i32], position: i32) -> i32 {
    let fuel_used = crabs.iter().fold(0, |acc, &crab| {
        let distance = (crab - position).abs();

        acc + distance
    });

    fuel_used
}

fn find_incremental_fuel_used_from_position(crabs: &[i32], position: i32) -> i32 {
    let fuel_used = crabs.iter().fold(0, |acc, &crab| {
        let distance = (crab - position).abs();
        let fuel_used_for_crab = (distance * (distance + 1)) / 2;

        acc + fuel_used_for_crab
    });

    fuel_used
}

fn find_fuel_used_in_optimal_crab_position(crabs: &[i32]) -> i32 {
    let best_position = median(crabs);
    find_fuel_used_from_position(crabs, best_position)
}

fn find_incremental_fuel_used_in_optimal_crab_position(crabs: &[i32]) -> i32 {
    let best_position = mean(crabs);
    // println!("Best position: {}", best_position);
    let floor = find_incremental_fuel_used_from_position(crabs, best_position.floor() as i32);
    let ceil = find_incremental_fuel_used_from_position(crabs, best_position.ceil() as i32);

    floor.min(ceil)
}

#[test]
fn test_optimal_crab_position() {
    let crabs = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    assert_eq!(find_fuel_used_from_position(&crabs, 2), 37);
    assert_eq!(find_fuel_used_from_position(&crabs, 1), 41);
    assert_eq!(find_fuel_used_from_position(&crabs, 3), 39);
    assert_eq!(find_fuel_used_from_position(&crabs, 10), 71);

    assert_eq!(find_fuel_used_in_optimal_crab_position(&crabs), 37);
}

#[test]
fn test_optimal_crab_position_with_fuel_increments() {
    let crabs = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    assert_eq!(find_incremental_fuel_used_from_position(&crabs, 5), 168);
    assert_eq!(find_incremental_fuel_used_from_position(&crabs, 2), 206);

    assert_eq!(
        find_incremental_fuel_used_in_optimal_crab_position(&crabs),
        168
    );
}
