use std::{
    collections::HashSet,
    io::{self, prelude::*},
};

enum Command {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Vector {
    x: i32,
    y: i32,
}

struct Rope {
    knots: Vec<Vector>,

    tail_visited: HashSet<Vector>,
}

impl Rope {
    fn new(knot_count: usize) -> Self {
        Self {
            knots: vec![Vector { x: 0, y: 0 }; knot_count],
            tail_visited: HashSet::new(),
        }
    }

    fn head(&self) -> Vector {
        self.knots[0]
    }

    fn tail(&self) -> Vector {
        *self.knots.last().unwrap()
    }

    fn process_command(&mut self, command: &Command) {
        match command {
            Command::Up(value) => self.move_head(*value, (0, 1)),
            Command::Down(value) => self.move_head(*value, (0, -1)),
            Command::Left(value) => self.move_head(*value, (-1, 0)),
            Command::Right(value) => self.move_head(*value, (1, 0)),
        }
    }

    fn move_head(&mut self, steps: u32, direction: (i32, i32)) {
        let (dx, dy) = direction;
        let rope_length = 1;

        for _ in 0..steps {
            self.knots[0].x += dx;
            self.knots[0].y += dy;

            for i in 1..self.knots.len() {
                if (self.knots[i].x - self.knots[i - 1].x).abs() > rope_length {
                    self.knots[i].x += if self.knots[i].x < self.knots[i - 1].x {
                        1
                    } else {
                        -1
                    };

                    // Check diagonal yoinks
                    if self.knots[i].y != self.knots[i - 1].y {
                        self.knots[i].y += if self.knots[i].y < self.knots[i - 1].y {
                            1
                        } else {
                            -1
                        }
                    }
                }

                if (self.knots[i].y - self.knots[i - 1].y).abs() > rope_length {
                    self.knots[i].y += if self.knots[i].y < self.knots[i - 1].y {
                        1
                    } else {
                        -1
                    };

                    // Check diagonal yoinks
                    if self.knots[i].x != self.knots[i - 1].x {
                        self.knots[i].x += if self.knots[i].x < self.knots[i - 1].x {
                            1
                        } else {
                            -1
                        }
                    }
                }
            }

            self.tail_visited.insert(*self.knots.last().unwrap());
        }
    }

