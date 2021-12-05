use std::collections::HashMap;
use std::error::Error;
use std::io::{self, prelude::*};

#[derive(PartialEq, Debug, Clone, Copy, Eq, Hash)]

pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

#[derive(PartialEq, Debug)]

pub struct Line {
    points: Vec<Point>,
}

impl Line {
    pub fn new(start_point: Point, end_point: Point) -> Line {
        let mut points = Vec::new();
        let mut point = start_point.clone();

        loop {
            points.push(point);

            if point == end_point {
                break;
            }

            if point.x < end_point.x {
                point.x += 1;
            } else if point.x > end_point.x {
                point.x -= 1;
            }

            if point.y < end_point.y {
                point.y += 1;
            } else if point.y > end_point.y {
                point.y -= 1;
            }

            // println!("{:?}", point);
        }

        Line { points }
    }

    pub fn get_points(&self) -> &Vec<Point> {
        &self.points
    }

    pub fn is_straight(&self) -> bool {
        let x_straight = self.points[0].x == self.points[self.points.len() - 1].x;
        let y_straight = self.points[0].y == self.points[self.points.len() - 1].y;

        x_straight || y_straight
    }
}

fn parse_line(line: &str) -> Line {
    let mut points = line.split(" -> ");

    let start_point_raw = points
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let start_point = Point::new(start_point_raw[0], start_point_raw[1]);

    let end_point_raw = points
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let end_point = Point::new(end_point_raw[0], end_point_raw[1]);

    Line::new(start_point, end_point)
}

pub struct Grid {
    points_count: HashMap<Point, i64>,
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            points_count: HashMap::new(),
        }
    }

    pub fn add_line(&mut self, line: &Line) {
        for point in line.get_points() {
            let count = self.points_count.entry(point.clone()).or_insert(0);
            *count += 1;
        }
    }

    pub fn get_point_counts(&self) -> &HashMap<Point, i64> {
        &self.points_count
    }

    pub fn get_point_overlap_count(&self) -> i64 {
        // println!("{:?}", self.points_count);
        // println!("{:?}", self.points_count.values());
        self.points_count.values().filter(|&&x| x > 1).count() as i64
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    // 0,9 -> 5,9

    let vent_lines = lines.map(|s| parse_line(&s.unwrap())).collect::<Vec<_>>();

    let mut grid = Grid::new();

    for line in vent_lines.iter().filter(|x| x.is_straight()) {
        grid.add_line(&line);
    }

    println!(
        "Points where lines overlap: {}",
        grid.get_point_overlap_count()
    );

    let mut diag_grid = Grid::new();

    for line in vent_lines {
        diag_grid.add_line(&line);
    }

    println!(
        "Points where lines overlap (diagonals too): {}",
        diag_grid.get_point_overlap_count()
    );

    Ok(())
}

#[test]
fn test_line_points_created() {
    let line = Line::new(Point::new(1, 1), Point::new(1, 3));

    assert_eq!(
        line.get_points(),
        &vec![Point::new(1, 1), Point::new(1, 2), Point::new(1, 3)]
    );
}

#[test]
fn test_line_points_created_backwards() {
    let line = Line::new(Point::new(9, 7), Point::new(7, 7));

    assert_eq!(
        line.get_points(),
        &vec![Point::new(9, 7), Point::new(8, 7), Point::new(7, 7)]
    );
}

#[test]
fn test_line_points_created_diagonally() {
    let line = Line::new(Point::new(1, 1), Point::new(3, 3));

    assert_eq!(
        line.get_points(),
        &vec![Point::new(1, 1), Point::new(2, 2), Point::new(3, 3)]
    );
}

#[test]
fn test_line_points_created_diagonally_backwards() {
    let line = Line::new(Point::new(9, 7), Point::new(7, 9));

    assert_eq!(
        line.get_points(),
        &vec![Point::new(9, 7), Point::new(8, 8), Point::new(7, 9)]
    );
}
