use std::io::{self, prelude::*};


#[derive(Debug)]
enum SafeCommands {
    Left(u32),
    Right(u32)
}

fn parse_input(lines: &[String]) -> Vec<SafeCommands> {
    let mut commands: Vec<SafeCommands> = Vec::new();

    for line in lines {
        let (dir, amount) = line.split_at(1);
        let amount: u32 = amount.parse().expect("Not a number");

        if dir == "L" {
            commands.push(SafeCommands::Left(amount));
        }
        else if dir == "R" {
            commands.push(SafeCommands::Right(amount));
        }
    }

    commands
}

fn open_safe(commands: &[SafeCommands]) -> u32 {
    let mut safe_number: i32 = 50;
    let mut zero_counts = 0;

    for command in commands {
        match command {
            SafeCommands::Left (amount) => safe_number -= *amount as i32,
            SafeCommands::Right (amount) => safe_number += *amount as i32,
        }

        safe_number = safe_number.rem_euclid(100);

        if safe_number == 0 {
            zero_counts += 1;
        }

        // println!("Number is {}", safe_number);
    }

    zero_counts
}

fn open_safe_clicky(commands: &[SafeCommands]) -> u32 {
    let mut safe_number: i32 = 50;
    let mut zero_counts: u32 = 0;

    for command in commands {
        // print!("Number is {}", safe_number);

        match command {
            SafeCommands::Left (amount) => {
                for _ in 0..*amount {
                    safe_number -= 1;
                    safe_number = safe_number.rem_euclid(100);
                    if safe_number == 0 {
                        zero_counts += 1;
                    } 
                }
            }
            SafeCommands::Right (amount) => {
                for _ in 0..*amount {
                    safe_number += 1;
                    safe_number = safe_number.rem_euclid(100);
                    if safe_number == 0 {
                        zero_counts += 1;
                    } 
                }
            }
        }
    }

    zero_counts
}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    let commands = parse_input(&lines);
    let password = open_safe(&commands);

    println!("The password is {}", password);

    let password = open_safe_clicky(&commands);

    println!("The clicky password is {}", password);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_2_clicks() {
        let count = open_safe_clicky(&[
            SafeCommands::Left(51),
            SafeCommands::Right(2),
        ]);

        assert_eq!(count, 2);
    }

    #[test]
    fn test_left_to_zero() {
        let count = open_safe_clicky(&[
            SafeCommands::Left(50),
        ]);

        assert_eq!(count, 1);
    }

    #[test]
    fn test_right_to_zero() {
        let count = open_safe_clicky(&[
            SafeCommands::Right(50),
        ]);

        assert_eq!(count, 1);
    }

    #[test]
    fn test_left_to_zero_then_turn() {
        let count = open_safe_clicky(&[
            SafeCommands::Left(50),
            SafeCommands::Left(1),
        ]);

        assert_eq!(count, 1);
    }

     #[test]
    fn test_right_to_zero_then_turn() {
        let count = open_safe_clicky(&[
            SafeCommands::Right(50),
            SafeCommands::Right(1),
        ]);

        assert_eq!(count, 1);
    }

    // #[test]
    // fn test_count_on_zero() {
    //     let count = open_safe_clicky(&[
    //         SafeCommands::Left(50),
    //         SafeCommands::Right(100),
    //     ]);

    //     assert_eq!(count, 2);
    // }
}