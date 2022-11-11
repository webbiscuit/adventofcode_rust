use std::error::Error;
use std::io::{self, prelude::*};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    println!("Santa is on floor {}", count_floors(&input));
    println!("Santa hits the basement on step {}", find_basement(&input));

    Ok(())
}

fn count_floors(line: &str) -> i32 {
    line.chars().fold(0, |acc, c| match c {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => acc,
    })
}

fn find_basement(line: &str) -> i32 {
    let mut floor = 0;

    for (i, c) in line.chars().enumerate() {
        floor = match c {
            '(' => floor + 1,
            ')' => floor - 1,
            _ => floor,
        };
        if floor == -1 {
            return i as i32 + 1;
        }
    }

    0
}

#[test]
fn test_count_floors_0() {
    let count = count_floors("(())");

    assert_eq!(count, 0);
}

#[test]
fn test_basement_floor() {
    let basement_floor = find_basement(")");

    assert_eq!(basement_floor, 1);
}

#[test]
fn test_no_basement() {
    let basement_floor = find_basement("((");

    assert_eq!(basement_floor, 0);
}
