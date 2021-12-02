use std::error::Error;
use std::io::{self, prelude::*};

use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Direction {
    Forward(u32),
    Up(u32),
    Down(u32),
}

impl FromStr for Direction {
    type Err = Box<Error>;

    fn from_str(input: &str) -> Result<Direction, Error<&str>> {
        let split = input.split(' ');

        let (dir, value) = (split.next().unwrap(), split.next().unwrap().parse::<u32>());

        match dir {
            "forward" => Ok(Direction::Forward(value.unwrap())),
            "up" => Ok(Direction::Up(value.unwrap())),
            "down" => Ok(Direction::Down(value.unwrap())),
            _ => Err(Box::new("Error")),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let parsed: Result<Vec<Direction>, Box<dyn Error>> = lines
        .map(|line| Direction::from_str(&line.unwrap()))
        .collect();

    // match parsed {
    //     Ok(items) => {
    //         println!("{:?}", items);
    //     }
    //     Err(e) => {
    //         eprintln!("Error parsing file: {}", e);
    //         return Err(e);
    //     }
    // }

    Ok(())
}