    fn draw_tail_visits(&self) {
        let max_x = self.tail_visited.iter().map(|v| v.x).max().unwrap();
        let min_x = self.tail_visited.iter().map(|v| v.x).min().unwrap();
        let max_y = self.tail_visited.iter().map(|v| v.y).max().unwrap();
        let min_y = self.tail_visited.iter().map(|v| v.y).min().unwrap();

        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                if self.tail_visited.contains(&Vector { x, y }) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn draw_positions(&self) {
        let max_x = self.knots.iter().map(|v| v.x).max().unwrap();
        let min_x = self.knots.iter().map(|v| v.x).min().unwrap();
        let max_y = self.knots.iter().map(|v| v.y).max().unwrap();
        let min_y = self.knots.iter().map(|v| v.y).min().unwrap();

        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                if let Some(pos) = self.knots.iter().position(|v| v == &Vector { x, y }) {
                    print!("{}", pos);
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let commands = lines
        .map(|l| {
            let line = l.unwrap();
            let (command, value) = line.split_once(' ').unwrap();
            let value = value.parse::<u32>().unwrap();
            match command {
                "U" => Command::Up(value),
                "D" => Command::Down(value),
                "L" => Command::Left(value),
                "R" => Command::Right(value),
                _ => panic!("Unknown command: {}", command),
            }
        })
        .collect::<Vec<_>>();

    let mut rope = Rope::new(2);
    let mut long_rope = Rope::new(10);
    for command in commands {
        rope.process_command(&command);
        long_rope.process_command(&command);

        // println!("");
        // long_rope.draw_positions();
    }

    let tail_location_count = rope.tail_visited.len();
    let long_tail_location_count = long_rope.tail_visited.len();

    // long_rope.draw_tail_visits();

    println!(
        "The tail of the rope has visited {} locations.",
        tail_location_count,
    );

    println!(
        "The tail of the long rope has visited {} locations.",
        long_tail_location_count
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_head_up() {
        let mut rope = Rope::new(2);
        rope.move_head(1, (0, 1));
        assert_eq!(rope.head(), Vector { x: 0, y: 1 });
        assert_eq!(rope.tail(), Vector { x: 0, y: 0 });

        rope.move_head(1, (0, 1));
        assert_eq!(rope.head(), Vector { x: 0, y: 2 });
        assert_eq!(rope.tail(), Vector { x: 0, y: 1 });
    }

    #[test]
    fn test_move_head_down() {
        let mut rope = Rope::new(2);
        rope.move_head(1, (0, -1));
        assert_eq!(rope.head(), Vector { x: 0, y: -1 });
        assert_eq!(rope.tail(), Vector { x: 0, y: 0 });

        rope.move_head(1, (0, -1));
        assert_eq!(rope.head(), Vector { x: 0, y: -2 });
        assert_eq!(rope.tail(), Vector { x: 0, y: -1 });
    }

    #[test]
    fn test_move_head_left() {
        let mut rope = Rope::new(2);
        rope.move_head(1, (-1, 0));
        assert_eq!(rope.head(), Vector { x: -1, y: 0 });
        assert_eq!(rope.tail(), Vector { x: 0, y: 0 });

        rope.move_head(1, (-1, 0));
        assert_eq!(rope.head(), Vector { x: -2, y: 0 });
        assert_eq!(rope.tail(), Vector { x: -1, y: 0 });
    }

    #[test]
    fn test_move_head_right() {
        let mut rope = Rope::new(2);
        rope.move_head(1, (1, 0));
        assert_eq!(rope.head(), Vector { x: 1, y: 0 });
        assert_eq!(rope.tail(), Vector { x: 0, y: 0 });

        rope.move_head(1, (1, 0));
        assert_eq!(rope.head(), Vector { x: 2, y: 0 });
        assert_eq!(rope.tail(), Vector { x: 1, y: 0 });
    }

    #[test]
    fn test_move_head_up_then_left() {
        let mut rope = Rope::new(2);
        rope.move_head(1, (0, 1));
        assert_eq!(rope.head(), Vector { x: 0, y: 1 });
        assert_eq!(rope.tail(), Vector { x: 0, y: 0 });

        rope.move_head(1, (-1, 0));
        assert_eq!(rope.head(), Vector { x: -1, y: 1 });
        assert_eq!(rope.tail(), Vector { x: 0, y: 0 });

        rope.move_head(1, (-1, 0));
        assert_eq!(rope.head(), Vector { x: -2, y: 1 });
        assert_eq!(rope.tail(), Vector { x: -1, y: 1 });
    }

    #[test]
    fn test_move_head_up_then_right() {
        let mut rope = Rope::new(2);
        rope.move_head(1, (0, 1));
        assert_eq!(rope.head(), Vector { x: 0, y: 1 });
        assert_eq!(rope.tail(), Vector { x: 0, y: 0 });

        rope.move_head(1, (1, 0));
        assert_eq!(rope.head(), Vector { x: 1, y: 1 });
        assert_eq!(rope.tail(), Vector { x: 0, y: 0 });

        rope.move_head(1, (1, 0));
        assert_eq!(rope.head(), Vector { x: 2, y: 1 });
        assert_eq!(rope.tail(), Vector { x: 1, y: 1 });
    }

    #[test]
    fn test_move_head_down_then_left() {
        let mut rope = Rope::new(2);
        rope.move_head(1, (0, -1));
        assert_eq!(rope.head(), Vector { x: 0, y: -1 });
        assert_eq!(rope.tail(), Vector { x: 0, y: 0 });

        rope.move_head(1, (-1, 0));
        assert_eq!(rope.head(), Vector { x: -1, y: -1 });
        assert_eq!(rope.tail(), Vector { x: 0, y: 0 });

        rope.move_head(1, (-1, 0));
        assert_eq!(rope.head(), Vector { x: -2, y: -1 });
        assert_eq!(rope.tail(), Vector { x: -1, y: -1 });
    }

    #[test]
    fn test_move_head_down_then_right() {
        let mut rope = Rope::new(2);
        rope.move_head(1, (0, -1));
        assert_eq!(rope.head(), Vector { x: 0, y: -1 });
        assert_eq!(rope.tail(), Vector { x: 0, y: 0 });

        rope.move_head(1, (1, 0));
        assert_eq!(rope.head(), Vector { x: 1, y: -1 });
        assert_eq!(rope.tail(), Vector { x: 0, y: 0 });

        rope.move_head(1, (1, 0));
        assert_eq!(rope.head(), Vector { x: 2, y: -1 });
        assert_eq!(rope.tail(), Vector { x: 1, y: -1 });
    }

    #[test]
    fn test_move_right_then_up_multiple() {
        let mut rope = Rope::new(2);
        rope.move_head(4, (1, 0));
        assert_eq!(rope.head(), Vector { x: 4, y: 0 });
        assert_eq!(rope.tail(), Vector { x: 3, y: 0 });

        rope.move_head(1, (0, 1));
        assert_eq!(rope.head(), Vector { x: 4, y: 1 });
        assert_eq!(rope.tail(), Vector { x: 3, y: 0 });

        rope.move_head(1, (0, 1));
        assert_eq!(rope.head(), Vector { x: 4, y: 2 });
        assert_eq!(rope.tail(), Vector { x: 4, y: 1 });

        rope.move_head(1, (0, 1));
        assert_eq!(rope.head(), Vector { x: 4, y: 3 });
        assert_eq!(rope.tail(), Vector { x: 4, y: 2 });

        rope.move_head(1, (0, 1));
        assert_eq!(rope.head(), Vector { x: 4, y: 4 });
        assert_eq!(rope.tail(), Vector { x: 4, y: 3 });
    }

    #[test]
    fn test_long_rope() {
        let mut rope = Rope::new(10);
        rope.move_head(5, (1, 0));
        assert_eq!(rope.head(), Vector { x: 5, y: 0 });
        assert_eq!(rope.tail(), Vector { x: 0, y: 0 });
    }
}
