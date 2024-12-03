use std::io::{self, prelude::*};

use regex::Regex;

type Muls = Vec<(isize, isize)>;

fn parse_for_mul(input: &[String]) -> Muls {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("Invalid regex");

    let muls = input
        .iter()
        .flat_map(|line| {
            re.captures_iter(&line).map(|mul| {
                let num1 = mul[1].parse::<isize>().unwrap();
                let num2 = mul[2].parse::<isize>().unwrap();

                (num1, num2)
            })
        })
        .collect();

    // println!("Found {:?}", muls);

    muls
}

fn sum_muls(muls: &Muls) -> isize {
    muls.iter().map(|(n1, n2)| n1 * n2).sum()
}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    let muls = parse_for_mul(&lines);

    let answer = sum_muls(&muls);

    println!("Adding up the multiplications gives {}", answer);

    Ok(())
}
