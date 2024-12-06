use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    io::{self, prelude::*},
};

#[derive(Clone)]
struct Map {
    data: Vec<char>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(data: &[char], width: usize, height: usize) -> Map {
        Map {
            data: data.to_vec(),
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

    fn set_char_at(&mut self, x: isize, y: isize, c: char) {
        if !self.in_bounds(x, y) {
            return;
        }

        self.data[(x as usize) + (y as usize) * self.width] = c;
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

    fn add_obstacle(&mut self, x: isize, y: isize) {
        self.set_char_at(x, y, '#')
    }

    fn find_first_position(&self, needle: char) -> Option<(isize, isize)> {
        let ix = self.data.iter().position(|c| *c == needle);

        ix.map(|ix| {
            (
                ix as isize % self.width as isize,
                ix as isize / self.height as isize,
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

type Point = (isize, isize);
type Direction = (i8, i8);

struct Guard {
    position: Point,
    direction: (i8, i8),
    map: Map,
    visited_positions: HashSet<(Point, Direction)>,
    start_position: Point,
    is_looping: bool,
}

impl Guard {
    fn new(start: (isize, isize), direction: (i8, i8), map: Map) -> Guard {
        let mut visited_positions = HashSet::new();
        visited_positions.insert((start, direction));

        Guard {
            position: start,
            direction,
            map,
            visited_positions,
            start_position: start,
            is_looping: false,
        }
    }

    fn turn_right(&mut self) {
        let mut dir_ix = WALK_DIRECTIONS
            .iter()
            .position(|d| *d == self.direction)
            .expect("Not sure which way this guy is going");

        dir_ix = (dir_ix + 1) % WALK_DIRECTIONS.len();

        self.direction = WALK_DIRECTIONS[dir_ix];
    }

    fn make_one_step(&mut self) {
        let next_position = (
            self.position.0 + self.direction.0 as isize,
            self.position.1 + self.direction.1 as isize,
        );

        if !self.map.in_bounds(next_position.0, next_position.1) {
            // Done - walked off the map
            self.position = next_position;
        } else if !self.map.is_walkable(next_position.0, next_position.1) {
            self.turn_right();
        } else {
            self.position = next_position;

            if self
                .visited_positions
                .contains(&(self.position, self.direction))
            {
                self.is_looping = true;
            }

            self.visited_positions
                .insert((self.position, self.direction));
        }
    }

    fn is_walk_complete(&self) -> bool {
        let (x, y) = self.position;
        !self.map.in_bounds(x, y)
    }

    fn is_looping(&self) -> bool {
        self.is_looping
    }
}

fn count_complete_guard_walk_steps(guard: &mut Guard) -> usize {
    while !guard.is_walk_complete() {
        guard.make_one_step();
    }

    guard.visited_positions.iter().collect::<HashSet<_>>().len()
}

fn detect_guard_looping(guard: &mut Guard) -> bool {
    loop {
        if guard.is_walk_complete() {
            return false;
        }

        if guard.is_looping() {
            return true;
        }

        guard.make_one_step();
    }

    // < 1567
    // == 1562
}

fn generate_all_maps(base_map: &Map, obstacle_positions: &[Point]) -> Vec<Map> {
    obstacle_positions
        .iter()
        .map(|(x, y)| {
            let mut map = base_map.clone();
            map.add_obstacle(*x, *y);
            map
        })
        .collect()
}

fn find_loops_in_maps(all_maps: &mut [Map], start_position: Point) -> usize {
    let mut guards = all_maps
        .iter_mut()
        .map(|m| Guard::new(start_position, WALK_DIRECTIONS[0], m.to_owned()))
        .collect::<Vec<_>>();

    let loopers = guards
        .par_iter_mut()
        .map(detect_guard_looping)
        .filter(|b| *b);

    loopers.count()
}

const WALK_DIRECTIONS: [(i8, i8); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn parse(lines: &[String]) -> (Map, Guard) {
    let grid_data = lines.iter().flat_map(|l| l.chars()).collect::<Vec<_>>();

    let height = lines.len();
    let width = lines[0].len();

    // Find the ^ and create the guard
    let map = Map::new(&grid_data, width, height);

    let guard_position = map.find_first_position('^').expect("Can't find guard");

    let guard = Guard::new(guard_position, WALK_DIRECTIONS[0], map.clone());

    (map, guard)
}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    let (map, mut guard) = parse(&lines);

    let answer = count_complete_guard_walk_steps(&mut guard);

    println!("Guard visits {} distinct positions.", answer);

    // Don't count the start step
    let mut all_visited_positions = guard
        .visited_positions
        .iter()
        .map(|(p, _)| p)
        .copied()
        .collect::<HashSet<_>>();

    all_visited_positions.remove(&guard.start_position);

    let all_visited_positions = all_visited_positions.iter().copied().collect::<Vec<_>>();

    let mut all_maps = generate_all_maps(&map, &all_visited_positions);

    let answer2 = find_loops_in_maps(&mut all_maps, guard.start_position);

    println!(
        "There are {} obstruction positions that will create loops",
        answer2
    );
    Ok(())
}
