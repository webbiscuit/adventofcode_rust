use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::{self, prelude::*},
};

type Point = (isize, isize);
type Area = usize;
type Perimeter = usize;

struct Map {
    data: Vec<char>,
    width: usize,
    height: usize,
}

impl Map {
    const ALL_DIRECTIONS: [(i8, i8); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

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

    fn find_all_fields(&self) -> HashSet<&char> {
        self.data.iter().collect::<HashSet<&char>>()
    }

    fn find_areas_and_perimeters(&self) -> Vec<(char, Area, Perimeter)> {
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

                // println!(
                //     "{} area {:?} perimeter {:?}",
                //     *field, area_points, perimeter_points
                // );
                // println!(
                //     "{} area {:?} perimeter {:?}",
                //     *field,
                //     area_points.len(),
                //     perimeter_points.len()
                // );

                Some((*field, area_points.len(), perimeter_points.len()))
            })
            .collect()
    }

    fn calculate_total_price_of_fence(&self) -> usize {
        // let all_fields = self.find_all_fields();
        let all_areas_and_permeters = self.find_areas_and_perimeters();
        let total_price = all_areas_and_permeters
            .iter()
            .fold(0, |acc, (c, a, p)| acc + a * p);

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

    println!("The total price of fence is {}", result);

    Ok(())
}
