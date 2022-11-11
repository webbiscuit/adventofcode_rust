use std::cmp::min;
use std::error::Error;
use std::io::{self, prelude::*};
use std::num::ParseIntError;
use std::str::FromStr;

struct Dimensions {
    w: i32,
    l: i32,
    h: i32,
}
impl FromStr for Dimensions {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items: Vec<&str> = s.split_terminator("x").collect();

        let w = items[0].parse::<i32>()?;
        let h = items[1].parse::<i32>()?;
        let l = items[2].parse::<i32>()?;

        Ok(Dimensions { w, h, l })
    }
}

impl Dimensions {
    pub fn wl_side(&self) -> i32 {
        self.w * self.l
    }

    pub fn hl_side(&self) -> i32 {
        self.l * self.h
    }

    pub fn wh_side(&self) -> i32 {
        self.w * self.h
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines = io::stdin().lines();

    let dimensions = lines
        .map(|l| l.unwrap().parse().unwrap())
        .collect::<Vec<Dimensions>>();

    let total_wrapping_paper = dimensions
        .iter()
        .fold(0, |acc, d| acc + calculate_wrapping_paper(d));

    let total_wrapping_ribbon = dimensions
        .iter()
        .fold(0, |acc, d| acc + calculate_ribbon(d));

    println!(
        "Total square feet of wrapping paper is {}",
        total_wrapping_paper
    );
    println!("Total feet of ribbon is {}", total_wrapping_ribbon);

    Ok(())
}

fn calculate_wrapping_paper(dimensions: &Dimensions) -> i32 {
    let smallest = min(
        min(dimensions.wh_side(), dimensions.wl_side()),
        dimensions.hl_side(),
    );
    (dimensions.wl_side() * 2) + (dimensions.wh_side() * 2) + (dimensions.hl_side() * 2) + smallest
}

fn calculate_ribbon(dimensions: &Dimensions) -> i32 {
    let smallest = min(
        min(
            dimensions.w * 2 + dimensions.h * 2,
            dimensions.w * 2 + dimensions.l * 2,
        ),
        dimensions.l * 2 + dimensions.h * 2,
    );

    (dimensions.h * dimensions.w * dimensions.l) + smallest
}

#[test]
fn test_present_2x3x4() {
    let feet = calculate_wrapping_paper(&"2x3x4".parse().unwrap());

    assert_eq!(feet, 58);
}

#[test]
fn test_present_1x1x10() {
    let feet = calculate_wrapping_paper(&("1x1x10").parse().unwrap());

    assert_eq!(feet, 43);
}

#[test]
fn test_ribbon_2x3x4() {
    let feet = calculate_ribbon(&"2x3x4".parse().unwrap());

    assert_eq!(feet, 34);
}

#[test]
fn test_ribbon_1x1x10() {
    let feet = calculate_ribbon(&("1x1x10").parse().unwrap());

    assert_eq!(feet, 14);
}
