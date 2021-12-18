use std::error::Error;
use std::fmt;
use std::io::{self, prelude::*};

use itertools::Itertools;

#[derive(Debug, PartialEq)]
struct Packet {
    version: u8,
    type_id: u8,
    payload: Payload
}

#[derive(Debug, PartialEq)]
enum Payload {
    Literal(u64),
    Operator(Vec<Packet>),
    None
}

// impl fmt::Display for Payload {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Payload::Literal(v) => write!(f, "{}", v),
//             Payload::Operator(v) => write!(f, "{:?}", v),
//             _ => write!(f, "None"),
//         };
        
//         Ok(())
//     }
// }

struct BitsSystem {
    message: Vec<bool>,
    packet: Packet,
}

impl BitsSystem {
    fn new(hex: &str) -> Self {
        let mut bits = hex.chars().flat_map(
            |c| {
                let i = i8::from_str_radix(&c.to_string(), 16).unwrap();
                (0..4).map(move |b| i & (8 >> b) != 0)
            }).collect::<Vec<_>>();

        Self { 
            message: bits.clone(), 
            packet: BitsSystem::parse_packet(&mut bits) } 
    }

    fn parse_packet(bits: &mut Vec<bool>) -> Packet {
        let version = BitsSystem::to_value(&bits.drain(0..3).collect_vec()) as u8;
        let type_id = BitsSystem::to_value(&bits.drain(0..3).collect_vec()) as u8;

        let mut payload: Payload = Payload::None;

        match type_id {
            4 => payload = Payload::Literal(BitsSystem::parse_literal(bits)),
            _ => payload = Payload::Operator(BitsSystem::parse_operator(bits)),
        }

        Packet { version, type_id, payload }
    }

    fn parse_operator(bits: &mut Vec<bool>) -> Vec<Packet> {
        let mut packets: Vec<Packet> = Vec::new();

        let length_type = if bits.drain(0..1).next().unwrap() == true { 1  } else { 0 };

        if length_type == 0 {
            let mut total_length_in_bits = BitsSystem::to_value(&bits.drain(0..15).collect_vec()) as i32;
            
            let total_bits = bits.len();

            while total_length_in_bits > 0 {
                let packet = BitsSystem::parse_packet(bits);
                total_length_in_bits -= total_bits as i32 - bits.len() as i32; 
                packets.push(packet);
            }
        }
        else if length_type == 1 {
            let mut number_of_sub_packets = BitsSystem::to_value(&bits.drain(0..11).collect_vec()) as i32;

            while number_of_sub_packets > 0 {
                let packet = BitsSystem::parse_packet(bits);
                number_of_sub_packets -= 1;
                packets.push(packet);
            }
        }

        packets
    }

    fn parse_literal(bits: &mut Vec<bool>) -> u64 {
        let mut value: u64 = 0;

        loop {
            let end = bits.drain(0..1).next().unwrap() == false;
            let segment = BitsSystem::to_value(&bits.drain(0..4).collect_vec()) as u64;

            value = value | segment;

            if end {
                break;
            }

            value <<= 4;
        }

        value
    }

    fn to_value(bits: &[bool]) -> u32 {
        bits.iter().fold(0, |acc, &b| acc * 2 + b as u32)
    }

    fn get_packet(&self) -> &Packet {
        &self.packet
    }

    fn sum_version_numbers(&self) -> u32 {
        BitsSystem::sum_version_for_packet(&self.packet)
    }

    fn sum_version_for_packet(packet: &Packet) -> u32 {
        match packet.payload {
            Payload::Literal(v) => packet.version as u32,
            // Payload::Operator(ref packets) => packet.version as u32 + packets.iter().map(|p| BitsSystem::sum_version_for_packet(&p)).sum::<u32>(),
            Payload::Operator(ref packets) => packet.version as u32 + packets.iter().fold(0,|acc, p| acc + BitsSystem::sum_version_for_packet(&p)),
            _ => 0
        }
    }

}

