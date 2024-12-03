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

fn parse_for_mul_do_dont(input: &[String]) -> Muls {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").expect("Invalid regex");

    let mut enabled = true;

    let muls = input
        .iter()
        .flat_map(|line| {
            re.captures_iter(&line).filter_map(move |mul| {
                println!("{:?}", mul[0].to_string());

                if mul[0].starts_with("do()") {
                    enabled = true;
                    return None;
                }

                if mul[0].starts_with("don't()") {
                    enabled = false;
                    return None;
                }

                if enabled == false {
                    return None;
                }

                if mul[0].starts_with("mul") {
                    let num1 = mul[1].parse::<isize>().unwrap();
                    let num2 = mul[2].parse::<isize>().unwrap();

                    return Some((num1, num2));
                }

                None
            })
        })
        .collect();

    // println!("Found {:?}", muls);
    // < 83942127

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

    let muls = parse_for_mul_do_dont(&lines);

    let answer2 = sum_muls(&muls);

    println!(
        "Adding up the conditional multiplications gives {}",
        answer2
    );

    Ok(())
}
