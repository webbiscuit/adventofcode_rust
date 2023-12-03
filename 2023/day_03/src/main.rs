use std::io::{self, prelude::*};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Position {
    x: u32,
    y: u32,
}

#[derive(Debug, Clone)]
struct Part {
    number: u32,
    start_position: Position,
    end_position: Position,
}

#[derive(Debug, Clone)]
struct Symbol {
    symbol: char,
    position: Position,
}

#[derive(Debug)]
struct Schematic {
    parts: Vec<Part>,
    symbols: Vec<Symbol>,
}

fn parse_lines(lines: &[String]) -> Schematic {
    let mut parts: Vec<Part> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    for (y, line) in lines.iter().enumerate() {
        let mut buffer: String = String::new();
        let mut reading_part = false;
        let mut start_position: Position = Position { x: 0, y: 0 };

        for (x, c) in line.chars().enumerate() {
            match c {
                '0'..='9' => {
                    buffer.push(c);

                    if !reading_part {
                        reading_part = true;
                        start_position = Position {
                            x: x as u32,
                            y: y as u32,
                        }
                    }
                }
                c => {
                    if reading_part {
                        let end_position = Position {
                            x: (x - 1) as u32,
                            y: y as u32,
                        };

                        parts.push(Part {
                            number: buffer.parse::<u32>().unwrap(),
                            start_position,
                            end_position,
                        });

                        buffer.clear();
                        reading_part = false;
                    }

                    if c == '.' {
                        continue;
                    }

                    symbols.push(Symbol {
                        symbol: c,
                        position: Position {
                            x: x as u32,
                            y: y as u32,
                        },
                    })
                }
                _ => {}
            }
        }

        if reading_part {
            let end_position = Position {
                x: (line.len() - 1) as u32,
                y: y as u32,
            };

            parts.push(Part {
                number: buffer.parse::<u32>().unwrap(),
                start_position,
                end_position,
            });
        }
    }

    Schematic { parts, symbols }
}

fn find_valid_part_numbers(schematic: &Schematic) -> Vec<&Part> {
    // Find the areas around the symbols which are classed as adjacent zones
    let mut adjacent_areas: Vec<Position> = Vec::new();

    fn get_adjacent_positions(position: &Position) -> Vec<Position> {
        let directions: [(i32, i32); 9] = [
            (1, 0),
            (1, 1),
            (0, 1),
            (0, 0),
            (-1, 0),
            (-1, -1),
            (-1, 1),
            (1, -1),
            (0, -1),
        ];

        let positions = directions
            .iter()
            .fold(Vec::<Position>::new(), |mut acc, d| {
                acc.push(Position {
                    x: (position.x as i32 + d.0) as u32,
                    y: (position.y as i32 + d.1) as u32,
                });

                acc
            });

        positions
    }

    for symbol in &schematic.symbols {
        let these_adjacent_areas = get_adjacent_positions(&symbol.position);
        adjacent_areas.extend(these_adjacent_areas);
    }

    let valid_parts = schematic
        .parts
        .iter()
        .filter(|&part| {
            adjacent_areas
                .iter()
                .any(|&p| p == part.start_position || p == part.end_position)
        })
        .collect::<Vec<_>>();

    valid_parts
}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();
    let schematic = parse_lines(&lines);

    let valid_engine_parts = find_valid_part_numbers(&schematic);

    // dbg!(valid_engine_parts);
    let sum = valid_engine_parts.iter().fold(0, |acc, &p| acc + p.number);

    println!("The sum of all parts in the engine schematic is {sum}");

    Ok(())
}
