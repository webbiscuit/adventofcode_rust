use core::cmp::max;
use core::cmp::min;
use std::io::{self, prelude::*};

type TreeSize = u8;

struct Grid {
    width: usize,
    height: usize,
    data: Vec<TreeSize>,
}

struct Point {
    x: usize,
    y: usize,
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

    fn is_visible_from(&self, from: Point, to: Point) -> bool {
        for y in min(from.y, to.y)..=max(from.y, to.y) {
            for x in min(from.x, to.x)..=max(from.x, to.x) {
                if x == from.x && y == from.y {
                    continue;
                }
                if self.get(from.x, from.y) <= self.get(x, y) {
                    return false;
                }
            }
        }

        true
    }

    fn count_trees_visible(&self, from: Point, to: Point) -> usize {
        let mut tree_count = 0;
        // TODO

        tree_count
    }

    fn find_visible_trees_from_outside(&self) -> usize {
        let mut visible_trees = 0;

        for y in 1..self.height - 1 {
            for x in 1..self.width - 1 {
                let tree = self.get(x, y);
                if self.is_visible_from_any_direction(x, y) {
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

    fn is_visible_from_any_direction(&self, x: usize, y: usize) -> bool {
        self.is_visible_from(Point { x, y }, Point { x: 0, y })
            || self.is_visible_from(Point { x, y }, Point { x, y: 0 })
            || self.is_visible_from(
                Point { x, y },
                Point {
                    x: self.width - 1,
                    y,
                },
            )
            || self.is_visible_from(
                Point { x, y },
                Point {
                    x,
                    y: self.height - 1,
                },
            )
    }

    fn calculate_scenic_score(&self, x: usize, y: usize) -> usize {
        self.count_trees_visible(Point { x, y }, Point { x: 0, y })
            * self.count_trees_visible(Point { x, y }, Point { x, y: 0 })
            * self.count_trees_visible(
                Point { x, y },
                Point {
                    x: self.width - 1,
                    y,
                },
            )
            * self.count_trees_visible(
                Point { x, y },
                Point {
                    x,
                    y: self.height - 1,
                },
            )
    }

    fn calculate_scenic_scores(&self) -> Vec<usize> {
        let mut scores = vec![0; self.width * self.height];

        for y in 1..self.height - 1 {
            for x in 1..self.width - 1 {
                scores[y * self.width + x] = self.calculate_scenic_score(x, y);

                println!("({}, {}) = {}", x, y, scores[y * self.width + x]);
            }
        }

        scores
    }

    fn draw(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.is_visible_from_any_direction(x, y) {
                    print!("{}", self.get(x, y));
                } else {
                    print!("X");
                }
            }
            println!();
        }
    }
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

    println!(
        "The highest scenic score is {}.",
        scenic_scores.iter().max().unwrap()
    );
}
