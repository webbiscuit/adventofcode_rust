use std::io::{self, prelude::*};

type Towel = String;
type Pattern = String;

fn parse(lines: &[String]) -> (Vec<Towel>, Vec<Pattern>) {
    let towels: Vec<String> = lines[0].split(", ").map(|s| s.to_string()).collect();

    let patterns = lines[2..].iter().map(|s| s.to_string()).collect();

    (towels, patterns)
}

fn is_pattern_possible(towels: &[Towel], pattern: &str) -> bool {
    if pattern.is_empty() {
        return true;
    }

    towels
        .iter()
        .any(|t| pattern.starts_with(t) && is_pattern_possible(towels, &pattern[t.len()..]))
}

fn find_possible_patterns(towels: &[Towel], patterns: &[Pattern]) -> Vec<Pattern> {
    patterns
        .iter()
        .filter(|p| is_pattern_possible(towels, p))
        .cloned()
        .collect()
}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    let (towels, patterns) = parse(&lines);

    // println!("Towels: {:?}", towels);
    // println!("Patterns: {:?}", patterns);

    let result = find_possible_patterns(&towels, &patterns);

    // println!("Results: {:?}", result);

    println!("There are {} designs possible", result.len());

    Ok(())
}
