use std::env;
use std::fs;
use std::io;
use std::io::BufRead;


type Value = i32;
type Line = (Value, Value);

fn parse_line(line: &str) -> Line {
    let parts: Vec<&str> = line.split_ascii_whitespace().collect();
    if parts.len() != 2 {
        panic!("too many values on a line");
    }

    (
        parts[0].parse().expect("value not an int"),
        parts[1].parse().expect("value not an int"),
    )
}

fn parse_file(path: &str) -> Vec<Line> {
    let file = fs::File::open(path).expect(&format!("{} does not exist", path));
    let reader = io::BufReader::new(file);

    reader.lines().map(|l| parse_line(&l.expect("could not read line"))).collect()
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let input_filename = args.get(1).expect("must specify input file");

    let mut group1: Vec<Value> = Vec::new();
    let mut group2: Vec<Value> = Vec::new();
    for line in parse_file(input_filename) {
        group1.push(line.0);
        group2.push(line.1);
    }
    group1.sort();
    group2.sort();

    let mut distances: Vec<Value> = Vec::with_capacity(group1.len());
    for i in 0..group1.len() {
        distances.push((group1[i] - group2[i]).abs());
    }
    println!("Total Distance: {}", distances.iter().sum::<Value>());

    let mut similarity: i32 = 0;
    for value in group1 {
        similarity += group2.iter().filter(|&v| *v == value).count() as Value * value;
    }
    println!("Similarity: {}", similarity)
}
