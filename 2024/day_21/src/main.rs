use std::io::{self, prelude::*};

struct Keypad {
    key_map: Vec<Option<char>>,
    width: usize,
    height: usize,
}

impl Keypad {
    fn new(layout: Vec<Option<char>>, width: usize, height: usize) -> Self {
        Keypad {
            key_map: layout,
            width,
            height,
        }
    }

    fn route(&self, from: char, to: char) -> Vec<char> {
        let mut route = vec![];

        let from_ix = self
            .key_map
            .iter()
            .position(|k| k.as_ref() == Some(&from))
            .unwrap();
        let from_pos = (from_ix % self.width, from_ix / self.width);

        let to_ix = self
            .key_map
            .iter()
            .position(|k| k.as_ref() == Some(&to))
            .unwrap();

        let to_pos = (to_ix % self.width, to_ix / self.width);

        let gap_ix = self
            .key_map
            .iter()
            .position(|k| k.as_ref() == None)
            .unwrap();
        let gap_row = gap_ix / self.width;

        let has_gap_on_row = gap_row == from_pos.1 || gap_row == to_pos.1;

        let is_two_spaces_away = (from_pos.0 as isize - to_pos.0 as isize).abs() == 2;

        if has_gap_on_row {
            if (to_pos.1 as isize) - (from_pos.1 as isize) < 0 {
                for _ in 0..((from_pos.1 as isize) - (to_pos.1 as isize)).abs() {
                    route.push('^');
                }
            } else {
                for _ in 0..((from_pos.1 as isize) - (to_pos.1 as isize)).abs() {
                    route.push('v');
                }
            }

            if (to_pos.0 as isize) - (from_pos.0 as isize) < 0 {
                for _ in 0..((from_pos.0 as isize) - (to_pos.0 as isize)).abs() {
                    route.push('<');
                }
            } else {
                for _ in 0..((from_pos.0 as isize) - (to_pos.0 as isize)).abs() {
                    route.push('>');
                }
            }
        } else {
            if (to_pos.0 as isize) - (from_pos.0 as isize) < 0 {
                for _ in 0..((from_pos.0 as isize) - (to_pos.0 as isize)).abs() {
                    route.push('<');
                }
            } else {
                for _ in 0..((from_pos.0 as isize) - (to_pos.0 as isize)).abs() {
                    route.push('>');
                }
            }

            if (to_pos.1 as isize) - (from_pos.1 as isize) < 0 {
                for _ in 0..((from_pos.1 as isize) - (to_pos.1 as isize)).abs() {
                    route.push('^');
                }
            } else {
                for _ in 0..((from_pos.1 as isize) - (to_pos.1 as isize)).abs() {
                    route.push('v');
                }
            }
        }

        route.push('A');

        route
    }

    fn full_route_to_code(&self, start: char, code: &[char]) -> Vec<char> {
        let mut current_key = start;

        code.iter()
            .flat_map(|c| {
                let seq = self.route(current_key, *c);
                current_key = *c;
                seq
            })
            .collect()
    }

    fn reverse_sequence(&self, start: char, sequence: &[char]) -> Vec<char> {
        let mut current_key = Some(start);
        let mut output = vec![];

        sequence.iter().for_each(|s| {
            let ix: usize = self.key_map.iter().position(|k| *k == current_key).unwrap();
            let mut pos = (ix % self.width, ix / self.width);

            match s {
                '^' => pos.1 -= 1,
                'v' => pos.1 += 1,
                '<' => pos.0 -= 1,
                '>' => pos.0 += 1,
                _ => (),
            }

            current_key = *self.key_map.get(pos.1 * self.width + pos.0).unwrap();

            // println!("Key {:?}", current_key);

            if *s == 'A' {
                output.push(current_key.unwrap());
            }
        });

        output
    }
}

fn calculate_final_sequence(code: &[char]) -> Vec<char> {
    let keypad = Keypad::new(
        vec![
            Some('7'),
            Some('8'),
            Some('9'),
            Some('4'),
            Some('5'),
            Some('6'),
            Some('1'),
            Some('2'),
            Some('3'),
            None,
            Some('0'),
            Some('A'),
        ],
        3,
        4,
    );

    let seq = keypad.full_route_to_code('A', &code);

    let seq_string: String = seq.iter().collect();
    println!("Seq {}", seq_string);

    let direction_pad = Keypad::new(
        vec![None, Some('^'), Some('A'), Some('<'), Some('v'), Some('>')],
        3,
        2,
    );

    let seq2 = direction_pad.full_route_to_code('A', &seq);

    let seq_string: String = seq2.iter().collect();
    println!("Seq {}", seq_string);

    let seq3 = direction_pad.full_route_to_code('A', &seq2);

    let seq_string: String = seq3.iter().collect();
    println!("Seq {}", seq_string);

    seq3
}

fn calculate_complexity(codes: &[String]) -> usize {
    let sequences = codes
        .iter()
        .map(|c| calculate_final_sequence(&c.chars().collect::<Vec<_>>()));

    sequences
        .zip(codes)
        .map(|(seq, c)| {
            let num = c[..c.len() - 1].parse::<usize>().unwrap_or(0);

            // println!("Num {} Seq {}", num, seq.len());
            num * seq.len()
        })
        .sum()
}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let codes: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    let result = calculate_complexity(&codes);
    println!("Sum of complexities is {}", result);

    let direction_pad = Keypad::new(
        vec![None, Some('^'), Some('A'), Some('<'), Some('v'), Some('>')],
        3,
        2,
    );
    let seq = calculate_final_sequence(&"1".chars().collect::<Vec<_>>());
    // let rev1 = direction_pad.reverse_sequence(
    //     'A',
    //     &"<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A"
    //         .chars()
    //         .collect::<Vec<_>>(),
    // );

    let seq_string: String = seq.iter().collect();
    println!("Rev {}", seq_string);

    // let rev2 = direction_pad.reverse_sequence('A', &rev1);

    // let seq_string: String = rev2.iter().collect();
    // println!("Rev2 {}", seq_string);

    Ok(())
}

// < 235218
// < 226566
// < 233106
