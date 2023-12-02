use std::{
    cmp,
    io::{self, prelude::*},
    str::FromStr,
};

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
        self.cube_count
            .iter()
            .find_map(|c| if c.0 == colour { Some(c.1) } else { None })
            .unwrap_or(0)
    }
}
#[derive(Debug, PartialEq, Eq)]
struct ParseError;

impl FromStr for Game {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let game_split = s.split(": ").collect::<Vec<&str>>();
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

        Ok(Game {
            game_number,
            rounds,
        })
    }
}

fn to_valid_games(games: &[Game], max_red: u32, max_green: u32, max_blue: u32) -> Vec<&Game> {
    let mut valid_games = Vec::new();

    for game in games {
        let mut valid_game = true;

        for round in &game.rounds {
            if round.get_colour_count(Cube::Red) > max_red
                || round.get_colour_count(Cube::Green) > max_green
                || round.get_colour_count(Cube::Blue) > max_blue
            {
                valid_game = false;
                break;
            }
        }

        if valid_game {
            valid_games.push(game);
        }
    }

    valid_games
}

fn get_fewest_possible_cubes(game: &Game) -> (u32, u32, u32) {
    let mut min_red = 0;
    let mut min_green = 0;
    let mut min_blue = 0;

    for round in &game.rounds {
        let red_count = round.get_colour_count(Cube::Red);
        let green_count = round.get_colour_count(Cube::Green);
        let blue_count = round.get_colour_count(Cube::Blue);

        min_red = cmp::max(min_red, red_count);
        min_green = cmp::max(min_green, green_count);
        min_blue = cmp::max(min_blue, blue_count);
    }

    (min_red, min_green, min_blue)
}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    let games = lines
        .iter()
        .map(|l| Game::from_str(l).unwrap())
        .collect::<Vec<_>>();
    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;
    let valid_games = to_valid_games(&games, MAX_RED, MAX_GREEN, MAX_BLUE);
    let sum = valid_games.iter().fold(0u32, |acc, g| acc + g.game_number);

    println!("Sum of possible games is {sum}.");

    let fewest_cube_games = games.iter().map(get_fewest_possible_cubes);

    // println!("{:?}", fewest_cube_games.collect::<Vec<_>>());

    let power = fewest_cube_games.fold(0u32, |acc, (r, g, b)| acc + r * g * b);

    println!("Sum of the power set of minimum cubes is {power}.");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_game() {
        let game =
            Game::from_str("Game 23: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").unwrap();

        assert_eq!(game.game_number, 23);

        let round = &game.rounds[0];
        assert_eq!(round.get_colour_count(Cube::Red), 4);
        assert_eq!(round.get_colour_count(Cube::Blue), 3);
        assert_eq!(round.get_colour_count(Cube::Green), 0);
    }
}