impl fmt::Display for BitsSystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for &bit in self.message.iter() {
            write!(f, "{}", if bit { 1 } else { 0 })?;
        }

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let hex = lines
        .next().unwrap()?;

    // println!("{}", hex);

    let bits_system = BitsSystem::new(&hex);

    println!("Sum of version numbers in all packets: {}", bits_system.sum_version_numbers());

    Ok(())
}


#[test]
fn check_super_simple_example() {
    let bits = "D2FE28";
    let bits_system = BitsSystem::new(bits);

    assert_eq!(bits_system.get_packet().version, 6);
    assert_eq!(bits_system.get_packet().type_id, 4);
    assert_eq!(bits_system.get_packet().payload, Payload::Literal(2021));
}

#[test]
fn check_simple_example() {
    let bits = "38006F45291200";
    let bits_system = BitsSystem::new(bits);

    assert_eq!(bits_system.get_packet().version, 1);
    assert_eq!(bits_system.get_packet().type_id, 6);
    assert_eq!(bits_system.get_packet().payload, Payload::Operator(vec![
        Packet { version: 6, type_id: 4, payload: Payload::Literal(10) },
        Packet { version: 2, type_id: 4, payload: Payload::Literal(20) },
    ]));

}

#[test]
fn check_example1b() {
    let bits = "EE00D40C823060";
    let bits_system = BitsSystem::new(bits);

    assert_eq!(bits_system.get_packet().version, 7);
    assert_eq!(bits_system.get_packet().type_id, 3);
    assert_eq!(bits_system.get_packet().payload, Payload::Operator(vec![
        Packet { version: 2, type_id: 4, payload: Payload::Literal(1) },
        Packet { version: 4, type_id: 4, payload: Payload::Literal(2) },
        Packet { version: 1, type_id: 4, payload: Payload::Literal(3) },
    ]));
}

#[test]
fn check_example2() {
    let bits = "8A004A801A8002F478";
    let bits_system = BitsSystem::new(bits);

    assert_eq!(bits_system.get_packet().version, 4);
    assert_eq!(bits_system.get_packet().type_id, 2);
    assert_eq!(bits_system.sum_version_numbers(), 16);
    // assert_eq!(bits_system.get_packet().payload, Payload::Operator(vec![
    //     Packet { version: 4, type_id: 4, payload: Payload::Operator(vec![
    //         Packet { version: 1, type_id: 4, payload: Payload::Operator(vec![
    //         Packet { version: 5, type_id: 4, payload: Payload::Literal(6) },
    //         ])},
    //     ]) },
    // ]));

}

#[test]
fn check_example3() {
    let bits = "620080001611562C8802118E34";
    let bits_system = BitsSystem::new(bits);

    assert_eq!(bits_system.get_packet().version, 3);
    assert_eq!(bits_system.get_packet().type_id, 0);
    assert_eq!(bits_system.sum_version_numbers(), 12);
    
    // assert_eq!(bits_system.get_packet().payload, Payload::Operator(vec![
    //     Packet { version: 2, type_id: 4, payload: Payload::Literal(1) },
    //     Packet { version: 1, type_id: 4, payload: Payload::Literal(3) },
    // ]));

}

#[test]
fn check_example4() {
    let bits = "C0015000016115A2E0802F182340";
    let bits_system = BitsSystem::new(bits);

    assert_eq!(bits_system.get_packet().version, 6);
    assert_eq!(bits_system.get_packet().type_id, 0);
    assert_eq!(bits_system.sum_version_numbers(), 23);
}

#[test]
fn check_example5() {
    let bits = "A0016C880162017C3686B18A3D4780";
    let bits_system = BitsSystem::new(bits);

    assert_eq!(bits_system.get_packet().version, 5);
    assert_eq!(bits_system.get_packet().type_id, 0);
    assert_eq!(bits_system.sum_version_numbers(), 31);
}