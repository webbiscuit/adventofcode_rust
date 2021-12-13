use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::io::{self, prelude::*};

use itertools::Itertools;

pub struct Instruction {
    dir: char,
    val: u32,
}

impl Instruction {
    pub fn new(dir: char, val: u32) -> Self {
        Self { dir, val }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "fold along {}={}", self.dir, self.val)?;

        Ok(())
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Point {
    x: u32,
    y: u32,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Point {
        Point { x, y }
    }
}

struct TransparentPaper {
    grid: HashMap<Point, bool>,
    width: u32,
    height: u32,
}

impl TransparentPaper {
    pub fn new() -> TransparentPaper {
        TransparentPaper {
            grid: HashMap::new(),
            width: 0,
            height: 0,
        }
    }

    pub fn add_point(&mut self, point: Point) {
        self.grid.insert(point, true);
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn count_points(&self) -> u32 {
        self.grid.len() as u32
    }

    fn apply_fold(&mut self, instruction: &Instruction) {
        // let height = self.height();
        // let width = self.width();

        let mut new_grid = HashMap::new();

        for &point in self.grid.keys() {
            if instruction.dir == 'y' {
                if point.y > self.height - instruction.val - 1 {
                    let new_y = self.height - point.y - 1;

                    new_grid.insert(Point::new(point.x, new_y), true);
                } else {
                    new_grid.insert(point, true);
                }
            } else if instruction.dir == 'x' {
                if point.x > self.width - instruction.val - 1 {
                    let new_x = self.width - point.x - 1;

                    new_grid.insert(Point::new(new_x, point.y), true);
                } else {
                    new_grid.insert(point, true);
                }
            }
        }

        if instruction.dir == 'y' {
            self.height = self.height - instruction.val - 1;
        } else if instruction.dir == 'x' {
            self.width = self.width - instruction.val - 1;
        }

        self.grid = new_grid;
    }

    pub fn measure(&mut self) {
        self.width = self.grid.keys().map(|p| p.x).max().unwrap() as u32 + 1;
        self.height = self.grid.keys().map(|p| p.y).max().unwrap() as u32 + 1;
    }
}

impl fmt::Display for TransparentPaper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width() {
                let point = Point::new(x as u32, y as u32);
                if self.grid.contains_key(&point) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut paper = TransparentPaper::new();
    let mut instructions = Vec::new();

    for line in lines.flatten() {
        if line.contains(",") {
            let points = line
                .split(',')
                .map(|x| x.parse::<u32>().unwrap())
                .collect_vec();
            paper.add_point(Point::new(points[0], points[1]));
            paper.measure();
        }

        if line.contains("=") {
            let (dir, val) = line.split(" ").collect_vec()[2].split_once("=").unwrap();
            let instruction =
                Instruction::new(dir.chars().next().unwrap(), val.parse::<u32>().unwrap());
            instructions.push(instruction);
        }
    }

    // for i in instructions {
    //     println!("{}", i);
    // }

    paper.apply_fold(&instructions[0]);
    // paper.apply_fold(&instructions[1]);

    println!("Visible dots after first fold: {}", paper.count_points());

    for i in 1..instructions.len() {
        paper.apply_fold(&instructions[i]);
    }

    println!("{}", paper);

    Ok(())
}
