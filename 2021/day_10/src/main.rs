use std::error::Error;
use std::io::{self, prelude::*};

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let nav_subsystem = lines
        .map(|l| l.expect("Could not parse line"))
        .collect::<Vec<String>>();

    let error_score = nav_subsystem.iter().fold(0, |acc, s| {     
        acc + calculate_error_score(s)
    });

    println!("Total syntax error score: {}", error_score);

    Ok(())
}

fn is_syntax_legal(nav_string: &str) -> bool {
    find_first_illegal_character(nav_string) == None
}

fn calculate_error_score(nav_string: &str) -> u32 {
    match find_first_illegal_character(nav_string) {
        Some(')') => return 3,
        Some(']') => return 57,
        Some('}') => return 1197,
        Some('>') => return 25137,
        _ => return 0
    };
}

fn find_first_illegal_character(nav_string: &str) -> Option<char> {
    let mut stack: Vec<char> = vec![];

    for c in nav_string.chars() {
        match c {
            ')' => if stack.pop() != Some('(') { return Some(')');},
            ']' => if stack.pop() != Some('[') { return Some(']');},
            '}' => if stack.pop() != Some('{') { return Some('}');},
            '>' => if stack.pop() != Some('<') { return Some('>');},
            _ => {
                stack.push(c);
            }
        }
    }

    None
}

fn find_completion_characters(nav_string: &str) -> Option<String> {
    let mut stack: Vec<char> = vec![];

    for c in nav_string.chars() {
        match c {
            ')' => if stack.pop() != Some('(') { return Some(')');},
            ']' => if stack.pop() != Some('[') { return Some(']');},
            '}' => if stack.pop() != Some('{') { return Some('}');},
            '>' => if stack.pop() != Some('<') { return Some('>');},
            _ => {
                stack.push(c);
            }
        }
    }

    None
}

#[test]
fn test_find_invalid_scores() {
    let nav = "{([(<{}[<>[]}>{[]{[(<()>";
    assert_eq!(is_syntax_legal(&nav), false);
    assert_eq!(find_first_illegal_character(&nav), Some('}'));
    assert_eq!(calculate_error_score(&nav), 1197);
}

#[test]
fn test_find_valid_scores() {
    let nav = "([])";
    assert_eq!(is_syntax_legal(&nav), true);
    assert_eq!(find_first_illegal_character(&nav), None);
    assert_eq!(calculate_error_score(&nav), 0);
}

#[test]
fn test_find_completion_scores() {
    let nav = "<{([{{}}[<[[[<>{}]]]>[]]";
    // assert_eq!(is_syntax_complete(&nav), false);
    assert_eq!(find_completion_characters(&nav), Some("])}>"));
    // assert_eq!(calculate_completion_score(&nav), 294);
}