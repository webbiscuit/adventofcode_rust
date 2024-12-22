use std::io::{self, prelude::*};

fn mix(num: isize, seed: isize) -> isize {
    num ^ seed
}

fn prune(num: isize) -> isize {
    num % 16777216
}

fn sequence(seed: isize) -> isize {
    let mut num = seed;

    // Seq 1
    num *= 64;

    // Mix
    num = mix(num, seed);

    // Prune
    num = prune(num);

    // Seq 2
    let seed = num;
    num /= 32;

    num = mix(num, seed);

    num = prune(num);

    // Seq 3
    let seed = num;

    num *= 2048;

    num = mix(num, seed);

    num = prune(num);

    num
}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    let nums = lines.iter().map(|l| l.parse::<isize>().unwrap());

    // let test_seq_1 = sequence(123);

    let results = nums.map(|n| {
        let mut seed = n;
        let mut out = 0;

        for _ in 0..2000 {
            out = sequence(seed);
            seed = out;
        }

        out
    });

    let result: isize = results.sum();

    println!("Sum of 2000th generated number is {}", result);

    Ok(())
}
