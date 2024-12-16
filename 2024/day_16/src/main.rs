use std::{
    collections::HashMap,
    io::{self, prelude::*},
    usize,
};

type Point = (isize, isize);
type Dir = (i8, i8);

struct Map {
    data: Vec<char>,
    width: usize,
    height: usize,
    start: Point,
    end: Point,
}

impl Map {
    fn new(data: &[char], width: usize, height: usize) -> Map {
        let mut map = Map {
            data: data.to_vec(),
            width,
            height,
            start: (0, 0),
            end: (0, 0),
        };

        map.start = map.find_first_position('S').expect("No start position");
        map.end = map.find_first_position('E').expect("No end position");

        map
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

    fn is_obstacle(c: char) -> bool {
        c == '#'
    }

    fn is_walkable(&self, x: isize, y: isize) -> bool {
        let c = self.get_char_at(x, y);

        match c {
            Some(c) => !Map::is_obstacle(c),
            _ => false,
        }
    }

    const ALL_DIRECTIONS: [(i8, i8); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    fn get_neighbours(&self, x: isize, y: isize) -> Vec<Point> {
        Map::ALL_DIRECTIONS
            .iter()
            .filter_map(|&(dx, dy)| {
                let p = (x + dx as isize, y + dy as isize);

                if self.in_bounds(p.0, p.1) {
                    if self.is_walkable(p.0, p.1) {
                        return Some(p);
                    } else {
                        return None;
                    }
                }

                None
            })
            .collect()
    }

    // fn calculate_distances_from_point(&self, source: Point) -> HashMap<Point, usize> {
    //     // let mut distances = vec![(source, 0)];
    //     let mut distances: HashMap<Point, usize> = HashMap::new();
    //     distances.insert(source, 0);

    //     let mut explore_queue = vec![source];

    //     while let Some((x, y)) = explore_queue.pop() {
    //         let neighbours = self.get_neighbours(x, y);
    //         let valid_neighbours = neighbours
    //             .into_iter()
    //             .filter(|n| !distances.contains_key(n))
    //             .collect::<Vec<_>>();

    //         for n in valid_neighbours.iter() {
    //             if !distances.contains_key(&n) {
    //                 let this_distance = distances.get(&(x, y)).unwrap();
    //                 distances.insert(*n, *this_distance + 1);
    //             }
    //         }

    //         explore_queue.extend(valid_neighbours);
    //     }

    //     distances
    // }

    fn find_lowest_score(&self) -> usize {
        let start_dir = Self::ALL_DIRECTIONS[0];
        let mut steps_to_try = vec![((self.start, start_dir), 0)];

        // Scores from starting point to Point
        let mut g_scores: HashMap<(Point, Dir), usize> = HashMap::new();
        g_scores.insert((self.start, start_dir), 0);

        while !steps_to_try.is_empty() {
            let (ix, &(current_state, current_score)) = steps_to_try
                .iter()
                .enumerate()
                .min_by_key(|&(_, &(_, score))| score)
                .unwrap();
            steps_to_try.remove(ix);

            let (current_point, current_dir) = current_state;

            if current_point == self.end {
                return current_score;
            }

            for neighbour in self.get_neighbours(current_point.0, current_point.1) {
                let neighbour_dir = (
                    (neighbour.0 - current_point.0) as i8,
                    (neighbour.1 - current_point.1) as i8,
                );

                let move_cost = if neighbour_dir == current_dir {
                    1
                } else {
                    1001
                };
                let tentative_g_score = current_score + move_cost;

                if tentative_g_score
                    < *g_scores
                        .get(&(neighbour, current_dir))
                        .unwrap_or(&usize::MAX)
                {
                    g_scores.insert((neighbour, current_dir), tentative_g_score);
                    steps_to_try.push(((neighbour, neighbour_dir), tentative_g_score));
                }
            }
        }

        usize::MAX
    }

    fn find_first_position(&self, needle: char) -> Option<(isize, isize)> {
        let ix = self.data.iter().position(|c| *c == needle);

        ix.map(|ix| {
            (
                ix as isize % self.width as isize,
                ix as isize / self.width as isize,
            )
        })
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(
                    f,
                    "{}",
                    self.get_char_at(x as isize, y as isize).unwrap_or(' ')
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse(lines: &[String]) -> Map {
    let grid_data = lines.iter().flat_map(|l| l.chars()).collect::<Vec<_>>();

    let height = lines.len();
    let width = lines[0].len();

    Map::new(&grid_data, width, height)
}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    let map = parse(&lines);

    // println!("{}", map);
    // println!("{:?} {:?}", map.start, map.end);

    // let distances = map.calculate_distances_from_point(map.end);
    // println!("Distances {:?}", distances);

    let lowest_score = map.find_lowest_score();

    println!("The lowest score a reindeer can get is {}", lowest_score);

    Ok(())
}
