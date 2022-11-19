use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let lines = io::stdin().lines();

    let strings = lines
        .map(|line| line.unwrap().parse().unwrap())
        .collect::<Vec<String>>();

    let nice_strings = strings.iter().filter(|&line| is_nice(line)).count();
    let nice_strings2 = strings.iter().filter(|&line| is_nice2(line)).count();

    println!("There are {} nice strings", nice_strings);
    println!("There are {} v2 nice strings", nice_strings2);

    Ok(())
}

fn is_nice(s: &str) -> bool {
    let mut test = s.chars().filter(|c| "aeiou".contains(*c)).count() >= 3;
    test &= s.chars().zip(s.chars().skip(1)).any(|(a, b)| a == b);
    test &= !s.contains("ab") && !s.contains("cd") && !s.contains("pq") && !s.contains("xy");

    test
}

fn is_nice2(s: &str) -> bool {
    let mut test = s.chars().zip(s.chars().skip(2)).any(|(a, b)| a == b);
    test &= s
        .chars()
        .zip(s.chars().skip(1))
        .enumerate()
        .any(|(i, (a, b))| {
            // println!("1 {} {} {}", i, a, b);
            s.chars()
                .skip(i + 2)
                .zip(s.chars().skip(i + 3))
                .any(|(c, d)| {
                    // println!("2 {} {} {}", i, c, d);
                    return a == c && b == d;
                })
        });

    test
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("ugknbfddgicrmopn", true)]
    #[test_case("aaa", true)]
    #[test_case("jchzalrnumimnmhp", false)]
    #[test_case("haegwjzuvuyypxyu", false)]
    #[test_case("dvszwmarrgswjxmb", false)]
    fn test_naughty_nice(s: &str, expected: bool) {
        let is_nice = is_nice(s);

        assert_eq!(is_nice, expected);
    }

    #[test_case("qjhvhtzxzqqjkmpb", true)]
    #[test_case("xxyxx", true)]
    #[test_case("uurcxstgmygtbstg", false)]
    #[test_case("ieodomkazucvgmuy", false)]
    fn test_naughty_nice2(s: &str, expected: bool) {
        let is_nice = is_nice2(s);

        assert_eq!(is_nice, expected);
    }
}
