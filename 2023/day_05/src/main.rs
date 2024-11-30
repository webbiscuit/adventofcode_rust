use ranges::{GenericRange, Ranges};
use std::{
    io::{self, prelude::*},
    str::FromStr,
};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, multispace1, newline, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::pair,
    IResult,
};

#[derive(Debug)]
struct Seed(u64);

#[derive(Debug)]
struct Range {
    source_start: u64,
    destination_start: u64,
    length: u64,
}
impl Range {
    fn lookup(&self, val: u64) -> Option<u64> {
        if val >= self.source_start && val <= self.source_start + self.length {
            let ix = val - self.source_start;
            return Some(self.destination_start + ix);
        }

        None
    }
}

#[derive(Debug)]
struct Mapping {
    source: String,
    destination: String,
    ranges: Vec<Range>,
}
impl Mapping {
    fn lookup(&self, val: u64) -> u64 {
        for range in &self.ranges {
            let new_val = range.lookup(val);

            if new_val.is_some() {
                return new_val.unwrap();
            }
        }

        val
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<Seed>,
    mappings: Vec<Mapping>,
}
impl Almanac {
    fn follow_lookups(&self, start: &str, end: &str) -> Vec<u64> {
        self.seeds
            .iter()
            .map(|s| {
                let mut next_map = start;
                let mut mapped_value = s.0;

                while next_map != end {
                    let map = self.mappings.iter().find(|m| m.source == next_map).unwrap();
                    mapped_value = map.lookup(mapped_value);
                    next_map = &map.destination;
                }

                mapped_value
            })
            .collect()
    }
}

#[derive(Debug)]
struct ParseAlmanacError {
    message: String,
}

impl std::fmt::Display for ParseAlmanacError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ParseAlmanacError {}

impl From<nom::Err<nom::error::Error<&str>>> for ParseAlmanacError {
    fn from(err: nom::Err<nom::error::Error<&str>>) -> Self {
        ParseAlmanacError {
            message: format!("Parsing error: {:?}", err),
        }
    }
}

impl FromStr for Almanac {
    type Err = ParseAlmanacError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_almanac(s)
            .map(|(_, almanac)| almanac)
            .map_err(ParseAlmanacError::from)
    }
}

fn parse_number(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse)(input)
}

fn parse_almanac(input: &str) -> IResult<&str, Almanac> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds) = separated_list1(space1, parse_number)(input)?;
    let (input, _) = pair(newline, newline)(input)?;
    let (input, mappings) = separated_list1(pair(newline, newline), parse_mapping)(input)?;

    Ok((
        input,
        Almanac {
            seeds: seeds.iter().map(|&s| Seed(s)).collect(),
            mappings,
        },
    ))
}

fn parse_almanac_as_seed_ranges(input: &str) -> IResult<&str, Almanac> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds) = separated_list1(space1, parse_number)(input)?;
    let (input, _) = pair(newline, newline)(input)?;
    let (input, mappings) = separated_list1(pair(newline, newline), parse_mapping)(input)?;

    Ok((
        input,
        Almanac {
            seeds: seeds.iter().map(|&s| Seed(s)).collect(),
            mappings,
        },
    ))
}

fn parse_mapping(input: &str) -> IResult<&str, Mapping> {
    let (input, source) = alpha1(input)?;
    let (input, _) = tag("-to-")(input)?;
    let (input, destination) = alpha1(input)?;
    let (input, _) = tag(" map:")(input)?;
    let (input, _) = newline(input)?;
    let (input, ranges) = separated_list1(newline, parse_range)(input)?;

    Ok((
        input,
        Mapping {
            source: source.to_string(),
            destination: destination.to_string(),
            ranges,
        },
    ))
}

fn parse_range(input: &str) -> IResult<&str, Range> {
    let (input, destination_range_start) = parse_number(input)?;
    let (input, _) = space1(input)?;
    let (input, source_range_start) = parse_number(input)?;
    let (input, _) = space1(input)?;
    let (input, range_length) = parse_number(input)?;

    Ok((
        input,
        Range {
            source_start: source_range_start,
            destination_start: destination_range_start,
            length: range_length,
        },
    ))
}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.lock().read_to_string(&mut buf).unwrap();

    let almanac = Almanac::from_str(&buf).unwrap();

    dbg!(&almanac);

    for i in 0..=100 {
        print!("{i}");
        let mut chained_val = i;

        for mapping in &almanac.mappings {
            let i2 = mapping.lookup(chained_val);
            chained_val = i2;
            print! {" {i2}"};
        }

        println!("");

        // let i2 = almanac.mappings[0].lookup(i);
        // println!("{i} {i2}");
    }

    let mut ranges = Vec::new();

    for mapping in &almanac.mappings {
        // ranges.push((mapping.ran)
        for range in &mapping.ranges {
            ranges.push((range.source_start, range.source_start + range.length));
        }
    }

    // dbg!(&ranges);

    let locations = almanac.follow_lookups("seed", "location");

    let location = locations.iter().min().unwrap();

    println!("Should plant in location {location}.");

    Ok(())
}
