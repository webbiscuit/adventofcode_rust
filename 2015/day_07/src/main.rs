use std::{
    collections::{HashMap, HashSet, VecDeque},
    io,
};

type Dest = Wire;
type Source = Wire;
type Wire = String;

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

impl Instruction {
    fn dest(&self) -> &Dest {
        match self {
            Instruction::Assign(dest, _)
            | Instruction::And(dest, _, _)
            | Instruction::Or(dest, _, _)
            | Instruction::LShift(dest, _, _)
            | Instruction::RShift(dest, _, _)
            | Instruction::Not(dest, _) => dest,
        }
    }

    fn operands(&self) -> Vec<&Operand> {
        match self {
            Instruction::Assign(_, operand1)
            | Instruction::LShift(_, operand1, _)
            | Instruction::RShift(_, operand1, _)
            | Instruction::Not(_, operand1) => vec![operand1],
            Instruction::And(_, operand1, operand2) | Instruction::Or(_, operand1, operand2) => {
                vec![operand1, operand2]
            }
        }
    }
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

fn evaluate_instructions(instructions: &[&Instruction]) -> Vec<(Wire, u16)> {
    let mut wire_values: Vec<(String, u16)> = Vec::new();

    fn lookup_wire_value(wire: &str, wire_values: &[(Wire, u16)]) -> u16 {
        let value = wire_values.iter().find(|(w, _)| w == wire).unwrap().1;
        value
    }

    fn handle_operand(operand: &Operand, wire_values: &[(Wire, u16)]) -> u16 {
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

fn build_dependency_graph(instructions: &[Instruction]) -> HashMap<Wire, HashSet<Wire>> {
    let mut graph: HashMap<Wire, HashSet<Wire>> = HashMap::new();

    for instruction in instructions {
        for operand in instruction.operands() {
            add_dependency(&mut graph, operand, instruction.dest());
        }
    }

    graph
}

fn add_dependency(graph: &mut HashMap<Wire, HashSet<Wire>>, operand: &Operand, dest: &str) {
    if let Operand::Wire(source) = operand {
        graph
            .entry(source.to_string())
            .or_default()
            .insert(dest.to_string());
    }
}

fn topological_sort(dependency_graph: &HashMap<Wire, HashSet<Wire>>) -> Result<Vec<Wire>, String> {
    let mut in_degree = HashMap::new();
    let mut zero_in_degree_queue = VecDeque::new();
    let mut sorted_elements = Vec::new();

    // Initialize in-degree of each node
    for node in dependency_graph.keys() {
        in_degree.insert(node.clone(), 0);
    }

    // Calculate in-degree
    for deps in dependency_graph.values() {
        for dep in deps {
            *in_degree.entry(dep.clone()).or_insert(0) += 1;
        }
    }

    // Find nodes with no incoming edges
    for (node, &degree) in in_degree.iter() {
        if degree == 0 {
            zero_in_degree_queue.push_back(node.clone());
        }
    }

    // Process nodes with zero in-degree and update dependent nodes
    while let Some(node) = zero_in_degree_queue.pop_front() {
        sorted_elements.push(node.clone());
        if let Some(deps) = dependency_graph.get(&node) {
            for dep in deps {
                let degree = in_degree.entry(dep.clone()).or_default();
                *degree -= 1;
                if *degree == 0 {
                    zero_in_degree_queue.push_back(dep.clone());
                }
            }
        }
    }

    // Check for cycle
    if sorted_elements.len() != in_degree.len() {
        return Err("Cycle detected in the graph".to_string());
    }

    Ok(sorted_elements)
}

fn main() -> std::io::Result<()> {
    let input = io::stdin().lines();

    let instructions: Vec<Instruction> = input.map(|line| parse_line(&line.unwrap())).collect();

    let graph = build_dependency_graph(&instructions);
    let sorted_wires = topological_sort(&graph).unwrap();
    let mut sorted_instructions: Vec<&Instruction> = Vec::new();

    for wire in sorted_wires {
        let instruction = instructions
            .iter()
            .find(|&instruction| instruction.dest() == &wire)
            .unwrap();
        sorted_instructions.push(instruction);
    }

    let wire_values = evaluate_instructions(&sorted_instructions);

    // for (wire, value) in wire_values {
    //     println!("{}: {}", wire, value);
    // }

    let wire_a = wire_values.iter().find(|(wire, _)| wire == "a").unwrap().1;
    println!("Signal on a is {}", wire_a);

    Ok(())
}
