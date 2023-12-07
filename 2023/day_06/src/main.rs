use std::{
    io::{self, prelude::*},
    str::FromStr,
};

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace1, newline, space1},
    combinator::map_res,
    multi::separated_list1,
    IResult,
};

#[derive(Debug)]
struct Race {
    time: u32,
    record: u32,
}

impl Race {
    fn new(time: u32, record: u32) -> Race {
        Race { time, record }
    }
}

#[derive(Debug)]
struct ParseRaceError {
    message: String,
}

impl std::fmt::Display for ParseRaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ParseRaceError {}

impl From<nom::Err<nom::error::Error<&str>>> for ParseRaceError {
    fn from(err: nom::Err<nom::error::Error<&str>>) -> Self {
        ParseRaceError {
            message: format!("Parsing error: {:?}", err),
        }
    }
}

fn parse_number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

fn parse_times(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, _) = tag("Time:")(input)?;
    let (input, _) = space1(input)?;
    let (input, times) = separated_list1(space1, parse_number)(input)?;

    Ok((input, times))
}

fn parse_distance_records(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, _) = tag("Distance:")(input)?;
    let (input, _) = space1(input)?;
    let (input, distances) = separated_list1(space1, parse_number)(input)?;

    Ok((input, distances))
}

fn parse_races(input: &str) -> IResult<&str, Vec<Race>> {
    let (input, times) = parse_times(input)?;
    let (input, _) = newline(input)?;
    let (input, distances) = parse_distance_records(input)?;

    Ok((
        input,
        times
            .iter()
            .zip(distances)
            .map(|(&t, d)| Race { time: t, record: d })
            .collect(),
    ))
}

fn count_record_beaters(race: &Race) -> u32 {
    // `n^2 - xn + r = 0`
    // -b +/- rt(b^2-4ac) / 2a
    // a = 1
    // b = time
    // c = distance

    // We need to beat the record
    let to_beat = race.record as f32 + 0.1;
    let core = ((race.time as f32 * race.time as f32) - (4.0 * to_beat)).sqrt();
    let max = (((race.time as f32) * 1.0) + core) / 2.0;
    let min = (((race.time as f32) * 1.0) - core) / 2.0;

    let vals = (max.floor()) - (min.floor());
    // let vals = max - min;

    // dbg!(min);
    // dbg!(max);
    // dbg!(vals);

    vals as u32
}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.lock().read_to_string(&mut buf).unwrap();

    let (_, races) = parse_races(&buf).unwrap();

    // dbg!(&races);

    // count_record_beaters(&races[0]);
    // count_record_beaters(&races[1]);

    let record_product: u32 = races.iter().map(count_record_beaters).product();

    println!("Multiplying the ways of winning together makes {record_product}.");

    Ok(())
}
