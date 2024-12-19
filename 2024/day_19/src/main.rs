use std::{
    collections::HashMap,
    io::{self, prelude::*},
};

type Towel = String;
type Pattern = String;

fn parse(lines: &[String]) -> (Vec<Towel>, Vec<Pattern>) {
    let towels: Vec<String> = lines[0].split(", ").map(|s| s.to_string()).collect();

    let patterns = lines[2..].iter().map(|s| s.to_string()).collect();

    (towels, patterns)
}

// fn is_pattern_possible(towels: &[Towel], pattern: &str) -> bool {
//     if pattern.is_empty() {
//         return true;
//     }

//     towels
//         .iter()
//         .any(|t| pattern.starts_with(t) && is_pattern_possible(towels, &pattern[t.len()..]))
// }

// fn find_possible_patterns(towels: &[Towel], patterns: &[Pattern]) -> Vec<Pattern> {
//     patterns
//         .iter()
//         .filter(|p| is_pattern_possible(towels, p))
//         .cloned()
//         .collect()
// }

fn find_possible_patterns_cached(towels: &[Towel], patterns: &[Pattern]) -> Vec<Pattern> {
    let mut cache = Cache::new();

    patterns
        .iter()
        .filter(|p| cache.is_pattern_possible(towels, p))
        .cloned()
        .collect()
}

struct Cache {
    cached_pattern: HashMap<String, bool>,
}

impl Cache {
    fn new() -> Cache {
        Cache {
            cached_pattern: HashMap::new(),
        }
    }

    fn is_pattern_possible(&mut self, towels: &[Towel], pattern: &str) -> bool {
        if self.cached_pattern.contains_key(pattern) {
            return self.cached_pattern[pattern];
        }

        let result = self._is_pattern_possible_cached(towels, pattern);

        self.cached_pattern.insert(pattern.to_string(), result);

        result
    }

    fn _is_pattern_possible_cached(&mut self, towels: &[Towel], pattern: &str) -> bool {
        if pattern.is_empty() {
            return true;
        }

        // println!("Pattern {}", pattern);

        towels.iter().any(|t| {
            pattern.starts_with(t) && self.is_pattern_possible(towels, &pattern[t.len()..])
        })
    }
}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    let (towels, patterns) = parse(&lines);

    // println!("Towels: {:?}", towels);
    // println!("Patterns: {:?}", patterns);

    let result = find_possible_patterns_cached(&towels, &patterns);

    // println!("Results: {:?}", result);

    println!("There are {} designs possible", result.len());

    Ok(())
}
