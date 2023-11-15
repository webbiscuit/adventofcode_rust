use std::{
    collections::HashSet,
    io::{self, BufRead},
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Cube {
    x: u8,
    y: u8,
    z: u8,
}

impl Cube {
    fn neighbors(&self, other_cubes: &HashSet<Cube>) -> HashSet<Cube> {
        let mut neighbors = HashSet::new();

        for x in [self.x.saturating_sub(1), self.x + 1].iter() {
            let cube_to_try = Cube {
                x: *x,
                y: self.y,
                z: self.z,
            };
            if other_cubes.contains(&cube_to_try) {
                neighbors.insert(cube_to_try);
            }
        }

        for y in [self.y.saturating_sub(1), self.y + 1].iter() {
            let cube_to_try = Cube {
                x: self.x,
                y: *y,
                z: self.z,
            };
            if other_cubes.contains(&cube_to_try) {
                neighbors.insert(cube_to_try);
            }
        }

        for z in [self.z.saturating_sub(1), self.z + 1].iter() {
            let cube_to_try = Cube {
                x: self.x,
                y: self.y,
                z: *z,
            };
            if other_cubes.contains(&cube_to_try) {
                neighbors.insert(cube_to_try);
            }
        }

        if neighbors.contains(&self) {
            neighbors.remove(&self);
        }

        neighbors
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    let mut cubes = HashSet::new();

    for line in lines {
        let mut parts = line.split(",");
        let x = parts.next().unwrap().parse::<u8>().unwrap();
        let y = parts.next().unwrap().parse::<u8>().unwrap();
        let z = parts.next().unwrap().parse::<u8>().unwrap();
        cubes.insert(Cube { x, y, z });
    }

    let mut surface_area = 0;

    for cube in &cubes {
        println!("Cube {:?}", cube);
    }

    for cube in &cubes {
        let neighbors = cube.neighbors(&cubes);
        println!("Cube {:?} has {} neighbors.", cube, neighbors.len());
        println!("Neighbors: {:?}", neighbors);
        surface_area += 6 - neighbors.len();
    }

    println!("The total surface area is {}.", surface_area)
}
