use std::io;

type Dest = String;
type Source = String;

#[derive(Debug)]
enum Instruction {
    Assign(Dest, Operand),
    And(Dest, Operand, Operand),
    Or(Dest, Operand, Operand),
    LShift(Dest, Operand, u16),
    RShift(Dest, Operand, u16),
    Not(Dest, Operand),
}

#[derive(Debug)]
enum Operand {
    Value(u16),
    Wire(Source),
}

fn parse_line(line: &str) -> Instruction {
    let parts: Vec<&str> = line.split_whitespace().collect();
    match parts[..] {
        [src, "->", dest] => Instruction::Assign(dest.to_string(), parse_operand(src)),
        [src, "AND", other, "->", dest] => {
            Instruction::And(dest.to_string(), parse_operand(src), parse_operand(other))
        }
        [src, "OR", other, "->", dest] => {
            Instruction::Or(dest.to_string(), parse_operand(src), parse_operand(other))
        }
        [src, "LSHIFT", other, "->", dest] => {
            Instruction::LShift(dest.to_string(), parse_operand(src), other.parse().unwrap())
        }
        [src, "RSHIFT", other, "->", dest] => {
            Instruction::RShift(dest.to_string(), parse_operand(src), other.parse().unwrap())
        }
        ["NOT", src, "->", dest] => Instruction::Not(dest.to_string(), parse_operand(src)),

        _ => panic!("Unknown instruction format"),
    }
}

fn parse_operand(operand: &str) -> Operand {
    match operand.parse::<u16>() {
        Ok(value) => Operand::Value(value),
        Err(_) => Operand::Wire(operand.to_string()),
    }
}

fn evaluate_instructions(instructions: &[Instruction]) -> Vec<(String, u16)> {
    let mut wire_values: Vec<(String, u16)> = Vec::new();

    fn lookup_wire_value(wire: &str, wire_values: &[(String, u16)]) -> u16 {
        let value = wire_values.iter().find(|(w, _)| w == wire).unwrap().1;
        value
    }

    fn handle_operand(operand: &Operand, wire_values: &[(String, u16)]) -> u16 {
        match operand {
            Operand::Value(value) => *value,
            Operand::Wire(wire) => lookup_wire_value(wire, wire_values),
        }
    }

    for instruction in instructions {
        match instruction {
            Instruction::Assign(dest, operand) => {
                let value = handle_operand(operand, &wire_values);
                wire_values.push((dest.to_string(), value));
            }
            Instruction::And(dest, operand1, operand2) => {
                let value1 = handle_operand(operand1, &wire_values);
                let value2 = handle_operand(operand2, &wire_values);
                wire_values.push((dest.to_string(), value1 & value2));
            }
            Instruction::Or(dest, operand1, operand2) => {
                let value1 = handle_operand(operand1, &wire_values);
                let value2 = handle_operand(operand2, &wire_values);
                wire_values.push((dest.to_string(), value1 | value2));
            }
            Instruction::LShift(dest, operand, shift) => {
                let value = handle_operand(operand, &wire_values);
                wire_values.push((dest.to_string(), value << shift));
            }
            Instruction::RShift(dest, operand, shift) => {
                let value = handle_operand(operand, &wire_values);
                wire_values.push((dest.to_string(), value >> shift));
            }
            Instruction::Not(dest, operand) => {
                let value = handle_operand(operand, &wire_values);
                wire_values.push((dest.to_string(), !value));
            } // _ => panic!("Unknown instruction {instruction:?}"),
        }
    }

    wire_values
}

fn main() -> std::io::Result<()> {
    let input = io::stdin().lines();

    let instructions: Vec<Instruction> = input.map(|line| parse_line(&line.unwrap())).collect();

    let wire_values = evaluate_instructions(&instructions);

    // for (wire, value) in wire_values {
    //     println!("{}: {}", wire, value);
    // }

    let wire_a = wire_values.iter().find(|(wire, _)| wire == "a").unwrap().1;
    println!("Signal on a is {}", wire_a);

    Ok(())
}
