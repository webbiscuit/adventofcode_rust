use std::{
    collections::{HashMap, HashSet},
    io::{self, prelude::*},
};

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

fn to_prices(nums: &[isize]) -> Vec<Vec<isize>> {
    nums.iter()
        .map(|&n| {
            let mut seed = n;
            let mut output = vec![];

            for _ in 0..2000 {
                let out = sequence(seed);
                seed = out;

                output.push(out % 10);
            }

            output
        })
        .collect::<Vec<_>>()
}

fn to_diffs(nums: &[isize]) -> Vec<isize> {
    nums.windows(2)
        .map(|pair| pair[1] - pair[0])
        .collect::<Vec<_>>()
}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    let nums: Vec<isize> = lines.iter().map(|l| l.parse::<isize>().unwrap()).collect();

    let results = nums
        .iter()
        .map(|&n| {
            let mut seed = n;
            let mut out = 0;

            for _ in 0..2000 {
                out = sequence(seed);
                seed = out;
            }

            out
        })
        .collect::<Vec<_>>();

    let result: isize = results.iter().sum();

    println!("Sum of 2000th generated number is {}", result);

    let prices = to_prices(&nums);
    // let prices = [[3, 0, 6, 5, 4, 4, 6]];
    let diffs: Vec<Vec<isize>> = prices.iter().map(|p| to_diffs(p)).collect();
    // Count up values pointed to by all the windows of 4
    let mut values_seen: HashMap<Vec<isize>, isize> = HashMap::new();

    diffs.iter().enumerate().for_each(|(d_ix, d)| {
        let mut seen: HashSet<Vec<isize>> = HashSet::new();

        d.windows(4).enumerate().for_each(|(ix, w)| {
            let key = w.to_vec();

            if seen.contains(&key) {
                return;
            }

            seen.insert(key.clone());

            let price = prices[d_ix][ix + 4];

            (*values_seen.entry(key).or_insert(0)) += price;
        });
    });

    let highest = values_seen.iter().max_by_key(|&(_, v)| v).unwrap();

    // println!("Prices {:?}", prices);
    // println!("diffs {:?}", diffs);
    // println!("values {:?}", values_seen);
    // println!("highest {:?}", highest);

    println!("Most banana is {}", highest.1);

    Ok(())
}

// < 1848
