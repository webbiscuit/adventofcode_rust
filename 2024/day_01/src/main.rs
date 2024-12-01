use std::{
    collections::HashMap,
    io::{self, prelude::*},
};

fn parse_input(lines: Vec<String>) -> (Vec<u32>, Vec<u32>) {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 2 {
            continue;
        }
        let l1 = parts[0].to_string().parse::<u32>().expect("Not a number");
        let l2 = parts[1].to_string().parse::<u32>().expect("Not a number");
        list1.push(l1);
        list2.push(l2);
    }

    (list1, list2)
}

fn total_sorted_differences(list1: &[u32], list2: &[u32]) -> u32 {
    let mut l1_sorted = list1.to_vec();
    let mut l2_sorted = list2.to_vec();

    l1_sorted.sort();
    l2_sorted.sort();

    l1_sorted
        .iter()
        .zip(l2_sorted.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}

fn calculate_simularity_score(list1: &[u32], list2: &[u32]) -> u32 {
    let mut counts = HashMap::new();

    for num in list2.iter() {
        *counts.entry(num).or_insert(0) += 1;
    }

    list1.iter().map(|a| a * counts.get(a).unwrap_or(&0)).sum()
}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();
    let (list1, list2) = parse_input(lines);

    let total_distance = total_sorted_differences(&list1, &list2);
    let similiarity_score = calculate_simularity_score(&list1, &list2);

    println!("Total distance between lists is {}", total_distance);
    println!("The similarity score is {}", similiarity_score);

    Ok(())
}
