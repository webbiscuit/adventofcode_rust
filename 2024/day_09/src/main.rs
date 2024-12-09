use std::{
    io::{self, prelude::*},
    iter::Zip,
};

type FileId = usize;
type Disk = Vec<Option<FileId>>;

fn parse(lines: &[String]) -> Disk {
    let line = &lines[0];
    let disk = line
        .char_indices()
        .flat_map(|(ix, c)| {
            let count = c.to_digit(10).expect("Not a digit");
            // println!("Count {}", count);

            if ix % 2 == 0 {
                let id = ix / 2;

                (0..count).map(|_| Some(id)).collect::<Vec<_>>()
            } else {
                (0..count).map(|_| None).collect::<Vec<_>>()
            }
        })
        .collect();

    disk
}

fn compact(mut disk: Disk) -> Disk {
    let mut i = 0;
    let mut j = disk.len() - 1;

    while j > 0 {
        let data_to_move = disk[j];

        if data_to_move.is_some() {
            let first_space = disk[i..]
                .iter_mut()
                .enumerate()
                .find(|(_, c)| c.is_none())
                .expect("No disk space left");

            // We've gone through it all
            if (first_space.0 + i) > j {
                break;
            }

            *first_space.1 = data_to_move;
            i = first_space.0 + i;
            disk[j] = None;
        }

        j -= 1;
    }

    disk
}

fn checksum(disk: &Disk) -> usize {
    disk.iter()
        .enumerate()
        .map(|(ix, i)| ix * i.unwrap_or(0))
        .sum()
}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    let disk = parse(&lines);

    // println!("Disk {:?}", disk);

    let compacted = compact(disk);
    let result = checksum(&compacted);

    // println!("Disk {:?}", compacted);

    println!("The filesystem checksum is {}", result);

    Ok(())
}
