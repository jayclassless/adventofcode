use std::env;
use std::fs;
use std::io;
use std::io::BufRead;


#[derive(Debug)]
struct Calibration {
    target: i64,
    values: Vec<i64>,
}

fn parse_file(path: &str) -> Vec<Calibration> {
    let file = fs::File::open(path).expect("could not open file");
    let reader = io::BufReader::new(file);

    let mut calibrations = Vec::new();

    for line in reader.lines() {
        let line = line.expect("could not read line");
        let mut parts = line.split(':');

        let target: i64 = parts.next().unwrap().parse().expect("could not parse value");

        let values: Vec<i64> = parts.next().unwrap()[1..].split(' ').map(|v| v.parse().expect("could not parse reading")).collect();

        calibrations.push(Calibration { target, values });
    }

    calibrations
}

fn part1_can_produce_target(target: i64, start: i64, values:&[i64]) -> bool {
    if values.len() == 0 {
        return target == start;
    }

    if start > target {
        return false;
    }

    let (next, rest) = values.split_first().unwrap();
    part1_can_produce_target(target, start + next, rest) || part1_can_produce_target(target, start * next, rest)
}

fn part2_can_produce_target(target: i64, start: i64, values:&[i64]) -> bool {
    if values.len() == 0 {
        return target == start;
    }

    if start > target {
        return false;
    }

    let (next, rest) = values.split_first().unwrap();
    let combined: i64 = format!("{}{}", start, next).parse().expect("uh oh");
    part2_can_produce_target(target, start + next, rest) || part2_can_produce_target(target, start * next, rest) || part2_can_produce_target(target, combined, rest)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = args.get(1).expect("must specify input file");

    let calibrations = parse_file(input_filename);

    let mut total: i64 = 0;
    for calibration in &calibrations {
        let (start, values) = calibration.values.split_first().unwrap();
        if part1_can_produce_target(calibration.target, *start, values) {
            total += calibration.target;
        }
    }
    println!("Part 1 Total: {}", total);
    
    let mut total: i64 = 0;
    for calibration in &calibrations {
        let (start, values) = calibration.values.split_first().unwrap();
        if part2_can_produce_target(calibration.target, *start, values) {
            total += calibration.target;
        }
    }
    println!("Part 2 Total: {}", total);
}
