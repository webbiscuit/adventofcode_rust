use std::io::{self, prelude::*};

use regex::Regex;

#[derive(Debug, PartialEq)]
enum Cube {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct Game {
    game_number: u32,
    rounds: Vec<Round>,
}

#[derive(Debug)]
struct Round {
    cube_count: Vec<(Cube, u32)>,
}

impl Round {
    pub fn get_colour_count(&self, colour: Cube) -> u32 {
        let found = self.cube_count.iter().find(|c| c.0 == Cube::Green);

        match found {
            Some((_, count)) => *count,
            None => 0,
        }
    }
}

fn parse_game(line: &str) -> Game {
    let game_split = line.split(": ").collect::<Vec<&str>>();
    let game_regex = Regex::new(r"Game (\d+)").unwrap();
    let game_number = game_regex.captures(game_split[0]).unwrap()[1]
        .parse::<u32>()
        .unwrap();

    let round_split = game_split[1].split("; ").collect::<Vec<&str>>();

    let mut rounds = Vec::new();

    let cube_regex = Regex::new(r"(\d+) (blue|red|green)").unwrap();

    for round in round_split {
        let mut cube_count = Vec::new();
        for cube_split in round.split(", ") {
            let captures = cube_regex.captures(cube_split).unwrap();

            let colour = match &captures[2] {
                "red" => Cube::Red,
                "green" => Cube::Green,
                "blue" => Cube::Blue,
                _ => panic!("Invalid cube colour"),
            };
            let count = captures[1].parse::<u32>().unwrap();
            cube_count.push((colour, count));
        }
        rounds.push(Round { cube_count });
    }

    Game {
        game_number,
        rounds,
    }
}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    println!("Hello, world!");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_parse_game() {
        let game = parse_game("Game 23: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");

        assert_eq!(game.game_number, 23);

        let round = &game.rounds[0];
        assert_eq!(round.get_colour_count(Cube::Red), 4);
        assert_eq!(round.get_colour_count(Cube::Blue), 3);
        assert_eq!(round.get_colour_count(Cube::Green), 0);
    }
}
