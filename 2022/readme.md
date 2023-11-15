# Advent of Code in Rust 2022

## Prerequisites

- Cargo
- Rust
- Christmas 2022

## Building

`cargo build`

## Running a day

    cd <to correct directory in src>

Then:

    cargo run < input.txt

or

    echo -n "yourinput" | cargo run

or 

    cargo run -- bin day_01
    <hey>

## Running a test

The tests are from the example of the day

    cargo test

### Needs more attention

Day 15 - Part 2 is really slow, need to optimise finding the first gap in the range 