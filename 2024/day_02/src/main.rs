use std::io::{self, prelude::*};

#[derive(Debug)]
struct Report(Vec<Level>);

#[derive(Debug)]
struct Level(usize);

fn parse(data: &[String]) -> Vec<Report> {
    let reports = data.iter().map(|l| {
        l.split_whitespace()
            .map(|item| Level(item.parse::<usize>().expect("Cannot parse item")))
            .collect()
    });

    let reports = reports.map(|r| Report(r)).collect();

    reports
}

fn is_valid_ascending(l1: &Level, l2: &Level) -> bool {
    let l1 = l1.0;
    let l2 = l2.0;

    if l2 > l1 {
        let diff = l2 - l1;
        return diff >= 1 && diff <= 3;
    }

    return false;
}

fn is_valid_descending(l1: &Level, l2: &Level) -> bool {
    let l1 = l1.0;
    let l2 = l2.0;

    if l2 < l1 {
        let diff = l1 - l2;
        return diff >= 1 && diff <= 3;
    }

    return false;
}

fn is_valid_report(report: &Report) -> bool {
    let levels = &report.0;

    if levels.len() < 3 {
        return false;
    }

    let mut iter = levels.windows(2);

    let first_window = iter.next().unwrap();

    let comp_fn: fn(&Level, &Level) -> bool =
        if is_valid_ascending(&first_window[0], &first_window[1]) {
            is_valid_ascending
        } else if is_valid_descending(&first_window[0], &first_window[1]) {
            is_valid_descending
        } else {
            return false;
        };

    iter
        // .inspect(|f| print!("{:?}", f))
        .all(|w| comp_fn(&w[0], &w[1]))
}

fn count_safe_reports(reports: &[Report]) -> usize {
    reports.iter().filter(|r| is_valid_report(r)).count()
}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();
    let reports = parse(&lines);

    let safe_reports = count_safe_reports(&reports);

    println!("There are {} safe reports", safe_reports);

    Ok(())
}
