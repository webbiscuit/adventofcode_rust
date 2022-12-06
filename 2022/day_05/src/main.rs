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

    fn add_crates_to_stack(&mut self, stack_index: usize, mut crate_contents: Vec<char>) {
        while stack_index > self.crate_stacks.len() {
            self.crate_stacks.push(CrateStack::new());
        }

        self.crate_stacks[stack_index - 1]
            .crates
            .append(&mut crate_contents);
    }

    fn take_crate_from_stack(&mut self, stack_index: usize) -> char {
        let stack = &mut self.crate_stacks[stack_index - 1];
        stack.crates.pop().unwrap()
    }

    fn take_crates_from_stack(&mut self, stack_index: usize, count: usize) -> Vec<char> {
        let stack = &mut self.crate_stacks[stack_index - 1];
        stack.crates.split_off(stack.crates.len() - count)
    }

    // CrateMover 9000 - can only move one crate at a time
    fn rearrange_using_single_mover(
        &mut self,
        count: usize,
        from_stack_index: usize,
        to_stack_index: usize,
    ) {
        for _i in 0..count {
            let crate_contents = self.take_crate_from_stack(from_stack_index);
            self.add_crate_to_stack(to_stack_index, crate_contents);
        }
    }

    // CrateMover 9001 - can only move multiples crates at a time
    fn rearrange_using_multi_mover(
        &mut self,
        count: usize,
        from_stack_index: usize,
        to_stack_index: usize,
    ) {
        let crate_contents = self.take_crates_from_stack(from_stack_index, count);
        self.add_crates_to_stack(to_stack_index, crate_contents);
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

struct Instruction {
    crate_count: usize,
    from_stack_index: usize,
    to_stack_index: usize,
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    let mut crane_area = CraneArea::new();
    let mut crane_area_9001 = CraneArea::new();

    let crate_count_ix = lines.iter().position(|l| l.starts_with(" 1 ")).unwrap();
    let crate_count: usize = lines[crate_count_ix]
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();

    // Count up from bottom of crate pic to make this easier
    for line_index in (0..crate_count_ix).rev() {
        let line = &lines[line_index];
        for i in 1..=crate_count {
            let index = (i - 1) * 4;
            let crate_contents = line[index + 1..index + 3].chars().next().unwrap();
            if crate_contents != ' ' {
                crane_area.add_crate_to_stack(i, crate_contents);
                crane_area_9001.add_crate_to_stack(i, crate_contents);
            }
        }
    }

    let mut instructions = Vec::new();

    for line in &lines[crate_count_ix + 1..] {
        if line.starts_with("move") {
            // println!("{:?}", line);

            let mut parts = line.split_whitespace();
            // println!("{:?}", parts);
            parts.next();
            let crate_count: usize = parts.next().unwrap().parse().unwrap();
            parts.next();
            let from_stack_index: usize = parts.next().unwrap().parse().unwrap();
            parts.next();
            let to_stack_index: usize = parts.next().unwrap().parse().unwrap();
            instructions.push(Instruction {
                crate_count,
                from_stack_index,
                to_stack_index,
            });
        }
    }

    // crane_area.draw();

    for instruction in instructions {
        crane_area.rearrange_using_single_mover(
            instruction.crate_count,
            instruction.from_stack_index,
            instruction.to_stack_index,
        );

        crane_area_9001.rearrange_using_multi_mover(
            instruction.crate_count,
            instruction.from_stack_index,
            instruction.to_stack_index,
        );
    }

    // crane_area.draw();
    // crane_area_9001.draw();

    println!("The top crates using CrateMover 9000 spell out {}.", crane_area.top_crates());
    println!("The top crates using CrateMover 9001 spell out {}.", crane_area_9001.top_crates());
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

    crane_area.rearrange_using_single_mover(1, 2, 1);
    crane_area.rearrange_using_single_mover(3, 1, 3);
    crane_area.rearrange_using_single_mover(2, 2, 1);
    crane_area.rearrange_using_single_mover(1, 1, 2);

    crane_area.draw();

    assert!(crane_area.top_crates() == "CMZ")
}
