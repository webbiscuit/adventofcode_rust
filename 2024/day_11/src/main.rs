use core::num;
use std::{
    io::{self, prelude::*},
    usize,
};

fn split_number(n: usize) -> (usize, usize) {
    let num_string = n.to_string();
    let len = num_string.len();

    // Only call this for even number of digits
    assert!(len % 2 == 0);

    let (left, right) = num_string.split_at(len / 2);
    (
        left.parse::<usize>().unwrap(),
        right.parse::<usize>().unwrap(),
    )
}

type Stone = usize;

enum Stones {
    Single(Stone),
    Double((Stone, Stone)),
}

fn blink_stone(n: Stone) -> Stones {
    if n == 0 {
        return Stones::Single(1);
    }

    let len = ((n as f64).log10() as usize) + 1;

    if len % 2 == 0 {
        return Stones::Double(split_number(n));
    }

    return Stones::Single(n * 2024);
}

fn parse(lines: &[String]) -> Vec<Stone> {
    lines
        .iter()
        .flat_map(|l| l.split_whitespace().map(|c| c.parse::<Stone>().unwrap()))
        .collect()
}

fn naive_blink(stones: &[Stone]) -> Vec<Stone> {
    let mut new_stones = vec![];

    for s in stones {
        let s = blink_stone(*s);

        match s {
            Stones::Single(s) => new_stones.push(s),
            Stones::Double((s, s2)) => {
                new_stones.push(s);
                new_stones.push(s2);
            }
        }
    }

    // println!("{:?}", new_stones);

    new_stones
}

fn count_after_blinks(start: Vec<Stone>, times: usize) -> usize {
    let mut stones = start;

    for _ in 0..times {
        stones = naive_blink(&stones);
    }

    stones.len()
}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    let nums = parse(&lines);

    // println!("{:?}", nums);

    let result = count_after_blinks(nums.clone(), 25);

    // println!("{:?}", split_number(512072));

    println!("After 25 blinks, there are {} stones", result);

    let result = count_after_blinks(nums, 75);

    // println!("{:?}", split_number(512072));

    println!("After 75 blinks, there are {} stones", result);

    Ok(())
}
