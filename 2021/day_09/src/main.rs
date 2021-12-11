use std::error::Error;
use std::fmt;
use std::io::{self, prelude::*};

pub struct Point {
    value: u8,
    region: Option<u8>,
}

pub struct HeightMap {
    heights: Vec<Vec<Point>>,
}

impl HeightMap {
    pub fn new(heights: Vec<Vec<u8>>) -> HeightMap {
        HeightMap {
            heights: heights
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|&value| Point {
                            value,
                            region: None,
                        })
                        .collect()
                })
                .collect(),
        }
    }

    pub fn calculate_risk_level(&self) -> u32 {
        let mut risk_level = 0u32;
        for (x, row) in self.heights.iter().enumerate() {
            for (y, height) in row.iter().enumerate() {
                let neighbour_heights = self.get_neighbour_heights(x, y);

                if neighbour_heights.iter().all(|&h| height.value < h) {
                    // println!("{:?}", neighbour_heights);
                    // println!("{}", height);
                    risk_level += height.value as u32 + 1;
                }
            }
        }
        risk_level
    }

    fn get_neighbour_heights(&self, x: usize, y: usize) -> Vec<u8> {
        let mut neighbour_heights = Vec::new();
        for i in -1..=1 {
            for j in -1..=1 {
                // Adjacent only
                if i32::abs(i) == i32::abs(j) {
                    continue;
                }

                let x_neighbour = x as i8 + i as i8;
                let y_neighbour = y as i8 + j as i8;

                if x_neighbour >= 0
                    && y_neighbour >= 0
                    && x_neighbour < self.heights.len() as i8
                    && y_neighbour < self.heights[x_neighbour as usize].len() as i8
                {
                    neighbour_heights
                        .push(self.heights[x_neighbour as usize][y_neighbour as usize].value);
                }
            }
        }

        neighbour_heights
    }
}

impl fmt::Display for HeightMap {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.heights.iter() {
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

    let heights = lines
        .map(|l| parse_line(&l.unwrap()))
        .collect::<Vec<Vec<_>>>();

    let height_map = HeightMap::new(heights);

    let risk_level = height_map.calculate_risk_level();

    // println!("{}", height_map);

    println!("Sum of the risk levels of all low points is {}", risk_level);

    Ok(())
}
