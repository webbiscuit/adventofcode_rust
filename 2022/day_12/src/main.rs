use std::{
    collections::HashMap,
    io::{self, prelude::*},
};

use pathfinding::prelude::astar;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Node {
    height: usize,

    steps_from_start: usize,
    steps_to_end: usize,
}

impl Node {
    fn new(height: usize) -> Node {
        Node {
            height,
            steps_from_start: 0,
            steps_to_end: 0,
        }
    }

    fn score(&self) -> usize {
        self.steps_from_start + self.steps_to_end
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn steps_to(&self, other: &Position) -> usize {
        let x_diff = (self.x as isize - other.x as isize).abs() as usize;
        let y_diff = (self.y as isize - other.y as isize).abs() as usize;

        x_diff + y_diff
    }
}

type Height = usize;

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    start: Position,
    end: Position,
    node_heights: HashMap<Position, Height>,
}

impl Grid {
    fn new(lines: &Vec<String>) -> Grid {
        let width = lines[0].len();
        let height = lines.len();
        let mut start: Position = Position { x: 0, y: 0 };
        let mut end: Position = Position { x: 0, y: 0 };
        let mut node_heights: HashMap<Position, Height> = HashMap::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let mut height = 0;

                match c {
                    'S' => {
                        start = Position { x, y };
                        height = 0;
                    }
                    'E' => {
                        end = Position { x, y };
                        height = 'z' as usize - 'a' as usize;
                    }
                    _ => {
                        height = c as usize - 'a' as usize;
                    }
                }

                node_heights.insert(Position { x, y }, height);
            }
        }

        Grid {
            width,
            height,
            start,
            end,
            node_heights,
        }
    }

    fn get_node_height(&self, x: usize, y: usize) -> Height {
        self.node_heights[&Position { x, y }]
    }

    fn walk_grid(&self, walker: &mut GridWalker) {
        walker.walk(self);
    }

    fn get_neighbors(&self, pos: Position) -> Vec<Position> {
        let mut neighbors = vec![];

        if pos.x > 0 {
            neighbors.push(Position {
                x: pos.x - 1,
                y: pos.y,
            });
        }

        if pos.x < self.width - 1 {
            neighbors.push(Position {
                x: pos.x + 1,
                y: pos.y,
            });
        }

        if pos.y > 0 {
            neighbors.push(Position {
                x: pos.x,
                y: pos.y - 1,
            });
        }

        if pos.y < self.height - 1 {
            neighbors.push(Position {
                x: pos.x,
                y: pos.y + 1,
            });
        }

        neighbors
    }

    fn draw(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.start.x == x && self.start.y == y {
                    print!("S");
                    continue;
                }

                if self.end.x == x && self.end.y == y {
                    print!("E");
                    continue;
                }

                let height = self.get_node_height(x, y);
                let c = (height + 'a' as usize) as u8 as char;
                print!("{}", c);
            }
            println!();
        }
    }
}

struct GridWalker {
    path: Option<(Vec<Position>, usize)>,
}

impl GridWalker {
    fn new() -> GridWalker {
        GridWalker { path: None }
    }

    fn walk(&mut self, grid: &Grid) {
        let start = grid.start;
        let goal = grid.end;

        self.path = astar(
            &start,
            |p| {
                grid.get_neighbors(*p)
                    .iter()
                    .map(|p| (*p, 1))
                    .collect::<Vec<_>>()
            },
            |p| p.steps_to(&goal),
            |p| *p == goal,
        );

        println!("{:?}", self.path);
    }

    fn draw(&self, grid: &Grid) {
        let mut new_paths: Vec<(Position, &str)> = Vec::new();

        if let Some(paths) = &self.path {
            for positions in paths.0.windows(2) {
                let p1 = positions[0];
                let p2 = positions[1];

                if p1.x > p2.x {
                    new_paths.push((p1, "←"));
                } else if p1.x < p2.x {
                    new_paths.push((p1, "→"));
                } else if p1.y > p2.y {
                    new_paths.push((p1, "↑"));
                } else if p1.y < p2.y {
                    new_paths.push((p1, "↓"));
                }
            }
        }

        for y in 0..grid.height {
            for x in 0..grid.width {
                let found = new_paths.iter().find(|(p, _)| p.x == x && p.y == y);

                if let Some((_, c)) = found {
                    print!("{}", c);
                    continue;
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    let grid = Grid::new(&lines);
    let mut walker = GridWalker::new();
    grid.walk_grid(&mut walker);

    println!("{:?}", grid);
    println!("{:?}", grid.get_node_height(0, 0));

    grid.draw();
    walker.draw(&grid);

    let fewest_steps = 0;

    println!(
        "The fewest steps to get to the best signal are {}.",
        fewest_steps
    );
}
