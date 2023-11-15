use std::{
    collections::HashMap,
    io::{self, BufRead},
};

struct RockShape {}

struct Chamber {
    width: u8,
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    println!("After 2022 rocks have fallen the tower is {} units high.")
}
