use std::{
    collections::HashSet,
    io::{self, prelude::*},
    str::FromStr,
};

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space1},
    combinator::map_res,
    multi::separated_list1,
    IResult,
};

#[derive(Debug)]
struct Card {
    card_number: u32,
    winning_numbers: HashSet<u32>,
    scratched_numbers: HashSet<u32>,
}

impl Card {
    pub fn calculate_score(&self) -> u32 {
        let winners = self
            .winning_numbers
            .intersection(&self.scratched_numbers)
            .count();
        Card::score_from_winners(winners as u32)
    }

    fn score_from_winners(winning_tickets: u32) -> u32 {
        // dbg!(winning_tickets);
        let score = if winning_tickets > 0 {
            2_u32.pow(winning_tickets - 1)
        } else {
            0
        };
        // dbg!(score);

        score
    }
}

#[derive(Debug)]
struct ParseCardError {
    // message: format!("Parsing error: {:?}", err),
    message: String,
}

impl From<nom::Err<nom::error::Error<&str>>> for ParseCardError {
    fn from(err: nom::Err<nom::error::Error<&str>>) -> Self {
        ParseCardError {
            message: format!("Parsing error: {:?}", err),
        }
    }
}

impl FromStr for Card {
    type Err = ParseCardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_card(s)
            .map(|(_, card)| card)
            .map_err(ParseCardError::from)
    }
}

fn parse_number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, _) = tag("Card")(input)?;
    let (input, _) = space1(input)?;
    let (input, card_number) = parse_number(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = space1(input)?;
    let (input, winning_numbers) = separated_list1(space1, parse_number)(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag("|")(input)?;
    let (input, _) = space1(input)?;
    let (input, scratched_numbers) = separated_list1(space1, parse_number)(input)?;
    Ok((
        input,
        Card {
            card_number,
            winning_numbers: HashSet::from_iter(winning_numbers.iter().cloned()),
            scratched_numbers: HashSet::from_iter(scratched_numbers.iter().cloned()),
        },
    ))
}

fn calculate_total_points(cards: &[Card]) -> u32 {
    cards.iter().map(|c| c.calculate_score()).sum()
}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    let cards = lines
        .iter()
        .map(|l| Card::from_str(l).unwrap())
        .collect::<Vec<_>>();

    let points = calculate_total_points(&cards);

    println!("The scratch cards are worth {points} points.");

    Ok(())
}
