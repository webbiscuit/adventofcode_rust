use std::io::{self, prelude::*};

type Point = (isize, isize);

#[derive(Debug, Clone)]
struct Map {
    data: Vec<char>,
    width: usize,
    height: usize,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_point(&self) -> Point {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    fn from(c: char) -> Direction {
        match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            '^' => Direction::Up,
            'v' => Direction::Down,
            _ => panic!("Cannot parse"),
        }
    }
}

impl Map {
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

    fn find_first_position(&self, needle: char) -> Option<(isize, isize)> {
        let ix = self.data.iter().position(|c| *c == needle);

        // println!("IX {:?}", ix);

        ix.map(|ix| {
            (
                ix as isize % self.width as isize,
                ix as isize / self.width as isize,
            )
        })
    }

    fn is_wall(c: char) -> bool {
        c == '#'
    }

    fn is_wall_on_tile(&self, x: isize, y: isize) -> bool {
        let c = self.get_char_at(x, y);

        Map::is_wall(c.unwrap())
    }

    fn is_space(c: char) -> bool {
        c == '.'
    }

    fn is_space_on_tile(&self, x: isize, y: isize) -> bool {
        let c = self.get_char_at(x, y);

        Map::is_space(c.unwrap())
    }

    fn is_barrel(c: char) -> bool {
        c == 'O' || c == '[' || c == ']'
    }

    fn is_barrel_on_tile(&self, x: isize, y: isize) -> bool {
        // println!("xy {} {} ", x, y);
        let c = self.get_char_at(x, y);

        Map::is_barrel(c.unwrap())
    }

    fn set_char_at(&mut self, x: isize, y: isize, c: char) {
        if !self.in_bounds(x, y) {
            return;
        }

        self.data[(x as usize) + (y as usize) * self.width] = c;
    }

    fn get_tiles_in_direction(&self, x: isize, y: isize, dir: Direction) -> Vec<(Point, char)> {
        let mut points = Vec::new();
        let mut current_x = x;
        let mut current_y = y;
        let dir_point = dir.to_point();

        loop {
            current_x += dir_point.0;
            current_y += dir_point.1;

            if !self.in_bounds(current_x, current_y) {
                break;
            }

            let c = self.get_char_at(current_x, current_y).unwrap();

            points.push(((current_x, current_y), c));
        }

        points
    }

    fn push_barrel(&mut self, x: isize, y: isize, dir: Direction) -> bool {
        // Find the first gap in this direction before a wall
        let tiles = self.get_tiles_in_direction(x, y, dir);

        // println!("Tiles {:?}", tiles);
        for t in tiles {
            if t.1 == '#' {
                return false;
            }

            if t.1 == '.' {
                self.set_char_at(x, y, '.');
                self.set_char_at(t.0 .0, t.0 .1, 'O');
                return true;
            }
        }

        false
    }

    fn double(&self) -> Map {
        let new_map_data = self
            .data
            .iter()
            .flat_map(|c| match c {
                '#' => ['#', '#'],
                'O' => ['[', ']'],
                '.' => ['.', '.'],
                '@' => ['@', '.'],
                _ => panic!("Not sure"),
            })
            .collect();

        Map::new(new_map_data, self.width * 2, self.height)
    }
}

fn draw(map: &Map, robot: &Robot) {
    for y in 0..map.height {
        for x in 0..map.width {
            if robot.position == (x as isize, y as isize) {
                print!("@");
            } else {
                print!("{}", map.get_char_at(x as isize, y as isize).unwrap());
            }
        }
        println!("");
    }
}

fn parse(lines: &[String]) -> (Map, Vec<Direction>) {
    let (map_lines, dir_lines) = lines.split_at(lines.iter().position(|l| l.is_empty()).unwrap());

    let grid_data = map_lines
        .iter()
        .flat_map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let height = map_lines.len();
    let width = map_lines[0].len();

    let map = Map::new(grid_data, width, height);

    let instructions = dir_lines
        .iter()
        .flat_map(|l| l.chars().map(Direction::from))
        .collect();

    (map, instructions)
}

struct Robot {
    position: Point,
    map: Map,
}

impl Robot {
    fn new(mut map: Map) -> Self {
        // println!("{:?}", map);

        let start_position = map
            .find_first_position('@')
            .expect("Cannot find start position");

        let start_position = (start_position.0, start_position.1);

        map.set_char_at(start_position.0, start_position.1, '.');

        Robot {
            map: map,
            position: start_position,
        }
    }

    fn walk(&mut self, dir: Direction) {
        let dir_vector = dir.to_point();
        let target_tile = (
            self.position.0 + dir_vector.0,
            self.position.1 + dir_vector.1,
        );

        if self.map.is_wall_on_tile(target_tile.0, target_tile.1) {
            return;
        } else if self.map.is_space_on_tile(target_tile.0, target_tile.1) {
            self.position = target_tile;
        } else if self.map.is_barrel_on_tile(target_tile.0, target_tile.1) {
            if self.map.push_barrel(target_tile.0, target_tile.1, dir) {
                self.position = target_tile;
            }
        }
    }
}

fn follow_instructions(robot: &mut Robot, instructions: &[Direction]) {
    instructions.iter().for_each(|&i| robot.walk(i));
}

fn get_gps(map: &Map) -> usize {
    map.data
        .iter()
        .enumerate()
        .map(|(p, _)| {
            let x = p % map.width;
            let y = p / map.width;

            if map.is_barrel_on_tile(x as isize, y as isize) {
                return 100 * y + x;
            }

            0
        })
        .sum()
}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    let (map, instructions) = parse(&lines);
    let big_map = map.double();

    let mut robot = Robot::new(map);

    follow_instructions(&mut robot, &instructions);

    // draw(&robot.map, &robot);
    // println!("{:?}", instructions);

    let result = get_gps(&robot.map);

    println!("The sum of all GPS coordinates is {}", result);

    let mut robot = Robot::new(big_map);
    // follow_instructions(&mut robot, &instructions);
    robot.walk(Direction::Left);
    // // robot.walk(Direction::Up);
    // // robot.walk(Direction::Right);
    // // robot.walk(Direction::Right);

    draw(&robot.map, &robot);
    // let result = get_gps(&robot.map);

    println!(
        "The sum of all GPS coordinates with big boxes is {}",
        result
    );

    Ok(())
}
