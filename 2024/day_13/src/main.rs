use regex::Regex;
use std::io::{self, prelude::*};

type Position = (isize, isize);
type MoveVector = (isize, isize);

#[derive(Debug)]
struct Machine {
    prize_position: Position,
    button_a: MoveVector,
    button_b: MoveVector,
}

impl Machine {
    fn new(button_a: MoveVector, button_b: MoveVector, prize_position: Position) -> Machine {
        Machine {
            prize_position,
            button_a,
            button_b,
        }
    }

    fn bump_prize_position(&mut self) {
        let bump = 10000000000000;
        self.prize_position.0 += bump;
        self.prize_position.1 += bump;
    }
}

fn parse(lines: &[String]) -> Vec<Machine> {
    let machine_split: Vec<_> = lines.split(|l| l.is_empty()).collect();

    machine_split
        .iter()
        .map(|m| {
            let re = Regex::new(r"(\d+)").expect("Invalid regex");

            let button_a_text = &m[0];
            let button_b_text = &m[1];
            let prize_text = &m[2];

            let a: (isize, isize) = re
                .captures_iter(button_a_text)
                .map(|mul| mul[1].parse::<isize>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .map(|arr: [isize; 2]| (arr[0], arr[1]))
                .expect("Only expecting 2 numbers");

            let b: (isize, isize) = re
                .captures_iter(button_b_text)
                .map(|mul| mul[1].parse::<isize>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .map(|arr: [isize; 2]| (arr[0], arr[1]))
                .expect("Only expecting 2 numbers");

            let prize: (isize, isize) = re
                .captures_iter(prize_text)
                .map(|mul| mul[1].parse::<isize>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .map(|arr: [isize; 2]| (arr[0], arr[1]))
                .expect("Only expecting 2 numbers");

            Machine::new(a, b, prize)
        })
        .collect()
}

type Var = isize;
type Target = isize;
type Equation = (Var, Var, Target);
type Answer = (Var, Var);

fn solve(eq1: Equation, eq2: Equation) -> Option<Answer> {
    // Multiply both sides by the other's A
    let other_a = eq2.0;
    let new_eq1 = (eq1.0 * other_a, eq1.1 * other_a, eq1.2 * other_a);

    let other_a = eq1.0;
    let new_eq2 = (eq2.0 * other_a, eq2.1 * other_a, eq2.2 * other_a);

    // Sub one from the other to eliminate A
    let eliminated_a_eq = (
        new_eq2.0 - new_eq1.0,
        new_eq2.1 - new_eq1.1,
        new_eq2.2 - new_eq1.2,
    );

    assert!(eliminated_a_eq.0 == 0);

    // println!("Eq {:?}", eliminated_a_eq);

    // Now can solve for B and A
    let b = eliminated_a_eq.2 / eliminated_a_eq.1;
    let a = (eq1.2 - (eq1.1 * b)) / eq1.0;

    // Check the other one
    if eq2.0 * a + eq2.1 * b == eq2.2 {
        return Some((a, b));
    }

    None
}

fn count_tokens_for_machine(machine: &Machine) -> Option<usize> {
    let eq1 = (
        machine.button_a.0,
        machine.button_b.0,
        machine.prize_position.0,
    );
    let eq2 = (
        machine.button_a.1,
        machine.button_b.1,
        machine.prize_position.1,
    );

    // println!("EQ1 {:?}", eq1);
    // println!("EQ2 {:?}", eq2);

    const A_TOKEN_COST: isize = 3;
    const B_TOKEN_COST: isize = 1;

    let solved = solve(eq1, eq2);

    // println!("solved {:?}", solved);

    solved.map(|solved| (solved.0 * A_TOKEN_COST + solved.1 * B_TOKEN_COST) as usize)
}

fn count_tokens_for_all_machine(machines: &[Machine]) -> usize {
    machines
        .iter()
        .filter_map(count_tokens_for_machine)
        // .inspect(|t| println!("Token cost {}", t))
        .sum()
}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();
    let mut machines = parse(&lines);

    // println!("{:?}", machines);

    let answer = count_tokens_for_all_machine(&machines);

    println!("You need to spend {} tokens to win all the prizes", answer);

    // Move to the new prize position
    machines.iter_mut().for_each(|m| m.bump_prize_position());

    let answer = count_tokens_for_all_machine(&machines);

    println!(
        "You need to spend {} tokens to win all the new prizes",
        answer
    );

    Ok(())
}
