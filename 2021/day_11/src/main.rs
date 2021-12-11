use ansi_term::Style;
use std::error::Error;
use std::fmt;
use std::io::{self, prelude::*};

use itertools::Itertools;

#[derive(Debug, Copy, Clone)]
pub struct Octopus {
    energy: u8,
    flashed: bool,
    x: u8,
    y: u8,
}

pub struct OctopusGrid {
    octopus_energies: Vec<Vec<Octopus>>,
}

impl OctopusGrid {
    pub fn new(octopus_energies: &Vec<Vec<u8>>) -> OctopusGrid {
        OctopusGrid {
            octopus_energies: octopus_energies
                .iter()
                .enumerate()
                .map(|(x, row)| {
                    row.iter()
                        .enumerate()
                        .map(|(y, &energy)| Octopus {
                            energy,
                            flashed: false,
                            x: x as u8,
                            y: y as u8,
                        })
                        .collect_vec()
                })
                .collect_vec(),
        }
    }

    pub fn step(&mut self) {
        for row in &mut self.octopus_energies {
            for octopus in row {
                octopus.flashed = false;
                octopus.energy += 1;

                if octopus.energy > 9 {
                    octopus.flashed = true;
                }
            }
        }

        self.octopus_energies.clone().iter().for_each(|row| {
            row.iter().for_each(|octopus| {
                if octopus.flashed {
                    let x = octopus.x;
                    let y = octopus.y;

                    self.process_flash(x, y);
                }
            })
        });

        for row in &mut self.octopus_energies {
            for octopus in row {
                if octopus.energy > 9 {
                    octopus.energy = 0;
                }
            }
        }
    }

    pub fn count_flashed(&self) -> usize {
        self.octopus_energies.iter().fold(0, |acc, row| {
            acc + row.iter().filter(|octopus| octopus.flashed).count()
        })
    }

    fn get_neighbours(&self, x: u8, y: u8) -> Vec<(u8, u8)> {
        let mut neighbours = Vec::new();

        for x_offset in -1..=1 {
            for y_offset in -1..=1 {
                if x_offset == 0 && y_offset == 0 {
                    continue;
                }

                let x_neighbour = x as i8 + x_offset;
                let y_neighbour = y as i8 + y_offset;

                // println!("x_neighbour: {}", x_neighbour);
                // println!("y_neighbour: {}", y_neighbour);

                // println!("{:?}", self.octopus_energies);

                if x_neighbour >= 0
                    && y_neighbour >= 0
                    && x_neighbour < self.octopus_energies.len() as i8
                    && y_neighbour < self.octopus_energies[x_neighbour as usize].len() as i8
                {
                    neighbours.push((x_neighbour as u8, y_neighbour as u8));
                }
            }
        }

        neighbours
    }

    fn process_flash(&mut self, x: u8, y: u8) {
        let neighbours = self.get_neighbours(x, y);

        for &(x, y) in &neighbours {
            let octopus = &mut self.octopus_energies[x as usize][y as usize];
            octopus.energy += 1;

            if !octopus.flashed && octopus.energy > 9 {
                octopus.flashed = true;
                let x = octopus.x;
                let y = octopus.y;

                self.process_flash(x, y);
            }
        }
    }

    fn all_in_sync(&self) -> bool {
        self.octopus_energies
            .iter()
            .all(|row| row.iter().all(|octopus| octopus.flashed == true))
    }
}

impl fmt::Display for OctopusGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.octopus_energies.iter() {
            for octopus in row.iter() {
                let octopus_style = if octopus.flashed {
                    Style::new().bold()
                } else {
                    Style::new()
                };

                write!(f, "{}", octopus_style.paint(octopus.energy.to_string()))?;
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

    let octopuses = lines
        .map(|l| parse_line(&l.unwrap()))
        .collect::<Vec<Vec<_>>>();

    let mut octo_grid = OctopusGrid::new(&octopuses);

    let mut total_score = 0;

    for _ in 0..100 {
        octo_grid.step();
        total_score += octo_grid.count_flashed();
    }

    println!("Number of flashes after 100 steps: {}", total_score);

    let mut octo_grid2 = OctopusGrid::new(&octopuses);

    let mut turn = 1;

    loop {
        octo_grid2.step();

        if octo_grid2.all_in_sync() {
            break;
        }

        turn += 1;
    }

    println!("All octopuses flash on turn: {}", turn);

    Ok(())
}
