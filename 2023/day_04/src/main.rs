use std::{
    collections::{HashMap, HashSet},
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
        let winners = self.count_winners();
        Card::score_from_winners(winners as u32)
    }

    pub fn count_winners(&self) -> usize {
        self.winning_numbers
            .intersection(&self.scratched_numbers)
            .count()
    }

    fn score_from_winners(winning_tickets: u32) -> u32 {
        // dbg!(winning_tickets);
        if winning_tickets > 0 {
            2_u32.pow(winning_tickets - 1)
        } else {
            0
        }
    }
}

#[derive(Debug)]
struct ParseCardError {
    message: String,
}

impl std::fmt::Display for ParseCardError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ParseCardError {}

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

fn count_all_cards(cards: &[Card]) -> u32 {
    let mut card_counts = HashMap::new();

    for card in cards {
        *card_counts.entry(card.card_number).or_insert(0) += 1;
        let card_count = card_counts[&card.card_number];

        let winners = card.count_winners();

        let start_dupes = card.card_number + 1;
        let end_dupes = start_dupes + winners as u32;

        for ix in start_dupes..end_dupes {
            *card_counts.entry(ix).or_insert(0) += card_count;
        }
    }

    card_counts.values().sum()
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

    let card_count = count_all_cards(&cards);

    println!("You end up with {card_count} scratchcards.");

    Ok(())
}
