use std::{
    collections::HashMap,
    io::{self, BufRead},
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn points_between(start: &Point, end: &Point) -> Vec<Point> {
        let mut points = vec![];

        if start.x == end.x {
            let start_y = start.y.min(end.y);
            let end_y = start.y.max(end.y);

            for y in start_y..=end_y {
                points.push(Point { x: start.x, y });
            }
        }

        if start.y == end.y {
            let start_x = start.x.min(end.x);
            let end_x = start.x.max(end.x);

            for x in start_x..=end_x {
                points.push(Point { x, y: start.y });
            }
        }

        points
    }
}

#[derive(Debug, PartialEq)]
enum Blocker {
    Wall,
    Sand,
}

struct Cave {
    sand_spawn_point: Point,
    blockers: HashMap<Point, Blocker>,
    into_the_void: bool,
    spawner_blocked: bool,
}

impl Cave {
    fn new() -> Self {
        Self {
            blockers: HashMap::new(),
            sand_spawn_point: Point { x: 500, y: 0 },
            into_the_void: false,
            spawner_blocked: false,
        }
    }

    fn add_walls(&mut self, start: &Point, end: &Point) {
        let points = Point::points_between(start, end);
        self.blockers
            .extend(points.into_iter().map(|p| (p, Blocker::Wall)));
    }

    fn spawn_sand(&mut self) {
        let mut sand = self.sand_spawn_point.clone();
        let the_void_start = self.get_max_y() + 1;

        loop {
            let first_free_point = self.find_first_free_point(&vec![
                Point {
                    x: sand.x,
                    y: sand.y + 1,
                },
                Point {
                    x: sand.x - 1,
                    y: sand.y + 1,
                },
                Point {
                    x: sand.x + 1,
                    y: sand.y + 1,
                },
            ]);

            if let Some(first_free_point) = first_free_point {
                sand = first_free_point;
            } else {
                self.blockers.insert(sand, Blocker::Sand);

                if sand == self.sand_spawn_point {
                    self.spawner_blocked = true;
                }

                break;
            }

            if sand.y >= the_void_start {
                self.into_the_void = true;
                break;
            }
        }
    }

    fn find_first_free_point(&self, points: &Vec<Point>) -> Option<Point> {
        for point in points {
            if self.get_blocker(point).is_none() {
                return Some(*point);
            }
        }

        None
    }

    fn get_blocker(&self, point: &Point) -> Option<&Blocker> {
        self.blockers.get(&point)
    }

    fn get_max_y(&self) -> usize {
        self.blockers.iter().max_by_key(|p| p.0.y).unwrap().0.y
    }

    fn get_min_y(&self) -> usize {
        self.blockers.iter().min_by_key(|p| p.0.y).unwrap().0.y
    }

    fn get_min_x(&self) -> usize {
        self.blockers.iter().min_by_key(|p| p.0.x).unwrap().0.x
    }

    fn get_max_x(&self) -> usize {
        self.blockers.iter().max_by_key(|p| p.0.x).unwrap().0.x
    }

    fn count_sand(&self) -> usize {
        self.blockers
            .iter()
            .filter(|b| b.1 == &Blocker::Sand)
            .count()
    }

    fn into_the_void(&self) -> bool {
        self.into_the_void
    }

    fn spawner_blocked(&self) -> bool {
        self.spawner_blocked
    }

    fn add_floor(&mut self) {
        // This is the min/max that should work for all inputs but the drawing is not very nice
        let min_x = 500 - (self.get_max_x() / 2); // self.get_min_x();
        let min_y = 0;
        let max_x = 500 + (self.get_max_x() / 2); // self.get_max_x();
        let max_y = self.get_max_y();

        self.add_walls(
            &Point {
                x: min_x,
                y: max_y + 2,
            },
            &Point {
                x: max_x,
                y: max_y + 2,
            },
        );
    }

    fn draw(&self) {
        let min_x = self.get_min_x();
        let min_y = 0;
        let max_x = self.get_max_x();
        let max_y = self.get_max_y() + 2;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if self.sand_spawn_point.x == x && self.sand_spawn_point.y == y {
                    print!("+");
                    continue;
                }

                let blocker = self.get_blocker(&Point { x, y });

                if let Some(Blocker::Wall) = blocker {
                    print!("#");
                } else if let Some(Blocker::Sand) = blocker {
                    print!("o");
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

    let mut cave_lines = Vec::new();

    for line in lines {
        let parts = line.split(" -> ").collect::<Vec<_>>();
        let mut cave_line = Vec::new();

        for part in parts {
            let part = part.split(',').collect::<Vec<_>>();

            let start: usize = part[0].parse().unwrap();
            let end: usize = part[1].parse().unwrap();

            cave_line.push(Point { x: start, y: end });
        }
        cave_lines.push(cave_line);
    }

    let mut cave = Cave::new();

    for cave_line in cave_lines {
        for points in cave_line.windows(2) {
            let start = &points[0];
            let end = &points[1];

            cave.add_walls(start, end);
        }
    }

    while !cave.into_the_void() {
        cave.spawn_sand();
    }

    let sand_count = cave.count_sand();

    println!(
        "There are {} grains of sand before they fall into the void.",
        sand_count
    );

    cave.add_floor();

    // cave.draw();

    while !cave.spawner_blocked() {
        cave.spawn_sand();
    }

    // cave.draw();

    let sand_count2 = cave.count_sand();

    println!(
        "With a floor, there are {} grains of sand before the source is blocked.",
        sand_count2
    );
}
