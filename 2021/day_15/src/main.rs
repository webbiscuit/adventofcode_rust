use std::error::Error;
use std::fmt;
use std::io::{self, prelude::*};

pub struct Point {
    value: u8,
    x: u8,
    y: u8,
}

pub struct Grid {
    points: Vec<Vec<Point>>,
}

impl Grid {
    pub fn new(points: Vec<Vec<u8>>) -> Self {
        Self {
            points: points
                .iter()
                .enumerate()
                .map(|(x, row)| {
                    row.iter()
                        .enumerate()
                        .map(|(y, &value)| Point {
                            value,
                            x: x as u8,
                            y: y as u8,
                        })
                        .collect()
                })
                .collect(),
        }
    }

    pub fn calculate_path_risks(&self) -> Vec<Vec<u32>> {
        let cumulative_risk: Vec<Vec<u32>>;

        cumulative_risk = vec![vec![0; self.points[0].len()]; self.points.len()];

        cumulative_risk
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.points.iter() {
            for height in row.iter() {
                write!(f, "{}", height.value)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn parse_line(line: &str) -> Vec<u8> {
    line.chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let points = lines
        .map(|l| parse_line(&l.unwrap()))
        .collect::<Vec<Vec<_>>>();

    let mut grid = Grid::new(points);

    println!("{}", grid);

    Ok(())
}
