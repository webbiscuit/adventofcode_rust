use std::io::{self, prelude::*};

use bitmaps::Bitmap;

#[derive(Debug, PartialEq)]
enum Tile {
    RoundRock,
    CubeRock,
    Floor,
}

struct Map {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl Map {
    fn draw(&self) {
        for h in 0..self.height {
            for w in 0..self.width {
                let tile = &self.tiles[w + (h * self.width)];

                let c = match tile {
                    Tile::Floor => '.',
                    Tile::CubeRock => '#',
                    Tile::RoundRock => 'O',
                    _ => '.',
                };

                print!("{}", c);
            }
            println!("");
        }
    }

    fn calculate_load(&self) -> usize {
        let mut sum = 0;
        for h in 0..self.height {
            for w in 0..self.width {
                let tile = &self.tiles[w + (h * self.width)];

                if tile == &Tile::RoundRock {
                    sum += self.height - h
                }
            }
        }
        sum
    }

    fn tilt_north(&mut self) {
        const SIZE: usize = 100;

        let mut bitmaps: Vec<Bitmap<SIZE>> = vec![Bitmap::new(); self.height];

        for h in 0..self.height {
            for w in 0..self.width {
                let tile = &self.tiles[w + (h * self.width)];

                if tile != &Tile::Floor {
                    bitmaps[h].set(w, true);
                }
            }
        }

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

        // dbg!(&bitmaps);
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

    // map.draw();
    map.tilt_north();
    // map.draw();

    let load = map.calculate_load();

    println!("The total load after tilting north is {load}.");

    Ok(())
}
