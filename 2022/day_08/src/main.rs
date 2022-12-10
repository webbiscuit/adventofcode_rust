use std::io::{self, prelude::*};

type TreeSize = u8;

struct Grid {
    width: usize,
    height: usize,
    data: Vec<TreeSize>,
}

struct Point {
    x: i32,
    y: i32,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![0; width * height],
        }
    }

    fn get(&self, x: usize, y: usize) -> TreeSize {
        self.data[y * self.width + x]
    }

    fn set(&mut self, x: usize, y: usize, value: TreeSize) {
        self.data[y * self.width + x] = value;
    }

    fn plant_trees(&mut self, line: &str, row: usize) {
        let y = row;
        for (x, c) in line.chars().enumerate() {
            self.set(x, y, c.to_digit(10).unwrap() as TreeSize);
        }
    }

    fn find_visible_trees_from_outside(&self) -> usize {
        let mut visible_trees = 0;

        for y in 1..self.height - 1 {
            for x in 1..self.width - 1 {
                if self.is_visible_from_any_direction(&Point {
                    x: x as i32,
                    y: y as i32,
                }) {
                    visible_trees += 1;
                    // println!(
                    //     "Tree at ({}, {}) {} is visible from the outside.",
                    //     x, y, tree
                    // );
                }
            }
        }

        visible_trees + self.outside_tree_count()
    }

    fn outside_tree_count(&self) -> usize {
        self.height * 2 + self.width * 2 - 4
    }

    fn get_trees_in_line(&self, from: &Point, dir: &Direction) -> Vec<TreeSize> {
        let dir = match dir {
            Direction::Up => Point { x: 0, y: -1 },
            Direction::Down => Point { x: 0, y: 1 },
            Direction::Left => Point { x: -1, y: 0 },
            Direction::Right => Point { x: 1, y: 0 },
        };

        let mut tree_sizes = Vec::new();

        let mut next = Point {
            x: from.x + dir.x,
            y: from.y + dir.y,
        };

        loop {
            if next.x < 0
                || next.x >= self.width as i32
                || next.y < 0
                || next.y >= self.height as i32
            {
                break;
            }
            tree_sizes.push(self.get(next.x as usize, next.y as usize));

            next = Point {
                x: next.x + dir.x,
                y: next.y + dir.y,
            };
        }

        tree_sizes
    }

    fn is_visible_from_any_direction(&self, point: &Point) -> bool {
        let tree = self.get(point.x as usize, point.y as usize);

        for dir in &[
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            let trees_in_line = self.get_trees_in_line(point, dir);
            let highest_tree = trees_in_line.iter().max().unwrap();
            if &tree > highest_tree {
                return true;
            }
        }

        false
    }

    fn calculate_scenic_score(&self, from: &Point) -> usize {
        let mut scenic_score = 1;

        for dir in &[
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            let this_tree = self.get(from.x as usize, from.y as usize);
            let trees_in_line = self.get_trees_in_line(from, dir);

            let score = trees_in_line
                .iter()
                .position(|&x| x >= this_tree)
                .unwrap_or(trees_in_line.len() - 1)
                + 1;

            scenic_score *= score;
        }

        scenic_score
    }

    fn calculate_scenic_scores(&self) -> Vec<usize> {
        let mut scores = vec![0; self.width * self.height];

        for y in 1..self.height - 1 {
            for x in 1..self.width - 1 {
                scores[y * self.width + x] = self.calculate_scenic_score(&Point {
                    x: x as i32,
                    y: y as i32,
                });

                //println!("({}, {}) = {}", x, y, scores[y * self.width + x]);
            }
        }

        scores
    }

    fn draw_scenic_scores(&self) {
        let scores = self.calculate_scenic_scores();

        for y in 1..self.height - 1 {
            for x in 1..self.width - 1 {
                print!("{}", scores[y * self.width + x]);
            }
            println!();
        }
    }

    // fn draw(&self) {
    //     for y in 0..self.height {
    //         for x in 0..self.width {
    //             if self.is_visible_from_any_direction(x, y) {
    //                 print!("{}", self.get(x, y));
    //             } else {
    //                 print!("X");
    //             }
    //         }
    //         println!();
    //     }
    // }
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    let grid_width = lines[0].len();
    let grid_height = lines.len();

    let mut grid = Grid::new(grid_width, grid_height);

    for (ix, line) in lines.iter().enumerate() {
        grid.plant_trees(line, ix);
    }

    // grid.draw();

    let visible_trees = grid.find_visible_trees_from_outside();

    println!(
        "There are {} trees visible from the outside of the grid.",
        visible_trees,
    );

    let scenic_scores = grid.calculate_scenic_scores();

    //grid.draw_scenic_scores();

    println!(
        "The highest scenic score is {}.",
        scenic_scores.iter().max().unwrap()
    );
}
