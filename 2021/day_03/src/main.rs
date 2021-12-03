use std::error::Error;
use std::io::{self, prelude::*};

fn calculate_power_consumption(report_items: &[u16]) -> u16 {
    let mut result = 0u16;

    for n in 0..12 {
        let mut hi_bit_count = 0;

        for i in report_items {
            if i & 1 << n == 1 << n {
                hi_bit_count += 1;
            }

            if hi_bit_count > report_items.len() / 2 {
                result |= 1 << n;
            }
        }
    }

    let mask = 0b111111111111; 

    println!("x: {:0>12b}", result);
    println!("x: {:0>12b}", result ^ mask);

    result * (result ^ mask)
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let parsed: Result<Vec<u16>, Box<dyn Error>> =
        lines.map(|line| u16::from_str_radix(&line.unwrap(), 2)).collect();

    // let parsed: Result<Vec<u16>, Box<dyn Error>> = lines
    //     .map(|line| u16::from_str(&line.unwrap()))
    //     .collect();

    // let bin_idx = "01110011001";
    // let intval = isize::from_str_radix(bin_idx, 2).unwrap();
    // println!("{}", intval);

    // let parsed: Result<Vec<u16>, Box<dyn Error>> = lines.map(|line| Ok(line?.parse::<u16>()?)).collect();

    match parsed {
        Ok(p) => {
            let power = calculate_power_consumption(&vec![0b000000000100, 0b000000000110, 0b000000001110]);

            println!("Power consumption of the submarine: {}", power);
            
        }
        Err(e) => {
            eprintln!("Error parsing file: {}", e);
            return Err(e);
        }
    }

    Ok(())
}

#[test]
fn test_calculate_power_consumption() {
    let count = calculate_power_consumption(&vec![0b000000000100, 0b000000000110, 0b000000001110]);

    assert_eq!(count, 0b000000000110 * 0b111111111001);
}
