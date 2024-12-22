use std::fmt;

use std::io::{self, prelude::*};

use std::fmt::Display;

type Byte = u8;

enum Operand {
    Literal(isize),
    RegisterA,
    RegisterB,
    RegisterC,
}

impl Operand {
    fn from(value: Byte) -> Option<Self> {
        match value {
            0 => Some(Operand::Literal(0)),
            1 => Some(Operand::Literal(1)),
            2 => Some(Operand::Literal(2)),
            3 => Some(Operand::Literal(3)),
            4 => Some(Operand::RegisterA),
            5 => Some(Operand::RegisterB),
            6 => Some(Operand::RegisterC),
            _ => None,
        }
    }

    fn dereference(&self, computer: &Computer) -> isize {
        match self {
            Operand::Literal(val) => *val,
            Operand::RegisterA => computer.register_a,
            Operand::RegisterB => computer.register_b,
            Operand::RegisterC => computer.register_b,
        }
    }

    fn to_literal(&self) -> isize {
        match self {
            Operand::Literal(val) => *val,
            Operand::RegisterA => 4,
            Operand::RegisterB => 5,
            Operand::RegisterC => 6,
        }
    }
}

enum Instruction {
    Adv(Operand),
    Bxl(Operand),
    Bst(Operand),
    Jnz(Operand),
    Bxc(Operand),
    Out(Operand),
    Bdv(Operand),
    Cdv(Operand),
}

impl Instruction {
    fn from(value: Byte, operand: Operand) -> Option<Self> {
        match value {
            0 => Some(Instruction::Adv(operand)),
            1 => Some(Instruction::Bxl(operand)),
            2 => Some(Instruction::Bst(operand)),
            3 => Some(Instruction::Jnz(operand)),
            4 => Some(Instruction::Bxc(operand)),
            5 => Some(Instruction::Out(operand)),
            6 => Some(Instruction::Bdv(operand)),
            7 => Some(Instruction::Cdv(operand)),
            _ => None,
        }
    }

    fn execute(&self, computer: &mut Computer) {
        match self {
            Instruction::Adv(op) => {
                let num = computer.register_a;
                let dom = 2_isize.pow(op.dereference(computer) as u32);

                let result = num / dom;
                computer.register_a = result;
            }

            Instruction::Bxl(op) => {
                computer.register_b ^= op.to_literal();
            }

            Instruction::Bst(op) => {
                let value = op.dereference(computer) % 8;

                computer.register_b = value;
            }

            Instruction::Jnz(op) => {
                if computer.register_a == 0 {
                    return;
                }

                // println!("JUMP");

                computer.program_counter = op.to_literal() as usize
            }

            Instruction::Bxc(_) => computer.register_b ^= computer.register_c,

            Instruction::Out(op) => {
                let value = op.dereference(computer) % 8;

                computer.output(value);
            }

            Instruction::Bdv(op) => {
                let num = computer.register_a;
                let dom = 2_isize.pow(op.dereference(computer) as u32);

                let result = num / dom;
                computer.register_b = result;
            }

            Instruction::Cdv(op) => {
                let num = computer.register_a;
                let dom = 2_isize.pow(op.dereference(computer) as u32);

                let result = num / dom;
                computer.register_c = result;
            }
        }
    }
}

struct Computer {
    register_a: isize,
    register_b: isize,
    register_c: isize,
    program: Vec<Byte>,
    program_counter: usize,
    output_buffer: Vec<isize>,
}

impl Computer {
    fn new(register_a: isize, register_b: isize, register_c: isize, program: Vec<Byte>) -> Self {
        Computer {
            register_a,
            register_b,
            register_c,
            program,
            program_counter: 0,
            output_buffer: vec![],
        }
    }

    fn run(&mut self) {
        self.program_counter = 0;

        while self.program_counter + 2 <= self.program.len() {
            // println!("PC {}", self.program_counter);
            self.step();
        }
    }

    fn step(&mut self) {
        println!("{}", self);

        let operand = Operand::from(self.program[self.program_counter + 1]).unwrap();
        let instruction = Instruction::from(self.program[self.program_counter], operand).unwrap();

        self.program_counter += 2;

        instruction.execute(self);
    }

    fn output(&mut self, val: isize) {
        self.output_buffer.push(val);
    }

    fn flush(&mut self) -> String {
        let output = self
            .output_buffer
            .iter()
            .map(|i| format!("{}", i))
            .collect::<Vec<_>>()
            .join(",");

        self.output_buffer.clear();

        output
    }
}

impl Display for Computer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Computer {{ register_a: {}, register_b: {}, register_c: {}, program: {:?} }}",
            self.register_a, self.register_b, self.register_c, self.program
        )
    }
}

fn parse(lines: &[String]) -> Computer {
    let register_a = lines[0]
        .split(' ')
        .nth(2)
        .unwrap()
        .parse::<isize>()
        .unwrap();
    let register_b = lines[1]
        .split(' ')
        .nth(2)
        .unwrap()
        .parse::<isize>()
        .unwrap();
    let register_c = lines[2]
        .split(' ')
        .nth(2)
        .unwrap()
        .parse::<isize>()
        .unwrap();
    let data = lines[4]
        .split(' ')
        .nth(1)
        .unwrap()
        .split(',')
        .map(|c| c.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    Computer::new(register_a, register_b, register_c, data)
}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    let mut computer = parse(&lines);

    // println!("{}", computer);

    computer.run();

    println!("{}", computer);

    let output = computer.flush();

    println!("The program will output {}", output);

    Ok(())
}

// !4,3,0,5,1,2,1,5,2
