use std::{
    collections::{BTreeSet, HashMap, HashSet},
    io::{self, prelude::*},
};

use itertools::Itertools;

type ComputerName = String;
type ComputerLink = (ComputerName, ComputerName);

fn parse(lines: &[String]) -> Vec<ComputerLink> {
    lines
        .iter()
        .map(|l| {
            let (a, b) = l.split_once('-').unwrap();
            (a.to_string(), b.to_string())
        })
        .collect()
}

fn find_triangle_networks(links: &[ComputerLink]) -> HashSet<BTreeSet<ComputerName>> {
    let mut results = HashSet::new();

    let mut computer_map: HashMap<ComputerName, HashSet<ComputerName>> = HashMap::new();

    for (c1, c2) in links {
        computer_map
            .entry(c1.clone())
            .or_insert(HashSet::new())
            .insert(c2.clone());

        computer_map
            .entry(c2.clone())
            .or_insert(HashSet::new())
            .insert(c1.clone());
    }

    // println!("Map {:?}", computer_map);

    for (k, values) in computer_map.iter() {
        for v in values {
            if let Some(neighbors) = computer_map.get(v) {
                let shared: BTreeSet<_> = neighbors.intersection(values).cloned().collect();

                for u in shared {
                    let mut triangle = BTreeSet::new();
                    triangle.insert(k.clone());
                    triangle.insert(v.clone());
                    triangle.insert(u.clone());

                    results.insert(triangle);
                }
            }
        }
    }

    results
}

fn is_clique(
    members: &HashSet<ComputerName>,
    computer_map: &HashMap<ComputerName, HashSet<ComputerName>>,
) -> bool {
    for a in members {
        for b in members {
            if a != b && !computer_map.get(a).unwrap().contains(b) {
                return false;
            }
        }
    }
    true
}

fn find_largest_clique_network(links: &[ComputerLink]) -> HashSet<ComputerName> {
    let mut computer_map: HashMap<ComputerName, HashSet<ComputerName>> = HashMap::new();

    for (c1, c2) in links {
        computer_map
            .entry(c1.clone())
            .or_insert(HashSet::new())
            .insert(c2.clone());

        computer_map
            .entry(c2.clone())
            .or_insert(HashSet::new())
            .insert(c1.clone());
    }

    // println!("Map {:?}", computer_map);

    let mut largest_clique = HashSet::new();
    let nodes: Vec<_> = computer_map.keys().cloned().collect();

    // Iterate over all subsets of nodes
    for subset_size in 3..5 {
        // for subset_size in 2..=nodes.len() {
        for subset in nodes.iter().cloned().combinations(subset_size) {
            let subset_set: HashSet<_> = subset.into_iter().collect();
            if subset_set.len() > largest_clique.len() && is_clique(&subset_set, &computer_map) {
                largest_clique = subset_set;
            }
        }
    }

    largest_clique
}

fn to_password(network: &HashSet<String>) -> String {
    let mut sorted: Vec<&String> = network.iter().collect::<Vec<_>>();

    sorted.sort();

    sorted.into_iter().join(",")
}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    let links = parse(&lines);

    // println!("Links {:?}", links);

    let networks = find_triangle_networks(&links);

    // println!("Triangles {:?}", networks);

    let networks: HashSet<_> = networks
        .iter()
        .filter(|&s| s.iter().any(|c| c.starts_with('t')))
        .collect();

    // println!("Triangles {:?}", networks);

    let result = networks.len();

    println!(
        "There are {} sets of interconnected computers linking to a 't'",
        result
    );

    let largest_network = find_largest_clique_network(&links);

    // println!("Largest = {:?}", largest_network);

    let password = to_password(&largest_network);

    println!("Password is {}", password);

    Ok(())
}

// !74
// < 2462
