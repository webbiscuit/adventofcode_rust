use std::{
    collections::HashSet,
    env::args,
    io::{self, BufRead},
};

use regex::Regex;

struct Sensor {
    x: isize,
    y: isize,
    closest_beacon: (isize, isize),
    distance_to_beacon: usize,
}

impl Sensor {
    fn new(x: isize, y: isize, closest_beacon: (isize, isize)) -> Self {
        let distance_to_beacon = ((x - closest_beacon.0).abs() + (y - closest_beacon.1).abs())
            .try_into()
            .unwrap();

        Self {
            x,
            y,
            closest_beacon,
            distance_to_beacon,
        }
    }
}

fn main() {
    let row: isize = args()
        .nth(1)
        .expect("must provide a row number")
        .parse()
        .unwrap();

    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    let sensor_re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();

    let mut sensors = Vec::new();

    for line in lines {
        let captures = sensor_re.captures(&line).unwrap();
        let x = captures[1].parse::<isize>().unwrap();
        let y = captures[2].parse::<isize>().unwrap();
        let closest_beacon_x = captures[3].parse::<isize>().unwrap();
        let closest_beacon_y = captures[4].parse::<isize>().unwrap();

        let sensor = Sensor::new(x, y, (closest_beacon_x, closest_beacon_y));

        sensors.push(sensor);
    }

    let max_distance = sensors.iter().map(|s| s.distance_to_beacon).max().unwrap();

    let min_x = sensors.iter().map(|s| s.x).min().unwrap() - max_distance as isize;
    let max_x = sensors.iter().map(|s| s.x).max().unwrap() + max_distance as isize;

    // println!("{} - {}", min_x, max_x);

    let mut no_beacon_spaces = HashSet::<(isize, isize)>::new();
    let mut beacon_locations: HashSet<(isize, isize)> = sensors
        .iter()
        .map(|s| (s.closest_beacon.0, s.closest_beacon.1))
        .collect::<HashSet<_>>();

    for x in min_x..=max_x {
        for sensor in &sensors {
            let distance = sensor.distance_to_beacon;

            if sensor.x.abs_diff(x) + sensor.y.abs_diff(row) <= distance {
                let coord = (x, row);

                if !beacon_locations.contains(&coord) {
                    no_beacon_spaces.insert((x, row));
                }
            }
        }

        // println!("{}: {:?}", x, closest_beacon);
    }

    // println!("{}: {:?}", row, no_beacon_spaces);

    let no_beacons = no_beacon_spaces.len();

    println!(
        "On row {}, there are {} positions where a beacon can not be present.",
        row, no_beacons
    )
}
