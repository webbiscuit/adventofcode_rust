use std::{
    collections::HashSet,
    io::{self, prelude::*},
};

fn find_first_marker(unique_window_size: usize, input: &str) -> Option<usize> {
    for (ix, char_check) in input.as_bytes().windows(unique_window_size).enumerate() {
        let found: HashSet<&u8> = HashSet::from_iter(char_check.iter());
        if found.len() == unique_window_size {
            return Some(ix + unique_window_size);
        }
    }

    None
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let line = lines.next().unwrap().unwrap();

    let start_marker = find_first_marker(4, &line).unwrap();
    let start_message_marker = find_first_marker(14, &line).unwrap();

    println!("Start of packet marker is at position {}.", start_marker);
    println!(
        "Start of message marker is at position {}.",
        start_message_marker
    );
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 5)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 6)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)]
    fn test_find_first_marker(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(expected, find_first_marker(4, input).unwrap())
    }

    #[rstest]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 23)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 23)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26)]
    fn test_find_first_message_marker(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(expected, find_first_marker(14, input).unwrap())
    }
}
