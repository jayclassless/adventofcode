use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::ops::Mul;


type Disk = Vec<i32>;

const EMPTY: i32 = -1;


fn parse_file(path: &str) -> Disk {
    let file = fs::File::open(path).expect("could not open file");
    let reader = io::BufReader::new(file);

    let mut disk = Vec::new();

    let mut seq = 0;
    for line in reader.lines() {
        for (idx, c) in line.expect("could not read line").chars().enumerate() {
            for _ in 0..c.to_digit(10).unwrap() {
                if idx % 2 == 0 {
                    disk.push(seq);
                } else {
                    disk.push(EMPTY);
                }
            }

            if idx % 2 == 0 {
                seq += 1;
            }
        }
    }

    disk
}

fn defrag(disk: &Disk) -> Disk {
    let mut defragged: Disk = disk.clone();

    let mut first_empty = disk.iter().position(|&b| b == EMPTY).unwrap();
    let mut last_block = disk.len() - 1;

    loop {
        if defragged[first_empty] != EMPTY {
            first_empty += 1;
            continue;
        }

        if first_empty > last_block {
            break;
        }

        if defragged[last_block] != EMPTY {
            defragged[first_empty] = defragged[last_block];
            defragged[last_block] = EMPTY;

            first_empty += 1;
            last_block -= 1;
        } else {
            last_block -= 1;
        }
    }

    defragged
}

fn checksum(disk: &Disk) -> i64 {
    let mut sum: i64 = 0;

    for (idx, id) in disk.iter().enumerate() {
        if *id != EMPTY {
            sum += (*id as i64) * (idx as i64);
        }
    }

    sum
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = args.get(1).expect("must specify input file");

    let disk = parse_file(input_filename);
    let defragged = defrag(&disk);
    println!("Part 1 Checksum: {}", checksum(&defragged));
}
