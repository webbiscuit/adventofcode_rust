use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::io::{self, prelude::*};

struct CaveMaze {
    paths: HashMap<String, Vec<String>>,
}

impl CaveMaze {
    fn new() -> CaveMaze {
        let paths = HashMap::new();

        CaveMaze { paths }
    }

    pub fn add_path(&mut self, src: &str, dest: &str) {
        self.paths
            .entry(src.to_string())
            .or_insert(Vec::new())
            .push(dest.to_string());
        self.paths
            .entry(dest.to_string())
            .or_insert(Vec::new())
            .push(src.to_string());
    }

    pub fn explore_caves(&self, max_small_cave_visits: u8) -> Vec<Vec<String>> {
        let mut paths: Vec<(Vec<String>, i8)> =
            vec![(vec!["start".to_string()], max_small_cave_visits as i8)];
        let mut complete_paths: Vec<Vec<String>> = vec![];

        while !paths.is_empty() {
            let (path, max_visits) = paths.pop().unwrap();
            let last_cave = path.last().unwrap();

            let exits = self.paths.get(last_cave).unwrap();

            for exit in exits {
                if exit == "end" {
                    let mut complete_path = path.clone();
                    complete_path.push(exit.to_string());

                    complete_paths.push(complete_path);
                } else if exit == "start" {
                    continue;
                } else if exit.chars().all(|c| c.is_ascii_uppercase()) || !path.contains(exit) {
                    let mut semi_complete_path = path.clone();
                    semi_complete_path.push(exit.to_string());

                    // println!("{:?}", semi_complete_path);
                    // println!("{:?}", max_visits);

                    paths.push((semi_complete_path, max_visits));
                } else if max_visits > 1 {
                    let mut semi_complete_path = path.clone();
                    semi_complete_path.push(exit.to_string());

                    // println!("{:?}", semi_complete_path);
                    // println!("{:?}", max_visits);

                    paths.push((semi_complete_path, max_visits - 1));
                }
            }
        }

        // println!("{:?}", complete_paths);

        complete_paths
    }
}

impl fmt::Display for CaveMaze {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (k, v) in self.paths.iter() {
            for v2 in v {
                writeln!(f, "{} -> {}", k, v2)?;
            }
        }

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut maze = CaveMaze::new();

    for line in lines.flatten() {
        let (src, dest) = line.split_once('-').unwrap();
        maze.add_path(src, dest);
    }

    let paths = maze.explore_caves(1);
    let path_count = paths.len();

    println!("Paths through the cave system: {}", path_count);

    let paths2 = maze.explore_caves(2);
    let path_count2 = paths2.len();

    println!(
        "Paths through the cave system with 2 small cave visits: {}",
        path_count2
    );

    Ok(())
}
