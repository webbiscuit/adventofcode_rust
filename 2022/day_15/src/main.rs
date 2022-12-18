use std::{
    collections::HashSet,
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

    let mut no_x_beacon_ranges = Vec::<(isize, isize)>::new();
    let beacon_locations: HashSet<(isize, isize)> = sensors
        .iter()
        .map(|s| (s.closest_beacon.0, s.closest_beacon.1))
        .collect::<HashSet<_>>();

    let no_beacon_spaces = find_no_beacon_spaces(&sensors, row, &beacon_locations);

    let no_beacons = no_beacon_spaces.len();

    println!(
        "On row {}, there are {} positions where a beacon can not be present.",
        row, no_beacons
    );

    let ys_to_check: Vec<(isize, isize)> = sensors.iter().map(|s| s.bounding_y_range).collect();
    println!("{:?}", ys_to_check);

    for r in 0..=20 {
        let no_beacon_spaces = find_no_beacon_spaces(&sensors, r, &beacon_locations);

        let no_beacons = no_beacon_spaces.len();

        println!(
            "On row {}, there are {} positions where a beacon can not be present.",
            r, no_beacons
        );
    }
}

fn find_no_beacon_spaces(
    sensors: &Vec<Sensor>,
    row: isize,
    beacon_locations: &HashSet<(isize, isize)>,
) -> HashSet<isize> {
    let mut no_x_beacon_ranges = Vec::<(isize, isize)>::new();

    for sensor in sensors {
        if row < sensor.bounding_y_range.0 || row > sensor.bounding_y_range.1 {
            continue;
        }

        let offset = row.abs_diff(sensor.y);

        // if sensor.y == 7 && sensor.x == 8 {
        //     println!("offset: {}", offset);
        // }

        no_x_beacon_ranges.push((
            sensor.bounding_x_range.0 + offset as isize,
            sensor.bounding_x_range.1 - offset as isize,
        ));
    }
    let mut no_beacon_spaces = no_x_beacon_ranges
        .iter()
        .flat_map(|range| range.0..=range.1)
        .collect::<HashSet<_>>();
    let removal_list = no_beacon_spaces
        .intersection(
            &beacon_locations
                .iter()
                .copied()
                .filter(|(_x, y)| *y == row)
                .map(|(x, _y)| x)
                .collect::<HashSet<_>>(),
        )
        .copied()
        .collect::<HashSet<_>>();
    // for item in removal_list {
    //     no_beacon_spaces.remove(&item);
    // }
    no_beacon_spaces = no_beacon_spaces
        .difference(&removal_list)
        .copied()
        .collect();
    no_beacon_spaces
}
