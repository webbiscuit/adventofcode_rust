use std::{
    collections::HashMap,
    io::{self, prelude::*},
};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    multi::separated_list1,
    IResult,
};

#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Right,
}

type LocationName = String;

#[derive(Debug)]
struct Location {
    name: LocationName,
    left: LocationName,
    right: LocationName,
}

#[derive(Debug)]
struct Map {
    locations: Vec<Location>,
    directions: Vec<Direction>,
}

fn parse_location_name(input: &str) -> IResult<&str, LocationName> {
    let (input, location) = alpha1(input)?;
    Ok((input, location.to_owned()))
}

fn parse_directions(input: &str) -> IResult<&str, Vec<Direction>> {
    let (input, direction_string) = alpha1(input)?;

    let directions = direction_string
        .chars()
        .map(|c| {
            if c == 'L' {
                Direction::Left
            } else {
                Direction::Right
            }
        })
        .collect();

    Ok((input, directions))
}

fn parse_location(input: &str) -> IResult<&str, Location> {
    let (input, source) = parse_location_name(input)?;
    let (input, _) = tag(" = (")(input)?;
    let (input, left) = parse_location_name(input)?;
    let (input, _) = tag(", ")(input)?;
    let (input, right) = parse_location_name(input)?;
    let (input, _) = tag(")")(input)?;

    Ok((
        input,
        Location {
            name: source,
            left,
            right,
        },
    ))
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    let (input, directions) = parse_directions(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;
    let (input, locations) = separated_list1(newline, parse_location)(input)?;

    Ok((
        input,
        Map {
            directions,
            locations,
        },
    ))
}

fn count_steps(map: &Map, start: &str, end: &str) -> usize {
    let location_lookup: HashMap<String, &Location> =
        HashMap::from_iter(map.locations.iter().map(|l| (l.name.clone(), l)));

    let mut visited_locations = Vec::new();

    let mut location_name = start;

    for dir in map.directions.iter().cycle() {
        if location_name == end {
            break;
        }

        let location = *location_lookup.get(location_name).unwrap();

        if *dir == Direction::Left {
            location_name = &location.left;
        } else {
            location_name = &location.right;
        }

        visited_locations.push(location);
    }

    // dbg!(&visited_locations);

    visited_locations.len()
}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.lock().read_to_string(&mut buf).unwrap();

    let (_, map) = parse_map(&buf).unwrap();

    // dbg!(&map);

    let steps = count_steps(&map, "AAA", "ZZZ");

    println!("There are {steps} steps to reach ZZZ.");

    Ok(())
}
