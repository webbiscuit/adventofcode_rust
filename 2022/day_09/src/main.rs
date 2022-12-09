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
    head: Vector,
    tail: Vector,

    tail_visited: HashSet<Vector>,
}

impl Rope {
    fn new() -> Self {
        Self {
            head: Vector { x: 0, y: 0 },
            tail: Vector { x: 0, y: 0 },
            tail_visited: HashSet::new(),
        }
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
            self.head.x += dx;
            self.head.y += dy;

            if (self.tail.x - self.head.x).abs() > rope_length {
                self.tail.x += dx;

                // Check diagonal yoinks
                if self.tail.y != self.head.y {
                    self.tail.y += self.head.y - self.tail.y;
                }
            }

            if (self.tail.y - self.head.y).abs() > rope_length {
                self.tail.y += dy;

                // Check diagonal yoinks
                if self.tail.x != self.head.x {
                    self.tail.x += self.head.x - self.tail.x;
                }
            }

            self.tail_visited.insert(self.tail);
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

    let mut rope = Rope::new();
    for command in commands {
        rope.process_command(&command);
    }

    let tail_location_count = rope.tail_visited.len();

    // println!("{:?}", rope.tail_visited);
    // rope.draw_tail_visits();

    println!(
        "The tail of the rope has visited {} locations.",
        tail_location_count
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_head_up() {
        let mut rope = Rope::new();
        rope.move_head(1, (0, 1));
        assert_eq!(rope.head, Vector { x: 0, y: 1 });
        assert_eq!(rope.tail, Vector { x: 0, y: 0 });

        rope.move_head(1, (0, 1));
        assert_eq!(rope.head, Vector { x: 0, y: 2 });
        assert_eq!(rope.tail, Vector { x: 0, y: 1 });
    }

    #[test]
    fn test_move_head_down() {
        let mut rope = Rope::new();
        rope.move_head(1, (0, -1));
        assert_eq!(rope.head, Vector { x: 0, y: -1 });
        assert_eq!(rope.tail, Vector { x: 0, y: 0 });

        rope.move_head(1, (0, -1));
        assert_eq!(rope.head, Vector { x: 0, y: -2 });
        assert_eq!(rope.tail, Vector { x: 0, y: -1 });
    }

    #[test]
    fn test_move_head_left() {
        let mut rope = Rope::new();
        rope.move_head(1, (-1, 0));
        assert_eq!(rope.head, Vector { x: -1, y: 0 });
        assert_eq!(rope.tail, Vector { x: 0, y: 0 });

        rope.move_head(1, (-1, 0));
        assert_eq!(rope.head, Vector { x: -2, y: 0 });
        assert_eq!(rope.tail, Vector { x: -1, y: 0 });
    }

    #[test]
    fn test_move_head_right() {
        let mut rope = Rope::new();
        rope.move_head(1, (1, 0));
        assert_eq!(rope.head, Vector { x: 1, y: 0 });
        assert_eq!(rope.tail, Vector { x: 0, y: 0 });

        rope.move_head(1, (1, 0));
        assert_eq!(rope.head, Vector { x: 2, y: 0 });
        assert_eq!(rope.tail, Vector { x: 1, y: 0 });
    }

    #[test]
    fn test_move_head_up_then_left() {
        let mut rope = Rope::new();
        rope.move_head(1, (0, 1));
        assert_eq!(rope.head, Vector { x: 0, y: 1 });
        assert_eq!(rope.tail, Vector { x: 0, y: 0 });

        rope.move_head(1, (-1, 0));
        assert_eq!(rope.head, Vector { x: -1, y: 1 });
        assert_eq!(rope.tail, Vector { x: 0, y: 0 });

        rope.move_head(1, (-1, 0));
        assert_eq!(rope.head, Vector { x: -2, y: 1 });
        assert_eq!(rope.tail, Vector { x: -1, y: 1 });
    }

    #[test]
    fn test_move_head_up_then_right() {
        let mut rope = Rope::new();
        rope.move_head(1, (0, 1));
        assert_eq!(rope.head, Vector { x: 0, y: 1 });
        assert_eq!(rope.tail, Vector { x: 0, y: 0 });

        rope.move_head(1, (1, 0));
        assert_eq!(rope.head, Vector { x: 1, y: 1 });
        assert_eq!(rope.tail, Vector { x: 0, y: 0 });

        rope.move_head(1, (1, 0));
        assert_eq!(rope.head, Vector { x: 2, y: 1 });
        assert_eq!(rope.tail, Vector { x: 1, y: 1 });
    }

    #[test]
    fn test_move_head_down_then_left() {
        let mut rope = Rope::new();
        rope.move_head(1, (0, -1));
        assert_eq!(rope.head, Vector { x: 0, y: -1 });
        assert_eq!(rope.tail, Vector { x: 0, y: 0 });

        rope.move_head(1, (-1, 0));
        assert_eq!(rope.head, Vector { x: -1, y: -1 });
        assert_eq!(rope.tail, Vector { x: 0, y: 0 });

        rope.move_head(1, (-1, 0));
        assert_eq!(rope.head, Vector { x: -2, y: -1 });
        assert_eq!(rope.tail, Vector { x: -1, y: -1 });
    }

    #[test]
    fn test_move_head_down_then_right() {
        let mut rope = Rope::new();
        rope.move_head(1, (0, -1));
        assert_eq!(rope.head, Vector { x: 0, y: -1 });
        assert_eq!(rope.tail, Vector { x: 0, y: 0 });

        rope.move_head(1, (1, 0));
        assert_eq!(rope.head, Vector { x: 1, y: -1 });
        assert_eq!(rope.tail, Vector { x: 0, y: 0 });

        rope.move_head(1, (1, 0));
        assert_eq!(rope.head, Vector { x: 2, y: -1 });
        assert_eq!(rope.tail, Vector { x: 1, y: -1 });
    }

    #[test]
    fn test_move_right_then_up_multiple() {
        let mut rope = Rope::new();
        rope.move_head(4, (1, 0));
        assert_eq!(rope.head, Vector { x: 4, y: 0 });
        assert_eq!(rope.tail, Vector { x: 3, y: 0 });

        rope.move_head(1, (0, 1));
        assert_eq!(rope.head, Vector { x: 4, y: 1 });
        assert_eq!(rope.tail, Vector { x: 3, y: 0 });

        rope.move_head(1, (0, 1));
        assert_eq!(rope.head, Vector { x: 4, y: 2 });
        assert_eq!(rope.tail, Vector { x: 4, y: 1 });

        rope.move_head(1, (0, 1));
        assert_eq!(rope.head, Vector { x: 4, y: 3 });
        assert_eq!(rope.tail, Vector { x: 4, y: 2 });

        rope.move_head(1, (0, 1));
        assert_eq!(rope.head, Vector { x: 4, y: 4 });
        assert_eq!(rope.tail, Vector { x: 4, y: 3 });
    }
}
