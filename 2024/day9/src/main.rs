use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::usize;


type Disk = Vec<i32>;

#[derive(Debug)]
struct Section {
    location: usize,
    size: usize,
    id: i32,
}

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

fn index_disk(disk: &Disk) -> (Vec<Section>, Vec<Section>) {
    let mut filled: Vec<Section> = Vec::new();
    let mut empty: Vec<Section> = Vec::new();

    let mut last_start: usize = usize::MAX;
    let mut last_value: i32 = -999;
    for (idx, value) in disk.iter().enumerate() {
        if *value != last_value {
            if last_start != usize::MAX {
                let section = Section {
                    location: last_start,
                    size: idx - last_start,
                    id: last_value,
                };

                if last_value == EMPTY {
                    empty.push(section);
                } else {
                    filled.push(section);
                }
            }
            last_start = idx;
            last_value = *value;
        }
    }

    let section = Section {
        location: last_start,
        size: disk.len() - last_start,
        id: last_value,
    };
    if last_value == EMPTY {
        empty.push(section);
    } else {
        filled.push(section);
    }

    (filled, empty)
}

fn defrag_full(disk: &Disk) -> Disk {
    let mut defragged: Disk = disk.clone();
    let (filled, mut empty) = index_disk(disk);

    for fblock in filled.iter().rev() {
        for (idx, eblock) in empty.iter().enumerate() {
            if fblock.size > eblock.size { continue; }
            if fblock.location < eblock.location { break; }

            let size_diff = eblock.size - fblock.size;

            if size_diff == 0 {
                for x in eblock.location..(eblock.location + eblock.size) {
                    defragged[x] = fblock.id;
                }
                for x in fblock.location..(fblock.location + fblock.size) {
                    defragged[x] = EMPTY;
                }

                empty.remove(idx);
                break;
            } else if size_diff > 0 {
                for x in eblock.location..(eblock.location + fblock.size) {
                    defragged[x] = fblock.id;
                }
                for x in fblock.location..(fblock.location + fblock.size) {
                    defragged[x] = EMPTY;
                }

                empty.insert(idx, Section {
                    location: eblock.location + fblock.size,
                    size: eblock.size - fblock.size,
                    id: eblock.id,
                });
                empty.remove(idx + 1);
                break;
            }
        }
    }

    defragged
}

fn print_disk(disk: &Disk) {
    let strs: Vec<String> = disk.iter().map(|&v| if v >= 0 { v.to_string() } else { String::from(".") }).collect();
    println!("{}", strs.join(""));
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

    let defragged = defrag_full(&disk);
    println!("Part 2 Checksum: {}", checksum(&defragged));
}
