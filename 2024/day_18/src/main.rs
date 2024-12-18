use std::{
    collections::{HashMap, HashSet, VecDeque},
    env::args,
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
    fn new(width: usize, height: usize) -> Map {
        Map {
            data: vec!['.'; width * height],
            width,
            height,
            start: (0, 0),
            end: ((width - 1) as isize, (height - 1) as isize),
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

    const ALL_DIRECTIONS: [Dir; 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    fn get_neighbours(&self, x: isize, y: isize) -> Vec<Point> {
        Map::ALL_DIRECTIONS
            .iter()
            .filter_map(|&(dx, dy)| {
                let p = (x + dx as isize, y + dy as isize);

                if self.in_bounds(p.0, p.1) && self.is_walkable(p.0, p.1) {
                    return Some(p);
                }

                None
            })
            .collect()
    }

    fn set_char_at(&mut self, x: isize, y: isize, c: char) {
        if !self.in_bounds(x, y) {
            return;
        }

        self.data[(x as usize) + (y as usize) * self.width] = c;
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

fn parse(lines: &[String]) -> Vec<Point> {
    lines
        .iter()
        .map(|l| l.split_once(',').unwrap())
        .map(|(x, y)| (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap()))
        .collect()
}

fn drop_bytes_onto_map(map: &mut Map, byte_positions: &[Point]) {
    byte_positions.iter().for_each(|(x, y)| {
        map.set_char_at(*x, *y, '#');
    });
}

fn find_path_from_start_to_end(map: &Map) -> Option<Vec<Point>> {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut came_from: HashMap<Point, Point> = HashMap::new();
    let mut explore_queue = VecDeque::new();

    visited.insert(map.start);
    explore_queue.push_back(map.start);

    while let Some(current) = explore_queue.pop_front() {
        if current == map.end {
            break;
        }

        for neighbour in map.get_neighbours(current.0, current.1) {
            if !visited.contains(&neighbour) {
                visited.insert(neighbour);
                came_from.insert(neighbour, current);
                explore_queue.push_back(neighbour);
            }
        }
    }

    // Reconstruct the path
    let mut path = Vec::new();
    let mut current = map.end;

    while current != map.start {
        path.push(current);
        current = match came_from.get(&current) {
            Some(&prev) => prev,
            None => return None,
        }
    }

    path.push(map.start);
    path.reverse();

    // println!("Path found: {:?}", path);
    Some(path)
}

fn draw_path_on_map(map: &Map, path: &[Point]) {
    let mut grid = vec![vec!['.'; map.width]; map.height];

    for (ix, c) in map.data.iter().enumerate() {
        let x = ix % map.width;
        let y = ix / map.width;
        grid[y][x] = *c;
    }

    for (x, y) in path {
        grid[*y as usize][*x as usize] = 'O';
    }

    for row in grid {
        let line: String = row.iter().collect();
        println!("{}", line);
    }
}

fn main() -> std::io::Result<()> {
    let grid_size: usize = args().nth(1).unwrap_or(71.to_string()).parse().unwrap();
    let bytes_to_drop: usize = args().nth(2).unwrap_or(1024.to_string()).parse().unwrap();

    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    let byte_positions = parse(&lines);

    let mut map = Map::new(grid_size, grid_size);

    drop_bytes_onto_map(&mut map, &byte_positions[0..bytes_to_drop]);

    // println!("{}", map);

    let shortest_path = find_path_from_start_to_end(&map);

    // draw_path_on_map(&map, &shortest_path);

    let result = shortest_path.unwrap().len() - 1;
    println!("Minimum number of steps required is {}", result);

    for n in bytes_to_drop.. {
        drop_bytes_onto_map(&mut map, &vec![byte_positions[n]]);
        let shortest_path = find_path_from_start_to_end(&map);

        if shortest_path.is_none() {
            println!("The byte that prevents exit is {:?}", byte_positions[n]);
            break;
        }
    }

    Ok(())
}

// < 360
// = 280
