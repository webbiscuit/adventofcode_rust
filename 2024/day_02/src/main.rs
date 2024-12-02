use std::io::{self, prelude::*};

#[derive(Debug)]
struct Report(Vec<Level>);

#[derive(Debug, Clone, Copy)]
struct Level(usize);

fn parse(data: &[String]) -> Vec<Report> {
    let reports = data
        .iter()
        .map(|l| {
            Report(
                l.split_whitespace()
                    .map(|item| Level(item.parse::<usize>().expect("Cannot parse item")))
                    .collect(),
            )
        })
        .collect();

    reports
}

fn is_valid_ascending(l1: &Level, l2: &Level) -> bool {
    let l1 = l1.0;
    let l2 = l2.0;

    if l2 > l1 {
        let diff = l2 - l1;
        return (1..=3).contains(&diff);
    }

    false
}

fn is_valid_descending(l1: &Level, l2: &Level) -> bool {
    let l1 = l1.0;
    let l2 = l2.0;

    if l2 < l1 {
        let diff = l1 - l2;
        return (1..=3).contains(&diff);
    }

    false
}

fn is_valid_report(report: &Report) -> bool {
    let levels = &report.0;

    if levels.len() < 3 {
        return false;
    }

    let mut iter = levels.windows(2);

    let first_window = iter.next().unwrap();

    let comp_fn = find_comparison_func(first_window);

    iter.all(|w| comp_fn(&w[0], &w[1]))
}

fn count_safe_reports(reports: &[Report]) -> usize {
    reports.iter().filter(|r| is_valid_report(r)).count()
}

fn find_comparison_func(first_window: &[Level]) -> fn(&Level, &Level) -> bool {
    let comp_fn: fn(&Level, &Level) -> bool =
        if is_valid_ascending(&first_window[0], &first_window[1]) {
            is_valid_ascending
        } else if is_valid_descending(&first_window[0], &first_window[1]) {
            is_valid_descending
        } else {
            |_, _| false
        };
    comp_fn
}

fn is_valid_report_with_damper(report: &Report) -> bool {
    // Generate all reports with 1 missing element
    let all_report_combos: Vec<Report> = report
        .0
        .iter()
        .enumerate()
        .map(|(ix, _)| {
            Report(
                report
                    .0
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| *j != ix)
                    .map(|(_, l)| *l)
                    .collect(),
            )
        })
        .collect();

    // Check if any of these reports is valid
    all_report_combos.iter().any(is_valid_report)
}

fn count_safe_damper_reports(reports: &[Report]) -> usize {
    reports
        .iter()
        // .inspect(|r| println!("Report {:?} - {}", r, is_valid_report_with_damper(r)))
        .filter(|r| is_valid_report_with_damper(r))
        .count()
}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();
    let reports = parse(&lines);

    let safe_reports = count_safe_reports(&reports);

    println!("There are {} safe reports", safe_reports);

    let safe_damper_reports = count_safe_damper_reports(&reports);

    println!(
        "There are {} safe reports using the Problem Damper",
        safe_damper_reports
    );

    Ok(())
}
