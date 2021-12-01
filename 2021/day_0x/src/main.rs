use std::error::Error;
use std::io::{self, prelude::*};

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let parsed: Result<Vec<_>, Box<dyn Error>> = lines.map(|line| -> Result<_, Box<dyn Error>> { Ok(line?.parse::<u32>()?) } ).collect();

    match parsed {
        Ok(p) => {
            for n in p {
                println!("{}", n);
            }
        },
        Err(e) => {
            eprintln!("Error parsing file: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
