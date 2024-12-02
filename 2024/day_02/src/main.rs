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

    let comp_fn = find_comparison_func(&first_window);

    iter
        // .inspect(|f| print!("{:?}", f))
        .all(|w| comp_fn(&w[0], &w[1]))
}

fn count_safe_reports(reports: &[Report]) -> usize {
    reports.iter().filter(|r| is_valid_report(r)).count()
}

fn _is_valid_report_with_damper(report: &Report, can_damp: bool) -> bool {
    let levels = &report.0;

    if levels.len() < 3 {
        return false;
    }

    let mut iter = levels.windows(2);

    let first_window = iter.next().unwrap();

    let comp_fn = find_comparison_func(&first_window);

    // println!("New inspection");

    // It's a latch
    let mut consumed_damper = !can_damp;
    let mut damping = false;
    let mut last_good: Option<(Level, Level)> = Some((first_window[0], first_window[1]));

    iter
        // .inspect(|w| println!("Current : {:?}, ", w))
        .all(|w| {
            if damping {
                damping = false;
                // println!("DAMP {:?} {:?}, ", &last_good.unwrap(), &w[1]);
                return comp_fn(&last_good.unwrap().0, &w[0])
                    || comp_fn(&last_good.unwrap().1, &w[1]);
            }

            if comp_fn(&w[0], &w[1]) {
                // println!("Checks out");
                last_good = Some((w[0], w[1]));

                return true;
            }

            if consumed_damper {
                // println!("No damper");

                return false;
            }

            consumed_damper = true;
            damping = true;

            // println!("Damping");

            true

            // >639, <676
            // !705
            // !654
            // !665
            // !632
            // !641
            // !645
            // !647
        })
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
    // Generate all the combos
    // Find any that work


    _is_valid_report_with_damper(report, true)
        || _is_valid_report_with_damper(&Report(report.0.iter().skip(1).cloned().collect()), false)
        || _is_valid_report_with_damper(
            &Report(
                report
                    .0
                    .iter()
                    .enumerate()
                    .filter_map(|(i, &x)| if i != 1 { Some(x) } else { None })
                    .collect(),
            ),
            false,
        )
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
