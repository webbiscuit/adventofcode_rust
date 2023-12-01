use std::io::{self, prelude::*};

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    println!("Hello, world!");

    Ok(())
}
