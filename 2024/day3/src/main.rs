use std::fs;
use std::env;
use std::io::Read;
use regex::Regex;


fn main() {
    let args: Vec<String> = env::args().collect();

    let input_filename = args.get(1).expect("must specify input file");
    let mut file = fs::File::open(input_filename).expect("could not open file");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("could not read file");

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut total: i32 = 0;
    for (_, [first, second]) in re.captures_iter(&data).map(|c| c.extract()) {
        total += first.parse::<i32>().unwrap() * second.parse::<i32>().unwrap();
    }
    println!("Part 1 Total: {}", total);

    let re = Regex::new(r"(?<op>do|don't|mul)\(((?<arg1>\d{1,3}),(?<arg2>\d{1,3}))?\)").unwrap();
    let mut ignore = false;
    let mut total: i32 = 0;
    for cap in re.captures_iter(&data) {
        match cap.name("op").unwrap().as_str() {
            "do" => { ignore = false }
            "don't" => { ignore = true }
            "mul" => {
                if !ignore {
                    total += cap.name("arg1").unwrap().as_str().parse::<i32>().unwrap() * cap.name("arg2").unwrap().as_str().parse::<i32>().unwrap()
                }
            }
            _ => {}
        }
    }
    println!("Part 2 Total: {}", total);
}
