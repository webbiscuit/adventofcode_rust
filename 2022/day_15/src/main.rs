use std::{
    collections::{HashMap, HashSet},
    env::args,
    fmt::Debug,
    io::{self, BufRead},
};

use regex::Regex;

#[derive(Debug)]
struct Sensor {
    x: isize,
    y: isize,
    closest_beacon: (isize, isize),
    distance_to_beacon: usize,
    bounding_x_range: (isize, isize),
    bounding_y_range: (isize, isize),
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
            bounding_x_range: (
                x - distance_to_beacon as isize,
                x + distance_to_beacon as isize,
            ),
            bounding_y_range: (
                y - distance_to_beacon as isize,
                y + distance_to_beacon as isize,
            ),
        }
    }
}

fn main() {
    let row: isize = args()
        .nth(1)
        .expect("must provide a row number")
        .parse()
        .unwrap();

    let min: isize = args()
        .nth(2)
        .expect("must provide a min grid check")
        .parse()
        .unwrap();

    let max: isize = args()
        .nth(3)
        .expect("must provide a max grid check")
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

    let no_beacon_rows = find_no_beacon_spaces(&sensors, min, max);
    let beacon_ranges = no_beacon_rows.get(&row).unwrap();
    let min_beacon_range = beacon_ranges.iter().min_by_key(|r| r.0).unwrap().0;
    let max_beacon_range = beacon_ranges.iter().max_by_key(|r| r.1).unwrap().1;

    let no_beacons = max_beacon_range - min_beacon_range;

    println!(
        "On row {}, there are {} positions where a beacon can not be present.",
        row, no_beacons
    );

    let mut frequency = 0;

    for i in min..max {
        let first_gap = find_first_gap_in_ranges(no_beacon_rows.get(&i).unwrap());
        if first_gap.is_some() {
            frequency = first_gap.unwrap() * 4000000 + i;
            break;
        }
    }

    println!("The distress frequency is {}.", frequency);
}

fn find_no_beacon_spaces(
    sensors: &Vec<Sensor>,
    min: isize,
    max: isize,
) -> HashMap<isize, Vec<(isize, isize)>> {
    let mut results: HashMap<isize, Vec<(isize, isize)>> = HashMap::new();

    for sensor in sensors {
        for y in min.max(sensor.bounding_y_range.0)..max.min(sensor.bounding_y_range.1) {
            let offset = y.abs_diff(sensor.y);

            let x_results = results.entry(y).or_insert_with(|| Vec::new());
            x_results.push((
                // (sensor.bounding_x_range.0 + offset as isize).max(min),
                // (sensor.bounding_x_range.1 - offset as isize).min(max),
                sensor.bounding_x_range.0 + offset as isize,
                sensor.bounding_x_range.1 - offset as isize,
            ));
        }
    }

    results
}

fn find_first_gap_in_ranges(ranges: &Vec<(isize, isize)>) -> Option<isize> {
    let start = ranges.iter().min_by_key(|r| r.0).unwrap().0;
    let end = ranges.iter().max_by_key(|r| r.1).unwrap().1;

    for i in start..end {
        let mut found = false;
        for r in ranges {
            if i >= r.0 && i <= r.1 {
                found = true;
                break;
            }
        }

        if !found {
            return Some(i);
        }
    }

    None
}
