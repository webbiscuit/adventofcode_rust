use std::{
    env::args,
    io::{self, prelude::*},
};

use regex::Regex;

type Point = (isize, isize);

#[derive(Debug)]
struct Robot {
    position: Point,
    velocity: Point,
}

impl Robot {
    fn teleport(&mut self, width: usize, height: usize, seconds: usize) {
        let move_velocity = (
            self.velocity.0 * seconds as isize,
            self.velocity.1 * seconds as isize,
        );

        let new_x = (self.position.0 + move_velocity.0).rem_euclid(width as isize);
        let new_y = (self.position.1 + move_velocity.1).rem_euclid(height as isize);

        self.position = (new_x, new_y);
    }
}

fn parse(lines: &[String]) -> Vec<Robot> {
    let re = Regex::new(r"(-?\d+)").expect("Invalid regex");

    lines
        .iter()
        .map(|l| {
            let parsed = re
                .captures_iter(l)
                .map(|mul| mul[1].parse::<isize>().unwrap())
                .collect::<Vec<isize>>();

            assert!(parsed.len() == 4);

            Robot {
                position: (parsed[0], parsed[1]),
                velocity: (parsed[2], parsed[3]),
            }
        })
        .collect()
}

fn simulate_robots(robots: &mut Vec<Robot>, width: usize, height: usize, seconds: usize) {
    robots
        .iter_mut()
        .for_each(|r| r.teleport(width, height, seconds));
}

fn count_robots_in_quadrants(robots: &[Robot], width: usize, height: usize) -> usize {
    let mut tl: Vec<&Robot> = vec![];
    let mut tr: Vec<&Robot> = vec![];
    let mut bl: Vec<&Robot> = vec![];
    let mut br: Vec<&Robot> = vec![];

    let half_w = width / 2;
    let half_h = height / 2;

    robots.iter().for_each(|r| {
        if r.position.0 < half_w as isize {
            if r.position.1 < half_h as isize {
                tl.push(r);
            }

            if r.position.1 > half_h as isize {
                bl.push(r);
            }
        }

        if r.position.0 > half_w as isize {
            if r.position.1 < half_h as isize {
                tr.push(r);
            }

            if r.position.1 > half_h as isize {
                br.push(r);
            }
        }
    });

    tl.len() * tr.len() * bl.len() * br.len()
}

fn draw(robots: &[Robot], width: usize, height: usize) {
    let mut grid = vec![vec!['.'; width]; height];

    for robot in robots {
        let (x, y) = robot.position;
        grid[y as usize][x as usize] = 'R';
    }

    for row in grid {
        let line: String = row.iter().collect();
        println!("{}", line);
    }
}

fn are_many_robots_adjacent(robots: &[Robot]) -> bool {
    let mut positions = robots.iter().map(|r| r.position).collect::<Vec<_>>();
    positions.sort();

    for window in positions.windows(10) {
        let mut adjacent = true;
        for i in 0..9 {
            if (window[i].0 - window[i + 1].0).abs() > 1
                || (window[i].1 - window[i + 1].1).abs() > 1
            {
                adjacent = false;
                break;
            }
        }
        if adjacent {
            return true;
        }
    }

    false
}

fn detect_christmas_tree(robots: &mut Vec<Robot>, width: usize, height: usize) -> usize {
    let mut seconds = 0;

    loop {
        seconds += 1;
        simulate_robots(robots, width, height, 1);

        // println!("Seconds: {}", seconds);

        // draw(robots, width, height);

        if are_many_robots_adjacent(robots) {
            // draw(robots, width, height);

            break;
        }
    }

    seconds
}

fn main() -> std::io::Result<()> {
    let width: usize = args().nth(1).unwrap_or(101.to_string()).parse().unwrap();
    let height: usize = args().nth(2).unwrap_or(103.to_string()).parse().unwrap();

    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();
    let mut robots = parse(&lines);

    simulate_robots(&mut robots, width, height, 100);
    let result = count_robots_in_quadrants(&robots, width, height);

    println!("Safety factor is {}", result);

    let mut robots = parse(&lines);

    let result2 = detect_christmas_tree(&mut robots, width, height);

    draw(&robots, width, height);

    println!("A christmas tree is made in {} seconds", result2);

    Ok(())
}
