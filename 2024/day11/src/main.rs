use std::collections::HashMap;
use std::env;
use std::fs;


type Stone = u64;
type Sequence = HashMap<Stone, usize>;


fn parse_file(path: &str) -> Sequence {
    let mut seq = HashMap::new();

    for value in fs::read_to_string(path).expect("could not read file").split(" ") {
        let v: u64 = value.parse().expect("could not parse value");
        *seq.entry(v).or_insert(0) += 1;
    }

    seq
}

fn blink(seq: &Sequence) -> Sequence {
    let mut new_seq = HashMap::new();

    for (stone, count) in seq {
        let str_stone = stone.to_string();

        if *stone == 0 {
            *new_seq.entry(1).or_insert(0) += *count;
        } else if str_stone.len() % 2 == 0 {
            let mid = str_stone.len() / 2;
            let left: u64 = str_stone[0..mid].parse().expect("oh no");
            let right: u64 = str_stone[mid..].parse().expect("oh no");

            *new_seq.entry(left).or_insert(0) += *count;
            *new_seq.entry(right).or_insert(0) += *count;
        } else {
            let new_stone = *stone * 2024;

            *new_seq.entry(new_stone).or_insert(0) += *count;
        }
    }

    new_seq
}

fn blink_n(seq: &Sequence, n: usize) -> Sequence {
    let mut new_seq = seq.clone();

    for _ in 0..n {
        new_seq = blink(&new_seq);
    }

    new_seq
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = args.get(1).expect("must specify input file");

    let mut seq = parse_file(input_filename);
    println!("Part 1 Total Stones: {}", blink_n(&mut seq, 25).values().sum::<usize>());
    println!("Part 2 Total Stones: {}", blink_n(&mut seq, 75).values().sum::<usize>());
}
