use std::{
    collections::HashMap,
    io::{self, prelude::*},
};

#[derive(Debug, PartialEq)]
enum TileType {
    StartPos,
    VerticalPipe,
    HorizontalPipe,
    NEBendPipe,
    NWBendPipe,
    SEBendPipe,
    SWBendPipe,
    Ground,
}

impl TileType {
    fn can_flow_from
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Maze {
    tiles: HashMap<Position, TileType>,
    start_pos: Position,
}

fn parse_tile(input: &char) -> TileType {
    let tile = match input {
        '|' => TileType::VerticalPipe,
        '-' => TileType::HorizontalPipe,
        'L' => TileType::NEBendPipe,
        'J' => TileType::NWBendPipe,
        '7' => TileType::SWBendPipe,
        'F' => TileType::SEBendPipe,
        '.' => TileType::Ground,
        'S' => TileType::StartPos,
        _ => {
            panic!("Dunno what this is!")
        }
    };

    tile
}

fn parse_maze(lines: &[String]) -> Maze {
    let mut tiles: HashMap<_, _> = HashMap::new();
    let mut start_pos = Position { x: 0, y: 0 };

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let tile = parse_tile(&c);

            if tile == TileType::StartPos {
                start_pos = Position { x, y }
            }

            tiles.insert(Position { x, y }, tile);
        }
    }

    Maze { tiles, start_pos }
}

fn count_furthest_step(maze: &Maze) -> usize {
    let north = Position {
        x: maze.start_pos.x,
        y: maze.start_pos.y - 1
    };
    let south = Position {
        x: maze.start_pos.x,
        y: maze.start_pos.y + 1
    };
    let east = Position {
        x: maze.start_pos.x + 1,
        y: maze.start_pos.y
    };
    let west = Position {
        x: maze.start_pos.x - 1,
        y: maze.start_pos.y
    };

    let north_tile = maze.tiles.get(&north).unwrap();

    match north_tile {
        '|'
    }

    if maze.tiles.get(&north).unwrap()  == '|' || {

    }

    if maze.start_pos + Position { x: 1, y: 0) {

    }

    0
}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    let maze = parse_maze(&lines);

    dbg!(&maze);

    let furthest_steps = 0;

    println!("The furthest part of the loop is {furthest_steps} steps away.");

    Ok(())
}
