use std::io::{self, prelude::*};

type Pin = usize;
type Key = Vec<Pin>;
type Lock = Vec<Pin>;

#[derive(Debug)]
enum ParseType {
    Key,
    Lock,
}

fn parse(lines: &[String]) -> (Vec<Key>, Vec<Lock>) {
    let mut working_item: Option<ParseType> = None;
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    let mut current_pin_count = vec![0, 0, 0, 0, 0];

    for (ix, l) in lines.iter().enumerate() {
        // println!("{:?}", l);
        // println!("Working item is {:?}", working_item);

        if working_item.is_none() {
            if l.starts_with('#') {
                // println!("It's a lock");
                working_item = Some(ParseType::Lock);
                continue;
            }

            if l.starts_with('.') {
                // println!("It's a key");

                working_item = Some(ParseType::Key);
                continue;
            }
        }

        // println!("CHecking these");

        for (ix, c) in l.chars().enumerate() {
            if c == '#' {
                current_pin_count[ix] += 1;
            }
        }

        if l.is_empty() || l.starts_with(' ') || ix == lines.len() - 1 {
            // println!("Done - {} {:?}", l, current_pin_count);

            match working_item {
                Some(ParseType::Key) => {
                    current_pin_count.iter_mut().for_each(|c| *c -= 1);
                    keys.push(current_pin_count.clone());
                }
                Some(ParseType::Lock) => locks.push(current_pin_count.clone()),
                _ => (),
            }

            working_item = None;
            current_pin_count = vec![0, 0, 0, 0, 0];
            continue;
        }
    }

    (keys, locks)
}

fn find_fitting_keys(keys: &[Key], locks: &[Lock]) -> Vec<Key> {
    let mut fitting_keys = vec![];

    for key in keys {
        for lock in locks {
            if key.iter().enumerate().all(|(ix, i)| lock[ix] + i <= 5) {
                fitting_keys.push(key.clone());
            }
        }
    }

    fitting_keys
}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    let (keys, locks) = parse(&lines);

    let fitting_keys = find_fitting_keys(&keys, &locks);

    println!("There are {} fitting keys", fitting_keys.len());

    Ok(())
}
