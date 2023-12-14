#![allow(clippy::needless_range_loop)]

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::{
    collections::hash_map::DefaultHasher,
    io::{self, prelude::*},
};

use bitmaps::Bitmap;

#[derive(Debug, PartialEq, Clone, Hash)]
enum Tile {
    RoundRock,
    CubeRock,
    Floor,
}

#[derive(Debug, Clone)]
struct Map {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Map {
    const SIZE: usize = 100;

    #[allow(dead_code)]
    fn draw(&self) {
        for h in 0..self.height {
            for w in 0..self.width {
                let tile = &self.tiles[w + (h * self.width)];

                let c = match tile {
                    Tile::Floor => '.',
                    Tile::CubeRock => '#',
                    Tile::RoundRock => 'O',
                };

                print!("{}", c);
            }
            println!();
        }
    }

    fn calculate_load(&self) -> usize {
        let mut sum = 0;
        for h in 0..self.height {
            for w in 0..self.width {
                let tile = &self.tiles[w + (h * self.width)];

                if tile == &Tile::RoundRock {
                    // let score = self.height - h;
                    // dbg!(score);
                    sum += self.height - h
                }
            }
        }
        sum
    }

    fn to_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.tiles.hash(&mut hasher);
        hasher.finish()
    }

    fn tilt(&mut self, direction: Direction) {
        let mut bitmaps: Vec<Bitmap<{ Self::SIZE }>> = vec![Bitmap::new(); self.height];

        for h in 0..self.height {
            for w in 0..self.width {
                let tile = &self.tiles[w + (h * self.width)];

                if tile != &Tile::Floor {
                    bitmaps[h].set(w, true);
                }
            }
        }

        match direction {
            Direction::North => self.tilt_north(&mut bitmaps),
            Direction::South => self.tilt_south(&mut bitmaps),
            Direction::East => self.tilt_east(&mut bitmaps),
            Direction::West => self.tilt_west(&mut bitmaps),
        }

        for h in 0..self.height {
            let bitmap = &bitmaps[h];

            for w in 0..self.width {
                let tile = &mut self.tiles[w + (h * self.width)];

                if tile != &Tile::CubeRock {
                    if bitmap.get(w) {
                        *tile = Tile::RoundRock;
                    } else {
                        *tile = Tile::Floor;
                    }
                }
            }
        }
    }

    fn tilt_north(&self, bitmaps: &mut [Bitmap<{ Self::SIZE }>]) {
        for w in 0..self.width {
            let mut next_free_space = 0;

            for h in 0..self.height {
                let tile = &self.tiles[w + (h * self.width)];

                if tile == &Tile::CubeRock {
                    next_free_space = h + 1;
                } else if tile == &Tile::RoundRock {
                    bitmaps[h].set(w, false);
                    bitmaps[next_free_space].set(w, true);
                    next_free_space += 1;
                }
            }
        }
    }
    fn tilt_south(&self, bitmaps: &mut [Bitmap<{ Self::SIZE }>]) {
        for w in 0..self.width {
            let mut next_free_space = self.height;

            for h in (0..self.height).rev() {
                let tile = &self.tiles[w + (h * self.width)];

                if tile == &Tile::CubeRock {
                    next_free_space = h;
                } else if tile == &Tile::RoundRock {
                    next_free_space -= 1;
                    bitmaps[h].set(w, false);
                    bitmaps[next_free_space].set(w, true);
                }
            }
        }
    }
    fn tilt_east(&self, bitmaps: &mut [Bitmap<{ Self::SIZE }>]) {
        for h in 0..self.height {
            let mut next_free_space = self.width;

            for w in (0..self.width).rev() {
                let tile = &self.tiles[w + (h * self.width)];

                if tile == &Tile::CubeRock {
                    next_free_space = w;
                } else if tile == &Tile::RoundRock {
                    next_free_space -= 1;
                    bitmaps[h].set(w, false);
                    bitmaps[h].set(next_free_space, true);
                }
            }
        }
    }
    fn tilt_west(&self, bitmaps: &mut [Bitmap<{ Self::SIZE }>]) {
        for h in 0..self.height {
            let mut next_free_space = 0;

            for w in 0..self.width {
                let tile = &self.tiles[w + (h * self.width)];

                if tile == &Tile::CubeRock {
                    next_free_space = w + 1;
                } else if tile == &Tile::RoundRock {
                    bitmaps[h].set(w, false);
                    bitmaps[h].set(next_free_space, true);
                    next_free_space += 1;
                }
            }
        }
    }
}

fn parse_lines(lines: &[String]) -> Map {
    let mut tiles = Vec::new();

    for line in lines {
        for c in line.chars() {
            if c == 'O' {
                tiles.push(Tile::RoundRock);
            } else if c == '#' {
                tiles.push(Tile::CubeRock);
            } else if c == '.' {
                tiles.push(Tile::Floor);
            }
        }
    }

    Map {
        width: lines[0].len(),
        height: lines.len(),
        tiles,
    }
}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    let mut map = parse_lines(&lines);

    map.tilt(Direction::North);
    let load = map.calculate_load();

    println!("The total load after tilting north is {load}.");

    let directions = [
        Direction::North,
        Direction::West,
        Direction::South,
        Direction::East,
    ];

    let mut count = 0;
    let mut seen_states: HashMap<u64, usize> = HashMap::new();
    const MAX_COUNT: usize = 1000000000;

    while count < MAX_COUNT {
        let state_hash = map.to_hash();
        if let Some(&first_seen) = seen_states.get(&state_hash) {
            // Cycle detected
            let cycle_length = count - first_seen;
            let remaining_cycles = (MAX_COUNT - count) / cycle_length;
            count += remaining_cycles * cycle_length;
            break;
        } else {
            seen_states.insert(state_hash, count);
        }

        for &direction in &directions {
            map.tilt(direction);
        }

        count += 1;
    }

    let remaining_iterations = MAX_COUNT - count;
    for _ in 0..remaining_iterations {
        for &direction in &directions {
            map.tilt(direction);
        }
    }

    // map.draw();

    let load = map.calculate_load();

    println!("The total load after tilting a lot is {load}.");

    Ok(())
}
