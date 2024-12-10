use std::io::{self, prelude::*};

type FileId = usize;
type Disk = Vec<Option<FileId>>;

fn parse(lines: &[String]) -> Disk {
    let line = &lines[0];
    let mut total_ix = 0;

    let disk = line
        .char_indices()
        .flat_map(|(ix, c)| {
            let count = c.to_digit(10).expect("Not a digit");

            total_ix += count as usize;

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
            i += first_space.0;
            disk[j] = None;
        }

        j -= 1;
    }

    disk
}

fn compact_by_file(mut disk: Disk) -> Disk {
    // let mut i = 0;
    let mut file_id_to_move = disk.last().unwrap().unwrap();

    while file_id_to_move > 0 {
        let start_ix = disk
            .iter()
            .position(|&f| f == Some(file_id_to_move))
            .expect("File ID not found");

        let end_ix = disk
            .iter()
            .rposition(|&f| f == Some(file_id_to_move))
            .expect("File ID not found");

        let chars_to_move = end_ix - start_ix + 1;

        // println!(
        //     "ID {}, Moving {}, ix {}",
        //     file_id_to_move, chars_to_move, start_ix
        // );

        let first_space = disk
            .windows(chars_to_move)
            .position(|window| window.iter().all(|&c| c.is_none()));

        if let Some(first_space) = first_space {
            // println!("first_space {}", first_space);

            if first_space > start_ix {
                file_id_to_move -= 1;
                continue;
            }

            for n in 0..chars_to_move {
                let source_ix = start_ix + n;

                // println!("SourceIX {}", source_ix);
                disk[first_space + n] = disk[source_ix];
                disk[source_ix] = None;
            }
        }

        file_id_to_move -= 1;
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

    let compacted = compact(disk.clone());
    let result = checksum(&compacted);

    // println!("Disk {:?}", compacted);

    println!("The filesystem checksum is {}", result);

    let compacted = compact_by_file(disk);
    let result = checksum(&compacted);

    // println!("Disk {:?}", compacted);

    println!(
        "When file compacting, the filesystem checksum is {}",
        result
    );

    Ok(())
}
