use std::io::BufRead;
use std::fs;
use std::io;
use std::env;


type Level = i16;
type Report = Vec<Level>;

fn parse_line(line: &str) -> Report {
    line.split_ascii_whitespace().map(|v| v.parse().expect("could not parse level")).collect()
}

fn parse_file(path: &str) -> Vec<Report> {
    let file = fs::File::open(path).expect(&format!("{} does not exist", path));
    let reader = io::BufReader::new(file);

    reader.lines().map(|l| parse_line(&l.expect("could not read line"))).collect()
}

fn is_report_safe(report: &Report) -> bool {
    let mut last_direction: Option<bool> = None;

    for i in 0..(report.len() - 1) {
        let diff = report[i] - report[i + 1];
        let current_direction = if diff > 0 { true } else { false };

        let diff = diff.abs();
        if diff < 1 || diff > 3 {
            return false
        }

        if let Some(dir) = last_direction {
            if dir != current_direction {
                return false
            }
        }

        last_direction = Some(current_direction);
    }

    true
}

fn is_report_safe_with_dampener(report: &Report) -> bool {
    if is_report_safe(report) {
        return true
    }

    for i in 0..report.len() {
        let mut dampened = report.clone();
        dampened.remove(i);

        if is_report_safe(&dampened) {
            return true
        }
    }

    false
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_filename = args.get(1).expect("must specify input file");
    let reports = parse_file(input_filename);

    let num_safe = reports.iter().filter(|&r| is_report_safe(r)).count();
    println!("Part 1 Safe Reports: {}", num_safe);

    let num_safe = reports.iter().filter(|&r| is_report_safe_with_dampener(r)).count();
    println!("Part 2 Safe Reports: {}", num_safe);
}
