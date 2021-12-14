use std::collections::HashMap;
use std::error::Error;
use std::hash::Hash;
use std::io::{self, prelude::*};

use counter::Counter;

struct PolymerBuilder {
    polymer: String,
    rules: HashMap<String, String>,
}

impl PolymerBuilder {
    fn new(initial_polymer: String) -> Self {
        Self {
            polymer: initial_polymer,
            rules: HashMap::new(),
        }
    }

    fn add_rule(&mut self, from: &String, to: &String) {
        self.rules.insert(from.to_string(), to.to_string());
    }

    fn get_polymer(&self) -> &String {
        &self.polymer
    }

    fn step(&mut self) {
        let mut new_segments = Vec::new();

        self.polymer
            .chars()
            .zip(self.polymer.chars().skip(1))
            .for_each(|(a, b)| {
                let segment = format!("{}{}", a, b);
                // println!("{}", segment);
                new_segments.push(self.rules[&segment].to_string());
            });

        let mut new_polymer = String::new();
        new_segments.reverse();

        self.polymer.chars().for_each(|c| {
            new_polymer.push(c);
            new_polymer.extend(new_segments.pop());
        });

        self.polymer = new_polymer;
    }

    fn calculate_most_least_common(&self) -> u64 {
        let counts = self.polymer.chars().collect::<Counter<_>>().most_common_ordered();
        // println!("{:?}", counts);
        counts.first().unwrap().1 as u64 - counts.last().unwrap().1 as u64
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let initial_polymer = lines.next().unwrap()?;
    let mut polymer_builder = PolymerBuilder::new(initial_polymer);

    for line in lines.flatten() {
        if line.contains(" -> ") {
            let (from, to) = line.split_once(" -> ").unwrap();
            polymer_builder.add_rule(
                &from.to_string(), 
                &to.to_string()
                // &format!(
                //     "{}{}{}",
                //     from.chars().nth(0).unwrap(),
                //     &to,
                //     from.chars().nth(1).unwrap()
                //),
            );
        }
    }

    // println!("{}", polymer_builder.get_polymer());

    for i in 1..=10 {
        polymer_builder.step();
    }

    println!("After 10 steps, most common element minus least common element: {}", polymer_builder.calculate_most_least_common());

    for i in 1..=30 {
        polymer_builder.step();
    }

    println!("After 40 steps, most common element minus least common element: {}", polymer_builder.calculate_most_least_common());



    Ok(())
}
