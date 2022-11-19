use md5::{self, Digest};
use std::error::Error;
use std::io;

fn calculate_hash(code: &str, value: u32) -> Digest {
    let mut hasher = md5::Context::new();
    hasher.consume(code);
    hasher.consume(value.to_string());
    hasher.compute()
}

fn find_hash(code: &str, prefix: &str) -> u32 {
    let mut value = 0;
    loop {
        let hash = calculate_hash(code, value);
        let hash_string = format!("{:x}", hash);
        if hash_string.starts_with(prefix) {
            break;
        }
        value += 1;
    }

    value
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut lines = io::stdin().lines();

    let path = lines.next().unwrap().unwrap();

    println!("Prefix 00000: {}", find_hash(&path, "00000"));
    println!("Prefix 000000: {}", find_hash(&path, "000000"));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("abcdef", 609043)]
    #[test_case("pqrstuv", 1048970)]
    fn test_five_zero_hash(path: &str, expected: u32) {
        let hash_value = find_hash(path, "00000");

        assert_eq!(hash_value, expected);
    }
}
