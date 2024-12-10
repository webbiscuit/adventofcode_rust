use std::{
    collections::{HashSet, VecDeque},
    io::{self, prelude::*},
};

type Point = (isize, isize);

struct Map {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Map {
    const ALL_DIRECTIONS: [(i8, i8); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    fn new(data: Vec<u8>, width: usize, height: usize) -> Map {
        Map {
            data,
            width,
            height,
        }
    }

    fn in_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0 && y >= 0 && x < self.width as isize && y < self.height as isize
    }

    fn get_char_at(&self, x: isize, y: isize) -> Option<u8> {
        if !self.in_bounds(x, y) {
            return None;
        }

        self.data
            .get((x as usize) + (y as usize) * self.width)
            .copied()
    }

    fn get_neighbours(&self, x: isize, y: isize) -> Vec<Point> {
        Map::ALL_DIRECTIONS
            .iter()
            .filter_map(|&(dx, dy)| {
                let p = (x + dx as isize, y + dy as isize);

                if self.in_bounds(p.0, p.1) {
                    return Some(p);
                }

                None
            })
            .collect()
    }

    fn find_paths_from_point_to_target(&self, start_pos: Point, target_height: u8) -> Vec<Point> {
        let pos = start_pos;

        let mut search_queue = VecDeque::new();
        search_queue.push_back(pos);

        let mut paths = vec![pos];

        while let Some(pos) = search_queue.pop_front() {
            let current_height = self.get_char_at(pos.0, pos.1).expect("Needs to be in grid");

            // println!("Pos {:?}, current: {}", pos, current_height);

            if current_height == target_height {
                paths.push(pos);
                continue;
            }

            let neighbours = self.get_neighbours(pos.0, pos.1);

            let valid_neighbours = neighbours.iter().filter_map(|&(x, y)| {
                let next_height = self.get_char_at(x, y).expect("Needs to be in grid");

                if next_height == current_height + 1 {
                    return Some((x, y));
                }

                None
            });

            paths.extend(valid_neighbours.clone());
            search_queue.extend(valid_neighbours);
        }

        paths
    }

    fn count_any_routes_to_end(&self, start: u8, end: u8) -> usize {
        let start_locs = self.data.iter().enumerate().filter_map(|(ix, d)| {
            if *d == start {
                let p = ((ix % self.width) as isize, (ix / self.width) as isize);
                Some((p.0, p.1))
            } else {
                None
            }
        });

        let all_unique_steps = start_locs
            .map(|p| {
                self.find_paths_from_point_to_target(p, end)
                    .iter()
                    .copied()
                    .collect::<HashSet<_>>()
            })
            .collect::<Vec<_>>();

        let end_locs = self
            .data
            .iter()
            .enumerate()
            .filter_map(|(ix, d)| {
                if *d == end {
                    let p = ((ix % self.width) as isize, (ix / self.width) as isize);
                    Some((p.0, p.1))
                } else {
                    None
                }
            })
            .collect::<HashSet<_>>();

        let found_end_locs_in_all_steps = all_unique_steps
            .iter()
            .flat_map(|steps| {
                let intersects = steps.intersection(&end_locs);
                intersects
            })
            .collect::<Vec<_>>();

        found_end_locs_in_all_steps.len()
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.get_char_at(x as isize, y as isize).unwrap())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse(lines: &[String]) -> Map {
    let grid_data = lines
        .iter()
        .flat_map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).expect("Not a digit") as u8)
        })
        .collect::<Vec<_>>();

    let height = lines.len();
    let width = lines[0].len();

    Map::new(grid_data, width, height)
}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    let map = parse(&lines);
    let answer = map.count_any_routes_to_end(0, 9);

    println!("The sum of all the trailheads {}", answer);

    Ok(())
}
