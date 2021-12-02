use std::error::Error;
use std::io::{self, prelude::*};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Direction {
    Forward(u32),
    Up(u32),
    Down(u32),
}

impl FromStr for Direction {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        let mut split = input.split(' ');

        let (dir, value) = (split.next().unwrap(), split.next().unwrap().parse::<u32>());

        match dir {
            "forward" => Ok(Direction::Forward(value.unwrap())),
            "up" => Ok(Direction::Up(value.unwrap())),
            "down" => Ok(Direction::Down(value.unwrap())),
            _ => Err(format!("Cannot parse {}", dir).into()),
        }
    }
}

fn calculate_depth_by_horizontal_position(dirs: &[Direction]) -> u32 {
    let (horizontal_position, depth) =
        dirs.iter()
            .fold((0, 0), |(horizontal_position, depth), d| match d {
                Direction::Forward(x) => (horizontal_position + x, depth),
                Direction::Up(x) => (horizontal_position, depth - x),
                Direction::Down(x) => (horizontal_position, depth + x),
            });

    horizontal_position * depth
}

fn calculate_aimed_depth(dirs: &[Direction]) -> u32 {
    let (_, horizontal_position, depth) =
        dirs.iter()
            .fold((0, 0, 0), |(aim, horizontal_position, depth), d| match d {
                Direction::Forward(x) => (aim, horizontal_position + x, depth + aim * x),
                Direction::Up(x) => (aim - x, horizontal_position, depth),
                Direction::Down(x) => (aim + x, horizontal_position, depth),
            });

    horizontal_position * depth
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let parsed: Result<Vec<Direction>, Box<dyn Error>> = lines
        .map(|line| Direction::from_str(&line.unwrap()))
        .collect();

    match parsed {
        Ok(dirs) => {
            let depth_by_horizontal_position = calculate_depth_by_horizontal_position(&dirs);
            println!(
                "Final horizonal position by final depth: {}",
                depth_by_horizontal_position
            );

            let aimed_depth = calculate_aimed_depth(&dirs);
            println!("Aimed depth: {}", aimed_depth);
        }
        Err(e) => {
            eprintln!("Error parsing file: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
