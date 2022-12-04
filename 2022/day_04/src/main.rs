use std::{
    io::{self, prelude::*},
    ops::Range,
};

fn is_fully_contained_in(a: &Range<u32>, b: &Range<u32>) -> bool {
    (a.start >= b.start && a.end <= b.end) || (b.start >= a.start && b.end <= a.end)
}

fn is_partially_contained_in(a: &Range<u32>, b: &Range<u32>) -> bool {
    (a.start >= b.start && a.start <= b.end)
        || (a.end >= b.start && a.end <= b.end)
        || (b.start >= a.start && b.start <= a.end)
        || (b.end >= a.start && b.end <= a.end)
}

fn range_from_elf_string(s: &str) -> Range<u32> {
    let (start, end) = s.split_once('-').unwrap();
    start.parse::<u32>().unwrap()..end.parse::<u32>().unwrap()
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut ranges: Vec<(Range<u32>, Range<u32>)> = Vec::new();

    for line in lines {
        let line = line.unwrap();
        let (elf1, elf2) = line.split_once(',').unwrap();
        let elf1_range = range_from_elf_string(elf1);
        let elf2_range = range_from_elf_string(elf2);

        ranges.push((elf1_range, elf2_range));
    }

    let overlapping_range_count = ranges
        .iter()
        .filter(|(elf1_range, elf2_range)| is_fully_contained_in(elf1_range, elf2_range))
        .count();

    let partial_overlapping_range_count = ranges
        .iter()
        .filter(|(elf1_range, elf2_range)| is_partially_contained_in(elf1_range, elf2_range))
        .count();

    println!(
        "Assignment pairs with full overlap: {}.",
        overlapping_range_count
    );
    println!(
        "Assignment pairs with partial overlap: {}.",
        partial_overlapping_range_count
    );
}

#[test]
fn test_ranges_fully_in() {
    assert!(is_fully_contained_in(&(0..10), &(0..10)));
    assert!(is_fully_contained_in(&(0..10), &(0..5)));
    assert!(is_fully_contained_in(&(0..10), &(5..10)));
    assert!(is_fully_contained_in(&(0..10), &(5..5)));
    assert!(is_fully_contained_in(&(9..10), &(0..10)));
    assert!(!is_fully_contained_in(&(9..10), &(0..9)));
}

#[test]
fn test_ranges_partially_in() {
    assert!(is_partially_contained_in(&(0..10), &(0..10)));
    assert!(is_partially_contained_in(&(0..10), &(0..5)));
    assert!(is_partially_contained_in(&(0..10), &(5..10)));
    assert!(is_partially_contained_in(&(0..10), &(5..5)));
    assert!(is_fully_contained_in(&(9..10), &(0..10)));
    assert!(is_partially_contained_in(&(9..10), &(0..9)));
    assert!(!is_partially_contained_in(&(10..20), &(0..9)));
}
