use std::io::{self, prelude::*};

#[derive(Debug)]
struct CrateStack {
    crates: Vec<char>,
}

impl CrateStack {
    fn new() -> CrateStack {
        CrateStack { crates: Vec::new() }
    }
}

struct CraneArea {
    crate_stacks: Vec<CrateStack>,
}

impl CraneArea {
    fn new() -> CraneArea {
        CraneArea {
            crate_stacks: Vec::new(),
        }
    }

    fn add_crate_to_stack(&mut self, stack_index: usize, crate_contents: char) {
        while stack_index > self.crate_stacks.len() {
            self.crate_stacks.push(CrateStack::new());
        }

        self.crate_stacks[stack_index - 1]
            .crates
            .push(crate_contents);
    }

    fn take_crate_from_stack(&mut self, stack_index: usize) -> char {
        let stack = &mut self.crate_stacks[stack_index - 1];
        stack.crates.pop().unwrap()
    }

    fn rearrange(&mut self, count: usize, from_stack_index: usize, to_stack_index: usize) {
        for i in 0..count {
            let crate_contents = self.take_crate_from_stack(from_stack_index);
            self.add_crate_to_stack(to_stack_index, crate_contents);
        }
    }

    fn top_crates(&self) -> String {
        self.crate_stacks
            .iter()
            .map(|stack| stack.crates.last().unwrap())
            .collect()
    }

    fn draw(&self) {
        let max_height = self
            .crate_stacks
            .iter()
            .map(|stack| stack.crates.len())
            .max()
            .unwrap_or(0);

        for i in (0..max_height).rev() {
            for stack in &self.crate_stacks {
                if i < stack.crates.len() {
                    print!("[{}]  ", stack.crates[i]);
                } else {
                    print!("     ");
                }
            }
            println!();
        }

        for i in 0..self.crate_stacks.len() {
            print!(" {}   ", i + 1);
        }
        println!();
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut crane_area = CraneArea::new();
    crane_area.add_crate_to_stack(1, 'Z');
    crane_area.add_crate_to_stack(1, 'N');
    crane_area.add_crate_to_stack(2, 'M');
    crane_area.add_crate_to_stack(2, 'C');
    crane_area.add_crate_to_stack(2, 'D');
    crane_area.add_crate_to_stack(3, 'P');

    crane_area.rearrange(1, 2, 1);
    crane_area.rearrange(3, 1, 3);
    crane_area.rearrange(2, 2, 1);
    crane_area.rearrange(1, 1, 2);

    crane_area.draw();

    println!("The top crates spell out {}.", crane_area.top_crates());
}

#[test]
fn test_example() {
    let mut crane_area = CraneArea::new();
    crane_area.add_crate_to_stack(1, 'Z');
    crane_area.add_crate_to_stack(1, 'N');
    crane_area.add_crate_to_stack(2, 'M');
    crane_area.add_crate_to_stack(2, 'C');
    crane_area.add_crate_to_stack(2, 'D');
    crane_area.add_crate_to_stack(3, 'P');

    crane_area.rearrange(1, 2, 1);
    crane_area.rearrange(3, 1, 3);
    crane_area.rearrange(2, 2, 1);
    crane_area.rearrange(1, 1, 2);

    crane_area.draw();

    assert!(crane_area.top_crates() == "CMZ")
}
