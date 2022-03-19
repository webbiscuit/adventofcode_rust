use std::error::Error;
use std::fmt;
use std::io::{self, prelude::*};

use itertools::Itertools;

#[derive(Debug, Copy, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Point {
    value: u8,
    region: Option<u8>,
    x: u8,
    y: u8,
}

#[derive(Debug, Clone)]
pub struct HeightMap {
    heights: Vec<Vec<Point>>,
}

impl HeightMap {
    pub fn new(heights: Vec<Vec<u8>>) -> HeightMap {
        HeightMap {
            heights: heights
                .iter()
                .enumerate()
                .map(|(x, row)| {
                    row.iter()
                        .enumerate()
                        .map(|(y, &value)| Point {
                            value,
                            region: None,
                            x: x as u8,
                            y: y as u8,
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

    fn get_neighbours(&mut self, x: u8, y: u8) -> Vec<(u8, u8)> {
        let mut neighbours = Vec::new();
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
                    neighbours.push((x_neighbour as u8, y_neighbour as u8));
                }
            }
        }

        neighbours
    }

    pub fn blend(&mut self) -> u32 {
        let mut lowest_points = Vec::new();
        let mut regions = 0u8;

        // Find lowest points
        // Mark as regions
        for x in 0..self.heights.len() {
            for y in 0..self.heights[x].len() {
                let neighbour_heights = self.get_neighbour_heights(x, y);
                let mut this_point = self.heights[x][y];

                if neighbour_heights.iter().all(|&h| this_point.value < h) {
                    lowest_points.push((x, y));

                    this_point.region = Some(regions);
                    regions += 1;
                    self.heights[x][y] = this_point;
                }
            }
        }

        for (x, y) in lowest_points {
            let mut neighbours = self.get_neighbours(x as u8, y as u8);
            let this_point = self.heights[x][y];

            while !neighbours.is_empty() {
                let neighbour = neighbours.pop().unwrap();
                let mut neighbour_point = self.heights[neighbour.0 as usize][neighbour.1 as usize];

                if neighbour_point.value == 9 {
                    continue;
                }

                if neighbour_point.region.is_none() {
                    neighbour_point.region = this_point.region;
                    neighbours.extend(self.get_neighbours(neighbour_point.x, neighbour_point.y));
                    self.heights[neighbour.0 as usize][neighbour.1 as usize] = neighbour_point;
                }
            }
        }

        let mut regions = self
            .heights
            .iter()
            .flat_map(|r| r.iter().flat_map(|p| p.region))
            .collect_vec();

        regions.sort_unstable();

        regions = regions
            .iter()
            .group_by(|&k| k)
            .into_iter()
            .map(|(_, v)| v.count() as u8)
            .collect_vec();

        regions.sort_unstable();
        regions.reverse();

        regions[0] as u32 * regions[1] as u32 * regions[2] as u32
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

    let mut height_map = HeightMap::new(heights);

    let risk_level = height_map.calculate_risk_level();

    // println!("{}", height_map);

    println!("Sum of the risk levels of all low points is {}", risk_level);

    let basins = height_map.blend();
    // println!("{}", height_map);

    println!("Product of 3 largest basins is {}", basins);

    Ok(())
}
