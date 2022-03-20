use std::collections::HashSet;
use std::error::Error;
use std::io::{self, prelude::*};
use std::ops::Sub;

use itertools::Itertools;

type Signal = String;
type Signals = Vec<Signal>;

type HashedSignal = HashSet<char>;

pub struct Entry {
    signal_patterns: Signals,
    output_values: Signals,
}

fn parse_line(line: &str) -> Entry {
    let mut points = line.split(" | ");

    let signal_patterns = points.next().unwrap().split(' ').map(|p| p.to_string()).collect();
    let output_values = points
        .next()
        .unwrap()
        .split(' ')
        .map(|p| p.to_string())
        .collect();

    Entry {
        signal_patterns,
        output_values
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let entries = lines.map(|s| parse_line(&s.unwrap())).collect::<Vec<_>>();

    let sum = entries.iter().fold(0, |acc, entry| {
        let x: Vec<&str> = entry.output_values.iter().map(AsRef::as_ref).collect();
        acc + find_1478_count(&x)
    });

    println!("Digits 1,4,7,8 appear {} times", sum);

    let sum2 = entries.iter().fold(0, |acc, entry| {
        let signals: Vec<&str> = entry.signal_patterns.iter().map(AsRef::as_ref).collect();

        // println!("Signals {:?}", entry.signal_patterns);
        // println!("Outs {:?}", entry.output_values);

        let calc = SignalCalculator::new(&signals);

        // println!("Outs {:?}", calc);

        let output = 
            calc.to_digit(&entry.output_values[0]).expect("Invalid output 0") as u32 * 1000 +
            calc.to_digit(&entry.output_values[1]).expect("Invalid output 1") as u32 * 100 +
            calc.to_digit(&entry.output_values[2]).expect("Invalid output 2") as u32 * 10 +
            calc.to_digit(&entry.output_values[3]).expect("Invalid output 3") as u32;

        acc + output
    });

    println!("The output digit total is {}", sum2);


    Ok(())
}

fn find_1478_count(segments: &[&str]) -> u32 {
    let count = segments
        .iter()
        .filter(|s| {
            let len = s.len();
            matches!(len, 2 | 4 | 3 | 7)
        })
        .count();

    count as u32
}

#[derive(Debug)]
pub struct SignalCalculator {
    signal_patterns: [HashedSignal; 10],
}

impl SignalCalculator {
    pub fn new(signals: &[&str]) -> SignalCalculator {
        if signals.len() != 10 {
            panic!("SignalCalculator::new() expects 10 signals to initialise");
        }

        let hashed_signals = signals.iter().map(|s| s.chars().collect::<HashedSignal>()).collect_vec();
    
        let one = hashed_signals.iter().find(|s| is_1(s)).expect("No 1").clone();
        let four = hashed_signals.iter().find(|s| is_4(s)).expect("No 4").clone();
        let seven = hashed_signals.iter().find(|s| is_7(s)).expect("No 7").clone();
        let eight = hashed_signals.iter().find(|s| is_8(s)).expect("No 8").clone();
    
        let bd_segment = four.sub(&one);
    
        let three = hashed_signals.iter().find(|s| is_3(s, &one)).expect("No 3").clone();
        let five = hashed_signals.iter().find(|s| is_5(s, &bd_segment)).expect("No 5").clone();
        let two = hashed_signals.iter().find(|s| is_2(s, &one, &bd_segment)).expect("No 2").clone();
    
        let zero = hashed_signals.iter().find(|s| is_0(s, &one, &four)).expect("No 0").clone();
        let six = hashed_signals.iter().find(|s| is_6(s, &five, &four)).expect("No 6").clone();
        let nine = hashed_signals.iter().find(|s| is_9(s, &four)).expect("No 9").clone();

        SignalCalculator {
            signal_patterns: [zero, one, two, three, four, five, six, seven, eight, nine]
        }
    }

    pub fn to_digit(&self, signal: &str) -> Option<u8> {
        let hashed_signal = signal.chars().collect::<HashedSignal>();

        let pos = self.signal_patterns.iter().position(|s| s == &hashed_signal);

        pos.map(|p| p as u8)
    }
}

fn is_1(segment: &HashSet<char>) -> bool {
    let len = segment.len();
    len == 2
}

fn is_4(segment: &HashSet<char>) -> bool {
    let len = segment.len();
    len == 4
}

fn is_7(segment: &HashSet<char>) -> bool {
    let len = segment.len();
    len == 3
}

fn is_8(segment: &HashSet<char>) -> bool {
    let len = segment.len();
    len == 7
}

fn is_3(segment: &HashSet<char>, one_segment: &HashSet<char>) -> bool {
    let len = segment.len();
    len == 5 && one_segment.is_subset(segment)
}

fn is_5(segment: &HashSet<char>, bd_segment: &HashSet<char>) -> bool {
    let len = segment.len();
    
    len == 5 && bd_segment.is_subset(segment)
}

fn is_2(segment: &HashSet<char>, one_segment: &HashSet<char>, bd_segment: &HashSet<char>) -> bool {
    let len = segment.len();

    len == 5 && !is_3(segment, one_segment) && !is_5(segment, bd_segment)
}

fn is_0(segment: &HashSet<char>, one_segment: &HashSet<char>, four_segment: &HashSet<char>) -> bool {
    let len = segment.len();

    len == 6 && one_segment.is_subset(segment) && !is_9(segment, four_segment)
}

fn is_6(segment: &HashSet<char>, five_segment: &HashSet<char>, four_segment: &HashSet<char>) -> bool {
    let len = segment.len();

    len == 6 && five_segment.is_subset(segment) && !is_9(segment, four_segment)
}

fn is_9(segment: &HashSet<char>, four_segment: &HashSet<char>) -> bool {
    let len = segment.len();

    len == 6 && four_segment.is_subset(segment)
}

#[test]
fn test_output_1478_count() {
    let segments = vec!["fdgacbe", "cefdb", "cefbgd", "gcbe"];

    assert_eq!(find_1478_count(&segments), 2);
}

#[test]
fn test_signals_to_digits() {
    let signals = vec!["acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb", "cagedb", "ab"];
    let calc = SignalCalculator::new(&signals);

    assert_eq!(calc.to_digit("cdfeb"), Some(5));
    assert_eq!(calc.to_digit("fcadb"), Some(3));
    assert_eq!(calc.to_digit("cdbaf"), Some(3));
    assert_eq!(calc.to_digit("a"), None);
}