use std::error::Error;
use std::io;

fn count_visits(path: &str) -> u32 {
    let mut visited = find_visits(path);

    visited.sort();
    visited.dedup();
    visited.len() as u32
}

fn find_visits(path: &str) -> Vec<(i32, i32)> {
    let mut x = 0;
    let mut y = 0;
    let mut visited = vec![(x, y)];
    for c in path.chars() {
        match c {
            '^' => y += 1,
            'v' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => (),
        }
        visited.push((x, y));
    }
    visited
}

fn santa_and_robo_visits(path: &str) -> u32 {
    let mut santa_path = String::new();
    let mut robo_path = String::new();

    for (ix, c) in path.chars().enumerate() {
        if ix % 2 == 0 {
            santa_path.push(c);
        } else {
            robo_path.push(c);
        }
    }

    let mut santa_visits = find_visits(&santa_path);
    let mut robo_visits = find_visits(&robo_path);

    santa_visits.append(&mut robo_visits);

    santa_visits.sort();
    santa_visits.dedup();
    santa_visits.len() as u32
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut lines = io::stdin().lines();

    let path = lines.next().unwrap().unwrap();

    println!("Santa visited {} houses", count_visits(&path));
    println!(
        "Santa and robo-santa visited {} houses",
        santa_and_robo_visits(&path)
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(">", 2)]
    #[test_case("^>v<", 4)]
    #[test_case("^v^v^v^v^v", 2)]
    fn test_santa_travel(path: &str, expected: u32) {
        let houses_visited = count_visits(path);

        assert_eq!(houses_visited, expected);
    }

    #[test_case("^v", 3; "Simple 3 steps")]
    #[test_case("^>v<", 3)]
    #[test_case("^v^v^v^v^v", 11)]
    fn test_robo_santa_travel(path: &str, expected: u32) {
        let houses_visited = santa_and_robo_visits(path);

        assert_eq!(houses_visited, expected);
    }
}
