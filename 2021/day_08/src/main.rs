use std::error::Error;
use std::io::{self, prelude::*};

// type OutputValues = Vec<&str>;

fn parse_line(line: &str) -> Vec<String> {
    let mut points = line.split(" | ");

    let signal_patterns = points.next().unwrap().split(" ").collect::<Vec<&str>>();
    let output_values = points
        .next()
        .unwrap()
        .split(" ")
        .map(|p| p.to_string())
        .collect::<Vec<String>>();

    output_values
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let entries = lines.map(|s| parse_line(&s.unwrap())).collect::<Vec<_>>();

    let sum = entries.iter().fold(0, |acc, entry| {
        // let mut sum = 0;
        // for (i, signal_pattern) in entry.iter().enumerate() {
        //     if signal_pattern == &"1" {
        //         sum += i as u32;
        //     }
        // }
        acc + find_1478_count(&entry.iter().map(|e| e.as_ref()).collect())
    });

    println!("Digits 1,4,7,8 appear {} times", sum);

    // let parsed: Result<Vec<u32>, Box<dyn Error>> =
    //     lines.map(|line| Ok(line?.parse::<u32>()?)).collect();

    // match parsed {
    //     Ok(p) => {
    //         for n in p {
    //             println!("{}", n);
    //         }
    //     }
    //     Err(e) => {
    //         eprintln!("Error parsing file: {}", e);
    //         return Err(e);
    //     }
    // }

    Ok(())
}

fn find_1478_count(segments: &Vec<&str>) -> u32 {
    let count = segments
        .iter()
        .filter(|s| {
            let len = s.len();
            match len {
                2 | 4 | 3 | 7 => true,
                _ => false,
            }
        })
        .count();

    count as u32
}

#[test]
fn test_output_1478_count() {
    let segments = vec!["fdgacbe", "cefdb", "cefbgd", "gcbe"];

    assert_eq!(find_1478_count(&segments), 2);
}
