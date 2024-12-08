use std::{
    collections::{HashMap, HashSet},
    io::{self, prelude::*},
};

type Point = (isize, isize);

struct Grid {
    data: Vec<char>,
    width: usize,
    height: usize,
    antennas: HashMap<char, Vec<Point>>,
}

impl Grid {
    fn new(data: &[char], width: usize, height: usize) -> Grid {
        let mut unique_antennas = data.iter().collect::<HashSet<_>>();
        unique_antennas.remove(&'.');
        let antennas = unique_antennas
            .iter()
            .map(|&a| {
                let positions = data
                    .iter()
                    .enumerate()
                    .filter_map(|(i, &c)| {
                        if c == *a {
                            Some(((i % width) as isize, (i / width) as isize))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                (*a, positions)
            })
            .collect::<HashMap<_, _>>();

        Grid {
            data: data.to_vec(),
            width,
            height,
            antennas,
        }
    }

    fn in_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0 && y >= 0 && x < self.width as isize && y < self.height as isize
    }

    fn get_char_at(&self, x: isize, y: isize) -> Option<char> {
        if !self.in_bounds(x, y) {
            return None;
        }

        self.data
            .get((x as usize) + (y as usize) * self.width)
            .copied()
    }

    fn find_antinodes(&self) -> HashSet<Point> {
        let mut antinodes: HashSet<Point> = HashSet::new();

        self.antennas.keys().for_each(|k| {
            for (i, &a) in self.antennas[k].iter().enumerate() {
                for (_, &b) in self.antennas[k].iter().enumerate().skip(i + 1) {
                    let diff = (a.0 - b.0, a.1 - b.1);
                    let p1 = (a.0 + diff.0, a.1 + diff.1);
                    let p2 = (b.0 - diff.0, b.1 - diff.1);

                    if self.in_bounds(p1.0, p1.1) {
                        antinodes.insert(p1);
                    }

                    if self.in_bounds(p2.0, p2.1) {
                        antinodes.insert(p2);
                    }
                }
            }
        });

        antinodes
    }

    fn find_echoed_antinodes(&self) -> HashSet<Point> {
        let mut antinodes: HashSet<Point> = HashSet::new();

        self.antennas.keys().for_each(|k| {
            for (i, &a) in self.antennas[k].iter().enumerate() {
                for (_, &b) in self.antennas[k].iter().enumerate().skip(i + 1) {
                    let diff = (a.0 - b.0, a.1 - b.1);
                    let mut p1 = a;
                    let mut p2 = b;

                    antinodes.insert(p1);
                    antinodes.insert(p2);

                    loop {
                        p1 = (p1.0 + diff.0, p1.1 + diff.1);

                        if self.in_bounds(p1.0, p1.1) {
                            antinodes.insert(p1);
                        } else {
                            break;
                        }
                    }

                    loop {
                        p2 = (p2.0 - diff.0, p2.1 - diff.1);

                        if self.in_bounds(p2.0, p2.1) {
                            antinodes.insert(p2);
                        } else {
                            break;
                        }
                    }
                }
            }
        });

        antinodes
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let antinodes = self.find_echoed_antinodes();

        for y in 0..self.height {
            for x in 0..self.width {
                write!(
                    f,
                    "{}",
                    if antinodes.contains(&(x as isize, y as isize)) {
                        '#'
                    } else {
                        self.get_char_at(x as isize, y as isize).unwrap_or(' ')
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse(lines: &[String]) -> Grid {
    let grid_data = lines.iter().flat_map(|l| l.chars()).collect::<Vec<_>>();

    let height = lines.len();
    let width = lines[0].len();

    Grid::new(&grid_data, width, height)
}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    let grid = parse(&lines);

    // println!("{}", grid);
    // println!("{:?}", grid.antennas);
    // println!("{:?}", grid.find_antinodes());

    let antinodes = grid.find_antinodes();
    let result = antinodes.len();

    println!("There are {} antinodes in bounds.", result);

    let antinodes = grid.find_echoed_antinodes();
    let result2 = antinodes.len();

    println!("There are {} echoed antinodes in bounds.", result2);

    Ok(())
}
