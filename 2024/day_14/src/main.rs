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

    // println!("TL {:?}", tl);
    // println!("TR {:?}", tr);
    // println!("BL {:?}", bl);
    // println!("BR {:?}", br);

    tl.len() * tr.len() * bl.len() * br.len()
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

    // let mut robot = Robot::new((2, 4), (2, -3));
    // robot.teleport(width, height, 5);

    // println!("{:?}", robots);

    Ok(())
}
