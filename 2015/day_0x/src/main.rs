use std::error::Error;
// use std::io::{self, prelude::*};

fn main() -> Result<(), Box<dyn Error>> {
    // let stdin = io::stdin();
    // let lines = stdin.lock().lines();

    // let parsed: Result<Vec<u32>, Box<dyn Error>> = lines.map(|line| Ok(line?.parse::<u32>()?)).collect();

    // match parsed {
    //     Ok(p) => {
    //         for n in p {
    //             println!("{}", n);
    //         }
    //     },
    //     Err(e) => {
    //         eprintln!("Error parsing file: {}", e);
    //         return Err(e);
    //     }
    // }

    println!("Hello, world!");

    Ok(())
}
