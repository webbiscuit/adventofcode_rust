use std::io::{self, prelude::*};

use regex::Regex;

#[derive(Debug)]
struct MonkeyBusiness {
    monkeys: Vec<Monkey>,
}

impl MonkeyBusiness {
    fn new() -> MonkeyBusiness {
        MonkeyBusiness {
            monkeys: Vec::new(),
        }
    }

    fn add_monkey(&mut self, monkey: Monkey) {
        self.monkeys.push(monkey);
    }

    fn chuck_items(&mut self, with_stress_relief: bool) {
        let monkey_mod: usize = self
            .monkeys
            .iter()
            .fold(1, |acc, m| acc * (m.test_divisible_condition as usize));

        for i in 0..self.monkeys.len() {
            let monkey = self.monkeys[i].clone();

            for item in monkey.items {
                let monkey_inspected_item = match monkey.operation {
                    Operation::Add(n) => item + n as usize,
                    Operation::Multiply(n) => item * n as usize,
                    Operation::MultiplySelf => item * item,
                };

                let monkey_boredom = if with_stress_relief {
                    monkey_inspected_item / 3
                } else {
                    monkey_inspected_item % monkey_mod
                };

                if monkey_boredom % monkey.test_divisible_condition as usize == 0 {
                    self.monkeys[monkey.true_test as usize]
                        .items
                        .push(monkey_boredom);
                } else {
                    self.monkeys[monkey.false_test as usize]
                        .items
                        .push(monkey_boredom);
                }
            }
            self.monkeys[i].items_inspected += self.monkeys[i].items.len();
            // All these items have been chucked
            self.monkeys[i].items.clear();
        }
    }

    fn calculate_level_of_monkey_business(&self) -> usize {
        let mut handled_items: Vec<usize> = self
            .monkeys
            .iter()
            .map(|m| m.items_inspected)
            .collect::<Vec<_>>();
        handled_items.sort_unstable_by(|a, b| b.cmp(a));

        // println!("{:?}", handled_items);

        handled_items[0] * handled_items[1]
    }
}

type Item = usize;
type MonkeyId = u8;

#[derive(Debug, Clone)]
struct Monkey {
    id: MonkeyId,
    items: Vec<Item>,
    operation: Operation,
    test_divisible_condition: u8,
    true_test: MonkeyId,
    false_test: MonkeyId,
    items_inspected: usize,
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(u8),
    Multiply(u8),
    MultiplySelf,
}

fn parse_monkey(lines: &[String]) -> Monkey {
    let monkey_re = Regex::new(r"Monkey (\d):").unwrap();
    let starting_items_re = Regex::new(r"Starting items: (\d+(, \d+)*)").unwrap();
    let operation_re = Regex::new(r"Operation: new = old ([\*\+]) (\d+|old)").unwrap();
    let test_re = Regex::new(r"Test: divisible by (\d+)").unwrap();
    let true_re = Regex::new(r"If true: throw to monkey (\d+)").unwrap();
    let false_re = Regex::new(r"If false: throw to monkey (\d+)").unwrap();

    let monkey_id = monkey_re.captures(&lines[0]).unwrap()[1]
        .parse::<MonkeyId>()
        .unwrap();

    let starting_items = starting_items_re.captures(&lines[1]).unwrap()[1]
        .split(", ")
        .map(|s| s.parse::<Item>().unwrap())
        .collect::<Vec<_>>();

    let operation_captures = operation_re.captures(&lines[2]).unwrap();

    let operation: Operation = match operation_captures[1].trim() {
        "+" => Operation::Add(operation_captures[2].parse::<u8>().unwrap()),
        "*" => {
            if operation_captures[2].trim() == "old" {
                Operation::MultiplySelf
            } else {
                Operation::Multiply(operation_captures[2].parse::<u8>().unwrap())
            }
        }
        _ => panic!("Unknown operation"),
    };

    let test_divisible_condition = test_re.captures(&lines[3]).unwrap()[1]
        .parse::<u8>()
        .unwrap();

    let true_test = true_re.captures(&lines[4]).unwrap()[1]
        .parse::<MonkeyId>()
        .unwrap();

    let false_test = false_re.captures(&lines[5]).unwrap()[1]
        .parse::<MonkeyId>()
        .unwrap();

    let monkey = Monkey {
        id: monkey_id,
        items: starting_items,
        operation,
        test_divisible_condition,
        true_test,
        false_test,
        items_inspected: 0,
    };

    monkey
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    let monkeys = lines.split(|l| l.is_empty()).map(|l| {
        let monkey = parse_monkey(l);
        // println!("{:?}", monkey);
        monkey
    });

    let mut monkey_business = MonkeyBusiness::new();
    let mut monkey_business2 = MonkeyBusiness::new();

    for monkey in monkeys {
        monkey_business.add_monkey(monkey.clone());
        monkey_business2.add_monkey(monkey);
    }

    for _ in 0..20 {
        monkey_business.chuck_items(true);
    }

    let monkey_business_score = monkey_business.calculate_level_of_monkey_business();

    println!("The level of monkey business is {}.", monkey_business_score);

    for _ in 0..10000 {
        monkey_business2.chuck_items(false);
    }

    let monkey_business_score2 = monkey_business2.calculate_level_of_monkey_business();

    println!(
        "The level of monkey business after 10000 rounds is {}.",
        monkey_business_score2
    );
}
