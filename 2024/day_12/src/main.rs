use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::{self, prelude::*},
};

type Point = (isize, isize);
type Area = usize;
type Perimeter = usize;
type Side = usize;

struct Map {
    data: Vec<char>,
    width: usize,
    height: usize,
}

impl Map {
    const ALL_DIRECTIONS: [(i8, i8); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    const LR_DIRECTIONS: [(i8, i8); 2] = [(1, 0), (-1, 0)];
    const UD_DIRECTIONS: [(i8, i8); 2] = [(0, 1), (0, -1)];

    fn new(data: Vec<char>, width: usize, height: usize) -> Map {
        Map {
            data,
            width,
            height,
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

    fn get_neighbours(&self, x: isize, y: isize) -> Vec<Point> {
        Map::ALL_DIRECTIONS
            .iter()
            .map(|&(dx, dy)| {
                let p = (x + dx as isize, y + dy as isize);

                p
            })
            .collect()
    }

    fn get_lr_neighbours(&self, x: isize, y: isize) -> Vec<Point> {
        Map::LR_DIRECTIONS
            .iter()
            .map(|&(dx, dy)| {
                let p = (x + dx as isize, y + dy as isize);

                p
            })
            .collect()
    }

    fn get_ud_neighbours(&self, x: isize, y: isize) -> Vec<Point> {
        Map::UD_DIRECTIONS
            .iter()
            .map(|&(dx, dy)| {
                let p = (x + dx as isize, y + dy as isize);

                p
            })
            .collect()
    }

    fn find_all_fields(&self) -> HashSet<&char> {
        self.data.iter().collect::<HashSet<&char>>()
    }

    fn find_areas_and_perimeters(&self) -> Vec<(char, Area, Perimeter, Side)> {
        let mut visited: HashSet<Point> = HashSet::new();

        self.data
            .iter()
            .enumerate()
            .filter_map(|(ix, field)| {
                let p = ((ix % self.width) as isize, (ix / self.width) as isize);

                // println!("{:?}", p);

                if visited.contains(&p) {
                    return None;
                }

                // Perimeters can appear twice
                let mut perimeter_points: Vec<Point> = Vec::new();
                let mut area_points: HashSet<Point> = HashSet::new();
                area_points.insert(p);

                let mut explore_list: VecDeque<Point> = VecDeque::new();
                explore_list.push_back(p);

                while let Some(point) = explore_list.pop_front() {
                    if visited.contains(&point) {
                        continue;
                    }

                    let neighbours = self.get_neighbours(point.0, point.1);

                    for n in neighbours {
                        if !self.in_bounds(n.0, n.1) {
                            perimeter_points.push(n);
                        } else {
                            let neighbouring_field = self.get_char_at(n.0, n.1).unwrap();

                            if neighbouring_field == *field {
                                explore_list.push_back(n);
                                area_points.insert(n);
                                visited.insert(point);
                            } else {
                                perimeter_points.push(n);
                            }
                        }
                    }
                }

                let mut sides: Vec<Vec<Point>> = vec![];
                let mut points_left = perimeter_points.clone();

                while let Some(start) = points_left.pop() {
                    let mut side: Vec<Point> = vec![start];
                    let mut side_explore_list: VecDeque<Point> = VecDeque::new();
                    side_explore_list.push_back(start);

                    while let Some(point) = side_explore_list.pop_front() {
                        let neighbours = self.get_lr_neighbours(point.0, point.1);

                        for n in neighbours {
                            if side.contains(&n) {
                                continue;
                            }

                            if points_left.contains(&n) {
                                // This is a corner
                                if let Some(p2) = self.get_char_at(n.0, n.1) {
                                    println!("{}", p2);
                                    if p2 == *field {
                                        println!("SKIPP");
                                        continue;
                                    }
                                }

                                if let Some(pos) = points_left.iter().position(|&p| p == n) {
                                    points_left.remove(pos);
                                }
                                side_explore_list.push_back(n);
                                side.push(n);
                                continue;
                            }
                        }

                        let neighbours = self.get_ud_neighbours(point.0, point.1);

                        for n in neighbours {
                            if side.contains(&n) {
                                continue;
                            }

                            if points_left.contains(&n) {
                                // This is a corner
                                if let Some(p2) = self.get_char_at(n.0, n.1) {
                                    println!("{}", p2);

                                    if p2 == *field {
                                        println!("SKIPP");

                                        continue;
                                    }
                                }

                                if let Some(pos) = points_left.iter().position(|&p| p == n) {
                                    points_left.remove(pos);
                                }
                                side_explore_list.push_back(n);
                                side.push(n);
                                continue;
                            }
                        }
                    }

                    sides.push(side);
                }

                println!(
                    "{} area {:?} perimeter {:?} sides {:?}",
                    *field, area_points, perimeter_points, sides,
                );
                println!(
                    "{} area {:?} perimeter {:?} side {:?}",
                    *field,
                    area_points.len(),
                    perimeter_points.len(),
                    sides.len()
                );

                Some((
                    *field,
                    area_points.len(),
                    perimeter_points.len(),
                    sides.len(),
                ))
            })
            .collect()
    }

    fn calculate_total_price_of_fence(&self) -> usize {
        // let all_fields = self.find_all_fields();
        let all_areas_and_permeters = self.find_areas_and_perimeters();
        let total_price = all_areas_and_permeters
            .iter()
            .fold(0, |acc, (_, a, p, _)| acc + a * p);

        total_price
    }

    fn calculate_total_price_of_fence_with_sides(&self) -> usize {
        // let all_fields = self.find_all_fields();
        let all_areas_and_permeters = self.find_areas_and_perimeters();
        let total_price = all_areas_and_permeters
            .iter()
            .fold(0, |acc, (_, a, _, s)| acc + a * s);

        total_price
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
        .flat_map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let height = lines.len();
    let width = lines[0].len();

    Map::new(grid_data, width, height)
}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    let map = parse(&lines);

    let result = map.calculate_total_price_of_fence();
    let result2 = map.calculate_total_price_of_fence_with_sides();

    println!("The total price of fence is {}", result);
    println!("The total price of fence with discount is {}", result2);

    Ok(())
}

// > 811603
