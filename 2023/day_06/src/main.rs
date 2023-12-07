use std::io::{self, prelude::*};

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    combinator::map_res,
    multi::separated_list1,
    IResult,
};

#[derive(Debug)]
struct Race {
    time: u64,
    record: u64,
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

fn parse_number(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse)(input)
}

fn parse_times(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, _) = tag("Time:")(input)?;
    let (input, _) = space1(input)?;
    let (input, times) = separated_list1(space1, parse_number)(input)?;

    Ok((input, times))
}

fn parse_distance_records(input: &str) -> IResult<&str, Vec<u64>> {
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

fn parse_races_v2(input: &str) -> IResult<&str, Race> {
    let (input, times) = parse_times(input)?;
    let (input, _) = newline(input)?;
    let (input, distances) = parse_distance_records(input)?;

    let big_time = times.into_iter().map(|t| t.to_string()).collect::<String>();
    let big_distance = distances
        .into_iter()
        .map(|t| t.to_string())
        .collect::<String>();

    Ok((
        input,
        Race {
            time: big_time.parse::<u64>().unwrap(),
            record: big_distance.parse::<u64>().unwrap(),
        },
    ))
}

fn count_record_beaters(race: &Race) -> u64 {
    // `n^2 - xn + r = 0`
    // -b +/- rt(b^2-4ac) / 2a
    // a = 1
    // b = time
    // c = distance

    // We need to beat the record
    let to_beat = race.record as f64 + 0.1;
    let core = ((race.time as f64 * race.time as f64) - (4.0 * to_beat)).sqrt();
    let max = (((race.time as f64) * 1.0) + core) / 2.0;
    let min = (((race.time as f64) * 1.0) - core) / 2.0;

    let vals = (max.floor()) - (min.floor());
    // let vals = max - min;

    // dbg!(&race);
    // dbg!(min);
    // dbg!(max);
    // dbg!(vals);

    vals as u64
}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.lock().read_to_string(&mut buf).unwrap();

    let (_, races) = parse_races(&buf).unwrap();

    let record_product: u64 = races.iter().map(count_record_beaters).product();

    println!("Multiplying the ways of winning together makes {record_product}.");

    let (_, race) = parse_races_v2(&buf).unwrap();

    let record2 = count_record_beaters(&race);

    println!("The ways of winning with improved kerning makes {record2}.");

    Ok(())
}
