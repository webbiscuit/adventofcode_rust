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

    let mut completion_scores = nav_subsystem.iter().map(|s| {
        calculate_completion_score(s)
    }).collect::<Vec<_>>();
    completion_scores.sort();
    completion_scores.retain(|&x| x != 0);

    // println!("Completion scores: {:?}", completion_scores);

    let middle_completion_score = completion_scores[completion_scores.len() / 2];

    println!("Middle completion score: {}", middle_completion_score);

    Ok(())
}

fn is_syntax_legal(nav_string: &str) -> bool {
    find_first_illegal_character(nav_string) == None
}

fn calculate_error_score(nav_string: &str) -> u32 {
    match find_first_illegal_character(nav_string) {
        Some(')') => 3,
        Some(']') => 57,
        Some('}') => 1197,
        Some('>') => 25137,
        _ =>  0
    }
}

fn partner_char(c: char) -> char {
    match c {
        '(' => ')',
        ')' => '(',
        '[' => ']',
        ']' => '[',
        '{' => '}',
        '}' => '{',
        '<' => '>',
        '>' => '<',
        _ => panic!("Unexpected character: {}", c)
    }
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

fn is_syntax_complete(nav_string: &str) -> bool {
    find_completion_characters(nav_string).is_empty()
}

fn find_completion_characters(nav_string: &str) -> Vec<char> {
    let mut stack: Vec<char> = vec![];

    for c in nav_string.chars() {
        match c {
            ')' => if stack.pop() != Some('(') { return vec![]},
            ']' => if stack.pop() != Some('[') { return vec![]},
            '}' => if stack.pop() != Some('{') { return vec![]},
            '>' => if stack.pop() != Some('<') { return vec![]},
            _ => {
                stack.push(c);
            }
        }
    }

    let mut partner_stack: Vec<char> = vec![];

    while !stack.is_empty() {
        partner_stack.push(partner_char(stack.pop().unwrap()));
    }

    partner_stack
}

fn calculate_completion_score(nav_string: &str) -> u64 {
    let completion_characters = find_completion_characters(nav_string);

    completion_characters.iter().fold(0, |acc, c| {
        let mut x = acc * 5;
        x += match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ =>  0
        };
        x
    })
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
    assert_eq!(is_syntax_complete(&nav), false);
    assert_eq!(find_completion_characters(&nav), "])}>".chars().collect_vec());
    assert_eq!(calculate_completion_score(&nav), 294);
}