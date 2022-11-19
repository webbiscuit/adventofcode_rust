use std::error::Error;
use std::io;

use regex::Regex;

struct LightGrid {
    lights: [[bool; 1000]; 1000],
}

impl LightGrid {
    fn new() -> LightGrid {
        LightGrid {
            lights: [[false; 1000]; 1000],
        }
    }

    fn turn_on(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        for x in x1..=x2 {
            for y in y1..=y2 {
                self.lights[x][y] = true;
            }
        }
    }

    fn turn_off(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        for x in x1..=x2 {
            for y in y1..=y2 {
                self.lights[x][y] = false;
            }
        }
    }

    fn toggle(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        for x in x1..=x2 {
            for y in y1..=y2 {
                self.lights[x][y] = !self.lights[x][y];
            }
        }
    }

    fn count_lit(&self) -> usize {
        let mut count = 0;
        for x in 0..1000 {
            for y in 0..1000 {
                if self.lights[x][y] {
                    count += 1;
                }
            }
        }
        count
    }
}

struct BrightGrid {
    lights: [[u16; 1000]; 1000],
}

impl BrightGrid {
    fn new() -> BrightGrid {
        BrightGrid {
            lights: [[0; 1000]; 1000],
        }
    }

    fn turn_on(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        for x in x1..=x2 {
            for y in y1..=y2 {
                self.lights[x][y] += 1;
            }
        }
    }

    fn turn_off(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        for x in x1..=x2 {
            for y in y1..=y2 {
                self.lights[x][y] = self.lights[x][y].saturating_sub(1);
            }
        }
    }

    fn toggle(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        for x in x1..=x2 {
            for y in y1..=y2 {
                self.lights[x][y] += 2;
            }
        }
    }

    fn brightness(&self) -> usize {
        let mut count = 0;
        for x in 0..1000 {
            for y in 0..1000 {
                count += self.lights[x][y] as usize;
            }
        }
        count
    }
}

#[derive(Debug)]
enum Commands {
    TurnOn(usize, usize, usize, usize),
    TurnOff(usize, usize, usize, usize),
    Toggle(usize, usize, usize, usize),
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines = io::stdin().lines();

    let re = Regex::new(r"(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)").unwrap();

    let commands = lines.map(|l| {
        let line = l.unwrap();

        re.captures(&line).map(|c| {
            let command = c[1].to_string();
            let x1 = c[2].parse::<usize>().unwrap();
            let y1 = c[3].parse::<usize>().unwrap();
            let x2 = c[4].parse::<usize>().unwrap();
            let y2 = c[5].parse::<usize>().unwrap();

            match command.as_str() {
                "turn on" => Commands::TurnOn(x1, y1, x2, y2),
                "turn off" => Commands::TurnOff(x1, y1, x2, y2),
                "toggle" => Commands::Toggle(x1, y1, x2, y2),
                _ => panic!("Unknown command"),
            }
        })
    });

    let mut grid = LightGrid::new();
    let mut bright_grid = BrightGrid::new();

    for command in commands {
        match command {
            Some(Commands::TurnOn(x1, y1, x2, y2)) => {
                grid.turn_on(x1, y1, x2, y2);
                bright_grid.turn_on(x1, y1, x2, y2);
            }
            Some(Commands::TurnOff(x1, y1, x2, y2)) => {
                grid.turn_off(x1, y1, x2, y2);
                bright_grid.turn_off(x1, y1, x2, y2);
            }
            Some(Commands::Toggle(x1, y1, x2, y2)) => {
                grid.toggle(x1, y1, x2, y2);
                bright_grid.toggle(x1, y1, x2, y2);
            }
            None => (),
        }
    }

    println!("There are {} lights lit", grid.count_lit());
    println!("The brightness is {}", bright_grid.brightness());

    Ok(())
}
