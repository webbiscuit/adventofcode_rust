use std::error::Error;
use std::io;

fn chars_in_code(str: &str) -> usize {
    str.len()
}

fn chars_in_memory(str: &str) -> usize {
    let mut count = 0;

    let mut chars = str.chars();
    while let Some(c) = chars.next() {
        // println!("c: {}", c);

        if c == '\\' {
            count += 1;
            if chars.next() == Some('x') {
                chars.next();
                chars.next();
            }
        } else if c == '"' {
        } else {
            count += 1;
        }
    }

    count
}

fn encode(str: &str) -> String {
    let mut encoded = String::new();
    encoded.push('"');
    for c in str.chars() {
        if c == '\\' || c == '"' {
            encoded.push('\\');
        }
        encoded.push(c);
    }
    encoded.push('"');
    encoded
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines = io::stdin().lines();

    let mut total_code = 0;
    let mut total_memory = 0;
    let mut total_encoded = 0;

    for line in lines {
        let line = line?;
        let code = chars_in_code(&line);
        let memory = chars_in_memory(&line);
        let encoded = chars_in_code(&encode(&line));

        // println!("code: {}, memory: {}", code, memory);

        total_code += code;
        total_memory += memory;
        total_encoded += encoded;
    }

    println!("total code - total memory = {}", total_code - total_memory);
    println!(
        "encoded length - code length = {}",
        total_encoded - total_code
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(r#""""#, 2)]
    #[test_case(r#""abc""#, 5)]
    #[test_case(r#""aaa\"aaa""#, 10)]
    #[test_case(r#""\x27""#, 6)]
    fn test_chars_of_code(s: &str, expected: usize) {
        let chars_in_code = chars_in_code(s);

        assert_eq!(chars_in_code, expected);
    }

    #[test_case(r#""""#, 0)]
    #[test_case(r#""abc""#, 3)]
    #[test_case(r#""aaa\"aaa""#, 7)]
    #[test_case(r#""\x27""#, 1)]
    fn test_chars_in_memory(s: &str, expected: usize) {
        let chars_in_code = chars_in_memory(s);

        assert_eq!(chars_in_code, expected);
    }

    #[test_case(r#""""#, 6)]
    #[test_case(r#""abc""#, 9)]
    #[test_case(r#""aaa\"aaa""#, 16)]
    #[test_case(r#""\x27""#, 11)]
    fn test_encoded(s: &str, expected: usize) {
        let encoded_chars_in_code = chars_in_code(&encode(s));

        assert_eq!(encoded_chars_in_code, expected);
    }
}
