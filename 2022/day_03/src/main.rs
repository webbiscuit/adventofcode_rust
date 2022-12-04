use std::{
    collections::HashSet,
    io::{self, prelude::*},
};

#[derive(Debug, Clone)]
struct Rucksack {
    compartment1: HashSet<char>,
    compartment2: HashSet<char>,
}

impl Rucksack {
    fn new() -> Rucksack {
        Rucksack {
            compartment1: HashSet::new(),
            compartment2: HashSet::new(),
        }
    }

    fn add_items(&mut self, items: &str) {
        self.compartment1 = items[0..items.len() / 2].chars().collect();
        self.compartment2 = items[items.len() / 2..].chars().collect();
    }

    fn common_items(&self) -> Vec<char> {
        self.compartment1
            .intersection(&self.compartment2)
            .cloned()
            .collect()
    }
}

#[derive(Debug)]
struct RucksackPile {
    contents: Vec<Rucksack>,
}

impl RucksackPile {
    fn new() -> RucksackPile {
        RucksackPile {
            contents: Vec::new(),
        }
    }

    fn add_rucksack(&mut self, rucksack: Rucksack) {
        self.contents.push(rucksack);
    }

    fn common_items(&self) -> Vec<char> {
        let item_piles = self
            .contents
            .iter()
            .map(|r| {
                r.compartment1
                    .union(&r.compartment2)
                    .cloned()
                    .collect::<HashSet<_>>()
            })
            .collect::<Vec<_>>();

        let common_items = item_piles.iter().fold(item_piles[0].clone(), |acc, x| {
            acc.intersection(x).cloned().collect()
        });

        Vec::from_iter(common_items)
    }
}

fn calculate_score(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 96,
        'A'..='Z' => c as u32 - 64 + 26,
        _ => 0,
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines(); //.map(|l| l.unwrap()).collect();

    let mut rucksacks = Vec::new();

    for line in lines {
        let line = line.unwrap();

        let mut rucksack = Rucksack::new();
        rucksack.add_items(&line);
        rucksacks.push(rucksack);
    }

    let priority_score: u32 = rucksacks
        .iter()
        .map(|r| r.common_items())
        .map(|items| items.iter().map(|i| calculate_score(*i)).sum::<u32>())
        .sum();

    println!(
        "Sum of priorities of shared item types is {}.",
        priority_score
    );

    let mut rucksack_piles: Vec<RucksackPile> = Vec::new();
    for piles in rucksacks.chunks(3) {
        let mut rucksack_pile = RucksackPile::new();
        rucksack_pile.add_rucksack(piles[0].clone());
        rucksack_pile.add_rucksack(piles[1].clone());
        rucksack_pile.add_rucksack(piles[2].clone());
        rucksack_piles.push(rucksack_pile);
    }

    let priority_score2: u32 = rucksack_piles
        .iter()
        .map(|r| r.common_items())
        .map(|items| items.iter().map(|i| calculate_score(*i)).sum::<u32>())
        .sum();

    println!(
        "Sum of priorities of 3 elf shared item types is {}.",
        priority_score2
    );
}

#[test]
fn test_split() {
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp";
    let mut rucksack = Rucksack::new();
    rucksack.add_items(input);

    let c1: HashSet<char> = "vJrwpWtwJgWr".chars().collect();
    let c2: HashSet<char> = "hcsFMMfFFhFp".chars().collect();
    let c: Vec<char> = "p".chars().collect();

    assert_eq!(rucksack.compartment1, c1);
    assert_eq!(rucksack.compartment2, c2);
    assert_eq!(rucksack.common_items(), c);
}

#[test]
fn test_calculate_score() {
    assert_eq!(calculate_score('p'), 16);
    assert_eq!(calculate_score('L'), 38);
}
