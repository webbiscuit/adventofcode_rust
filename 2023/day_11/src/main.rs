use std::{
    io::{self, prelude::*},
    sync::PoisonError,
};

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Image {
    galaxies: Vec<Point>,
    width: usize,
    height: usize,
}

fn parse_image(lines: &[String]) -> Image {
    let mut galaxies: Vec<Point> = Vec::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push(Point { x, y })
            }
        }
    }

    Image {
        galaxies,
        width: lines[0].len(),
        height: lines.len(),
    }
}

fn find_all_shortest_paths(image: &Image) -> Vec<usize> {
    let mut y_expansions = Vec::new();

    for y in 0..=image.height {
        if image.galaxies.iter().any(|point| point.y == y) {
            continue;
        }

        y_expansions.push(y);
    }

    let mut x_expansions = Vec::new();

    for x in 0..=image.width {
        if image.galaxies.iter().any(|point| point.x == x) {
            continue;
        }

        x_expansions.push(x);
    }

    let mut distances = Vec::new();

    for (ix, g) in image.galaxies.iter().enumerate() {
        for other_g in image.galaxies[ix + 1..].iter() {
            let mut distance_x = g.x.abs_diff(other_g.x);
            let mut distance_y = g.y.abs_diff(other_g.y);

            let first_x = g.x.min(other_g.x);
            let other_x = g.x.max(other_g.x);

            let first_y = g.y.min(other_g.y);
            let other_y = g.y.max(other_g.y);

            for x in first_x..=other_x {
                if x_expansions.contains(&x) {
                    distance_x += 1;
                }
            }

            for y in first_y..=other_y {
                if y_expansions.contains(&y) {
                    distance_y += 1;
                }
            }

            distances.push(distance_x + distance_y);
        }
    }

    distances
}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    let image = parse_image(&lines);

    // dbg!(&image);

    let shortest_paths = find_all_shortest_paths(&image);
    let sum: usize = shortest_paths.iter().sum();

    // dbg!(&shortest_paths);
    // dbg!(&shortest_paths.len());

    println!("The shortest path between all pairs of galaxies is {sum}.");

    Ok(())
}
