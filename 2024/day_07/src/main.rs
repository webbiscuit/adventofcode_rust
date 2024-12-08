use std::io::{self, prelude::*};

#[derive(Clone, Copy, Debug)]
enum Operation {
    Add,
    Multiply,
    Concat,
}

fn check_nums_rec(target: isize, nums: &[isize], ops: Vec<Operation>) -> Option<Vec<Operation>> {
    if nums.is_empty() {
        // println!("Not found here");
        return None;
    }

    let n = nums[0];

    // println!("T: {}, n: {}", target, n);

    if (target - n) == 0 {
        // println!("WE ARE DONE");
        // We're done!!
        return Some(ops);
    }

    if (target % n) == 0 {
        // println!("It's a mul");
        let mut ops = ops.clone();
        ops.push(Operation::Multiply);

        let ret = check_nums_rec(target / n, &nums[1..], ops);

        if let Some(result) = ret {
            return Some(result);
        }
    }

    if (target - n) > 0 {
        // println!("It's an add");
        let mut ops = ops.clone();
        ops.push(Operation::Add);
        let ret = check_nums_rec(target - n, &nums[1..], ops);

        if let Some(result) = ret {
            return Some(result);
        }
    }

    None
}

fn check_nums_with_concats_rec(
    target: isize,
    nums: &[isize],
    ops: Vec<Operation>,
) -> Option<Vec<Operation>> {
    // println!("T: {}", target);

    if nums.is_empty() {
        // println!("Not found here");
        return None;
    }

    let n = nums[0];

    // println!("T: {}, n: {}", target, n);

    if (target - n) == 0 {
        // println!("WE ARE DONE");
        // println!("{:?}", ops.clone().iter().rev().collect::<Vec<_>>());
        // We're done!!
        return Some(ops);
    }

    if (target % n) == 0 {
        // println!("It's a mul");
        let mut ops = ops.clone();
        ops.push(Operation::Multiply);

        let ret = check_nums_with_concats_rec(target / n, &nums[1..], ops);

        if let Some(result) = ret {
            return Some(result);
        }
    }

    if (target - n) > 0 {
        // println!("It's an add");
        let mut ops = ops.clone();
        ops.push(Operation::Add);
        let ret = check_nums_with_concats_rec(target - n, &nums[1..], ops);

        if let Some(result) = ret {
            return Some(result);
        }
    }

    let num_digits = n.to_string().len();
    let mod_value = 10_isize.pow(num_digits as u32);
    // println!("CHECK {}", target % mod_value);

    if target % mod_value == n {
        // println!("Concat");
        let mut ops = ops.clone();
        ops.push(Operation::Concat);
        let ret = check_nums_with_concats_rec(target / mod_value, &nums[1..], ops);

        if let Some(result) = ret {
            return Some(result);
        }
    }

    None
}

type Targets = Vec<(isize, Vec<isize>)>;

fn total_possible_calibrations(targets: &Targets) -> isize {
    let possibles = targets
        .iter()
        .map(|(target, nums)| {
            let rev_list = nums.iter().rev().copied().collect::<Vec<_>>();

            let res = check_nums_rec(*target, &rev_list, vec![])
                .into_iter()
                .rev()
                .collect::<Vec<_>>();

            (target, res)
        })
        .filter(|(_, ops)| !ops.is_empty());

    possibles.map(|(t, _)| t).sum()
}

fn total_possible_calibrations_with_concats(targets: &Targets) -> isize {
    let possibles = targets
        .iter()
        .map(|(target, nums)| {
            let rev_list = nums.iter().rev().copied().collect::<Vec<_>>();

            let res = check_nums_with_concats_rec(*target, &rev_list, vec![])
                .into_iter()
                .rev()
                .collect::<Vec<_>>();

            (target, res)
        })
        .filter(|(_, ops)| !ops.is_empty());

    possibles.map(|(t, _)| t).sum()
}

fn parse(lines: &[String]) -> Targets {
    lines
        .iter()
        .map(|l| {
            let split = l.split_once(":").expect("Cannot parse");

            let (target, rest) = split;
            let target = target.parse::<isize>().expect("Cannot parse");

            let nums = rest
                .split_whitespace()
                .map(|i| i.parse::<isize>().unwrap())
                .collect::<Vec<isize>>();

            (target, nums)
        })
        .collect()
}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    let targets = parse(&lines);
    let result = total_possible_calibrations(&targets);

    println!("Total calibration result is {}", result);

    let result2 = total_possible_calibrations_with_concats(&targets);

    println!("Total calibration result with concats is {}", result2);

    // let t = 1234;
    // println!("{}", t % 100);

    Ok(())
}
