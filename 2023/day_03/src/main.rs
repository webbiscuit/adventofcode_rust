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

fn get_adjacent_positions(position: &Position) -> Vec<Position> {
    let directions: [(i32, i32); 8] = [
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 0),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (0, -1),
    ];

    directions
        .iter()
        .map(|&(dx, dy)| Position {
            x: (position.x as i32 + dx) as u32,
            y: (position.y as i32 + dy) as u32,
        })
        .collect()
}

fn find_valid_part_numbers(schematic: &Schematic) -> Vec<&Part> {
    // Find the areas around the symbols which are classed as adjacent zones
    let adjacent_areas: Vec<Position> = schematic
        .symbols
        .iter()
        .flat_map(|s| get_adjacent_positions(&s.position))
        .collect();

    // dbg!(&adjacent_areas);

    let valid_parts = schematic
        .parts
        .iter()
        .filter(|&part| {
            adjacent_areas
                .iter()
                .any(|&p| p == part.start_position || p == part.end_position)
        })
        .collect::<Vec<_>>();

    // dbg!(&valid_parts);

    valid_parts
}

fn find_valid_gear_parts(schematic: &Schematic) -> Vec<(&Part, &Part)> {
    let gears = schematic.symbols.iter().filter(|s| s.symbol == '*');

    let mut adjacent_part_pairs: Vec<(&Part, &Part)> = Vec::new();

    gears.for_each(|gear: &Symbol| {
        let adjacent_areas = get_adjacent_positions(&gear.position);

        let mut adjacent_parts: Vec<&Part> = Vec::new();

        for part in &schematic.parts {
            if adjacent_areas
                .iter()
                .any(|&p| p == part.start_position || p == part.end_position)
            {
                adjacent_parts.push(part);
            }
        }

        if adjacent_parts.len() == 2 {
            adjacent_part_pairs.push((adjacent_parts[0], adjacent_parts[1]))
        }
    });

    adjacent_part_pairs
}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();
    let schematic = parse_lines(&lines);

    let valid_engine_parts = find_valid_part_numbers(&schematic);

    let sum: u32 = valid_engine_parts.iter().map(|&p| p.number).sum();

    println!("The sum of all parts in the engine schematic is {sum}");

    let valid_gear_parts = find_valid_gear_parts(&schematic);

    let ratio_sum: u32 = valid_gear_parts
        .iter()
        .map(|(p1, p2)| p1.number * p2.number)
        .sum();

    println!("Sum of all gear ratios produces {ratio_sum}");

    Ok(())
}
