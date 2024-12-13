use std::{
    collections::HashMap,
    io::{self, prelude::*},
    usize,
};

fn split_number(n: Stone) -> (Stone, Stone) {
    let len = ((n as f64).log10() as usize) + 1;
    let half_len = len / 2;
    let divisor = 10_usize.pow(half_len as u32);
    (n / divisor, n % divisor)
}

fn parse(lines: &[String]) -> Vec<Stone> {
    lines
        .iter()
        .flat_map(|l| l.split_whitespace().map(|c| c.parse::<Stone>().unwrap()))
        .collect()
}

fn blink_stone(n: Stone) -> Vec<Stone> {
    if n == 0 {
        return vec![1];
    }

    let len = ((n as f64).log10() as usize) + 1;

    if len % 2 == 0 {
        let (left, right) = split_number(n);
        return vec![left, right];
    }

    return vec![n * 2024];
}

type Stone = usize;

struct CachedCounter {
    blink_cache: HashMap<Stone, Vec<Stone>>,
    stone_counts: HashMap<Stone, usize>,
}

impl CachedCounter {
    fn new() -> Self {
        CachedCounter {
            blink_cache: HashMap::new(),
            stone_counts: HashMap::new(),
        }
    }

    fn cached_blink_stone(&mut self, n: Stone) -> Vec<Stone> {
        if let Some(lookup) = self.blink_cache.get(&n) {
            return lookup.clone();
        }

        let answer = blink_stone(n);

        self.blink_cache.insert(n, answer.clone());

        answer
    }

    fn naive_blink(&mut self, stones: &[Stone]) -> Vec<Stone> {
        stones
            .iter()
            .flat_map(|&s| self.cached_blink_stone(s))
            .collect()
    }

    fn counted_blink(&mut self) {
        let stone_counts: Vec<(Stone, usize)> =
            self.stone_counts.iter().map(|(&k, &v)| (k, v)).collect();

        // println!("stone_couints {:?}", stone_counts);

        for (k, v) in stone_counts {
            let new_stone = self.cached_blink_stone(k);

            // println!("Getting a stone for {} - {:?}", k, new_stone);

            for stone in new_stone {
                *self.stone_counts.entry(stone).or_insert(0) += v;
            }

            *self.stone_counts.entry(k).or_insert(0) -= v;
        }
    }

    fn count_after_blinks(&mut self, start: Vec<Stone>, times: usize) -> usize {
        self.stone_counts.clear();

        for stone in start {
            *self.stone_counts.entry(stone).or_insert(0) += 1;
        }

        for _ in 0..times {
            self.counted_blink();
        }

        self.stone_counts.values().sum()
    }
}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    let nums = parse(&lines);

    let mut cached_counter = CachedCounter::new();

    let result = cached_counter.count_after_blinks(nums.clone(), 25);

    println!("After 25 blinks, there are {} stones", result);

    let result = cached_counter.count_after_blinks(nums, 75);

    println!("After 75 blinks, there are {} stones", result);

    Ok(())
}
